use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::{
    jamo::Jamo,
    syllable::{Syllable, SyllableError},
};

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, thiserror::Error)]
pub enum HangulError {
    #[error(transparent)]
    Syllable(#[from] SyllableError),
}
pub type HangulResult<T> = Result<T, HangulError>;
