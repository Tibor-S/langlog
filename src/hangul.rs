use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::{
    jamo::Jamo,
    syllable::{Syllable, SyllableError},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hangul(Vec<Syllable>);
impl Hangul {
    pub fn push_back(&mut self, jamo: Jamo) -> HangulResult<()> {
        let syl = match self.last_mut() {
            Some(s) => s,
            None => {
                self.push(Syllable::default());
                &mut self[0]
            }
        };

        let overflow = syl.push(jamo)?;
        match overflow {
            Some(new_syl) => self.push(new_syl),
            None => (),
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub fn pop_back(&mut self) -> Option<Jamo> {
        match self.last_mut() {
            Some(syl) if syl.is_empty() => {
                self.0.pop();
                self.pop_back()
            }
            Some(syl) => {
                let ret = syl.pop();
                self.0.pop_if(|s| s.is_empty());
                ret
            }
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn break_with(&mut self, jamo: Jamo) -> HangulResult<()> {
        self.push(jamo.try_into()?);
        Ok(())
    }
}
impl Deref for Hangul {
    type Target = Vec<Syllable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Hangul {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Display for Hangul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for syl in self.iter() {
            write!(f, "{}", syl)?;
        }
        Ok(())
    }
}
impl FromIterator<Syllable> for Hangul {
    fn from_iter<T: IntoIterator<Item = Syllable>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl From<Syllable> for Hangul {
    fn from(value: Syllable) -> Self {
        Self(vec![value])
    }
}
impl From<&Syllable> for Hangul {
    fn from(value: &Syllable) -> Self {
        Self(vec![value.clone()])
    }
}
impl TryFrom<Jamo> for Hangul {
    type Error = HangulError;

    fn try_from(value: Jamo) -> Result<Self, Self::Error> {
        Ok(Self(vec![Syllable::try_from(value)?]))
    }
}
impl TryFrom<&Jamo> for Hangul {
    type Error = HangulError;

    fn try_from(value: &Jamo) -> Result<Self, Self::Error> {
        Ok(Self(vec![Syllable::try_from(value)?]))
    }
}
impl TryFrom<Vec<Jamo>> for Hangul {
    type Error = HangulError;

    fn try_from(value: Vec<Jamo>) -> HangulResult<Self> {
        let mut hangul = Self::default();
        for j in value {
            hangul.push_back(j)?;
        }
        Ok(hangul)
    }
}
impl<'a> TryFrom<&'a str> for Hangul {
    type Error = HangulError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value
            .chars()
            .map(|c| Ok(Syllable::try_from(c)?))
            .collect::<HangulResult<Hangul>>()
    }
}
impl From<Hangul> for String {
    fn from(value: Hangul) -> Self {
        value.iter().map(char::from).collect()
    }
}
impl From<&Hangul> for String {
    fn from(value: &Hangul) -> Self {
        value.iter().map(char::from).collect()
    }
}
impl Serialize for Hangul {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let as_string = String::from(self);
        serializer.serialize_str(&as_string)
    }
}
impl<'de> Deserialize<'de> for Hangul {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let as_string = <&str>::deserialize(deserializer)?;
        match Hangul::try_from(as_string) {
            Ok(h) => Ok(h),
            Err(e) => {
                log::error!("Could not deserialize Hangul!");
                panic!("{}", e)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HangulError {
    #[error(transparent)]
    Syllable(#[from] SyllableError),
}
pub type HangulResult<T> = Result<T, HangulError>;
