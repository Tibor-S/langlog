use std::fmt::{Display, Write};

use crate::jamo::{FinalJamo, InitialJamo, Jamo, JamoError, MedialJamo};

#[derive(Debug, Clone, Copy)]
pub enum State {
    /// Next input must be initial or medial
    Start,
    /// Next input must be medial
    Medial,
    /// Syllable could end here, or next input must be medial or final
    Open,
    /// Syllable could end here or input must be final
    OpenFinal,
    /// Syllable must end here, cannot accept more inputs
    End,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Syllable {
    initial: Option<InitialJamo>,
    medial: Option<MedialJamo>,
    finale: Option<FinalJamo>,
}
impl Syllable {
    pub fn state(self) -> State {
        match (self.initial, self.medial, self.finale) {
            (None, _, _) => State::Start,
            (Some(_), None, _) => State::Medial,
            (Some(_), Some(m), None) if !m.combine_possible().is_empty() => {
                State::Open
            }
            (Some(_), Some(_), None) => State::OpenFinal,
            (Some(_), Some(_), Some(f)) if !f.append_possible().is_empty() => {
                State::OpenFinal
            }
            _ => State::End,
        }
    }

    pub fn possible(self) -> Vec<Jamo> {
        match self.state() {
            State::Start => Jamo::all_multi(true, true, false),
            State::Medial => Jamo::all_medial(),
            State::Open => Jamo::all_or_possible(
                false,
                (true, self.medial),
                (true, self.finale),
            ),
            State::OpenFinal => {
                Jamo::all_or_possible(false, (false, None), (true, self.finale))
            }
            State::End => vec![],
        }
    }

    /// .
    /// # Append
    /// Appends `jamo` to syllable if applicable.
    ///
    /// ## Error free usage:
    /// If `state` is `Start` (`Syllable::default()`):
    /// - Appending `jamo` which is `Initial` or `Medial`
    ///
    /// If `state` is `Medial` (Syllable consists of an `Initial`):
    /// - Appending `jamo` which is `Medial`
    ///
    /// If `state` is `Open` (Syllable consists of an `Initial` and a **single** `Medial`):
    /// - Appending any `jamo` is fine, overflow will be returned
    ///
    /// If `state` is `OpenFinal` and syllable does not consist of any `Final` jamo:
    /// - Appending any `jamo` is fine, overflow will be returned
    ///
    /// If `state` is `OpenFinal` but syllable consists of a **single** `Final` jamo:
    /// - Appending any `Initial` or `Medial` will be returned in overflow
    /// - Appending `Final` jamo which is compatible with current `Final`
    ///
    /// No matter the `state`:
    /// - Appending `Jamo` returned by `.possible()`, will not result
    /// in any errors or overflow.
    ///
    pub fn append(&mut self, jamo: Jamo) -> SyllableResult<Option<Syllable>> {
        match self.state() {
            State::Start => {
                if let Ok(ij) = InitialJamo::try_from(jamo) {
                    self.initial = Some(ij);
                    Ok(None)
                } else if let Ok(mj) = MedialJamo::try_from(jamo) {
                    self.initial = Some(InitialJamo::Silent);
                    self.medial = Some(mj);
                    Ok(None)
                } else {
                    Err(SyllableError::ExpectedInitialOrMedial(jamo))
                }
            }
            State::Medial => {
                if let Ok(mj) = MedialJamo::try_from(jamo) {
                    Ok(self.set_or_combine(mj))
                } else {
                    Err(SyllableError::ExpectedMedial(jamo))
                }
            }
            State::Open => {
                if let Ok(mj) = MedialJamo::try_from(jamo) {
                    Ok(self.set_or_combine(mj))
                } else if let Ok(fj) = FinalJamo::try_from(jamo) {
                    self.finale = Some(fj);
                    Ok(None)
                } else if let Ok(ij) = InitialJamo::try_from(jamo) {
                    Ok(Some(ij.into()))
                } else {
                    panic!(
                        "Critical logic error! Jamo could not be translated to `Initial`, `Medial` or `Final`"
                    );
                }
            }
            State::OpenFinal => {
                if let Ok(fj) = FinalJamo::try_from(jamo) {
                    self.set_or_append(fj)
                } else if let Ok(ij) = InitialJamo::try_from(jamo) {
                    Ok(Some(ij.into()))
                } else if let Ok(mj) = MedialJamo::try_from(jamo) {
                    Ok(Some(mj.into()))
                } else {
                    panic!(
                        "Critical logic error! Jamo could not be translated to `Initial`, `Medial` or `Final`"
                    );
                }
            }
            State::End => {
                if let Ok(ij) = InitialJamo::try_from(jamo) {
                    Ok(Some(ij.into()))
                } else if let Ok(mj) = MedialJamo::try_from(jamo) {
                    Ok(Some(mj.into()))
                } else {
                    Err(SyllableError::ExpectedInitialOrMedial(jamo))
                }
            }
        }
    }

    fn set_or_combine(&mut self, medial: MedialJamo) -> Option<Syllable> {
        match self.medial {
            Some(mj) => match mj.combine(medial) {
                Ok(nmj) => {
                    self.medial = Some(nmj);
                    None
                }
                Err(_) => Some(medial.into()),
            },
            None => {
                self.medial = Some(medial);
                None
            }
        }
    }

    fn set_or_append(
        &mut self,
        finale: FinalJamo,
    ) -> SyllableResult<Option<Syllable>> {
        // Redundant option in return type to make impl in `append` seamless
        // A new syllable cannot be created from a single final
        match self.finale {
            Some(fj) => {
                self.finale = Some(fj.append(finale)?);
                Ok(None)
            }
            None => {
                self.finale = Some(finale);
                Ok(None)
            }
        }
    }
}
impl From<InitialJamo> for Syllable {
    fn from(value: InitialJamo) -> Self {
        Self {
            initial: Some(value),
            ..Default::default()
        }
    }
}
impl From<MedialJamo> for Syllable {
    fn from(value: MedialJamo) -> Self {
        Self {
            initial: Some(InitialJamo::Silent),
            medial: Some(value),
            ..Default::default()
        }
    }
}
impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ij) = self.initial
            && self.medial.is_none()
        {
            write!(f, "{}", Jamo::from(ij))
        } else if let Some(ij) = self.initial
            && let Some(mj) = self.medial
        {
            let ini = ij.id();
            let med = mj.id();
            let fin = self.finale.map(FinalJamo::id).unwrap_or(0);

            let unicode: u32 = (0xac00 + ini * 588 + med * 28 + fin) as u32;

            // Safety: all combinations of ini, med, fin are guaranteed to be
            // less than 0xD7A4
            let chr = unsafe { char::from_u32_unchecked(unicode) };

            f.write_char(chr)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SyllableError {
    #[error("Expected Initial or Medial got: Jamo {0} <{0:?}>")]
    ExpectedInitialOrMedial(Jamo),
    #[error("Expected Medial got: Jamo {0} <{0:?}>")]
    ExpectedMedial(Jamo),
    #[error(transparent)]
    Jamo(#[from] JamoError),
}
pub type SyllableResult<T> = Result<T, SyllableError>;
