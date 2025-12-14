use std::fmt::{Display, Write};

use crate::jamo::{FinalJamo, InitialJamo, Jamo, JamoError, MedialJamo};

#[derive(Debug, Clone, Copy)]
pub enum State {
    /// Next input must be initial or medial
    Start,
    /// Next input must be medial
    Medial(InitialJamo),
    /// Syllable could end here, or next input must be medial or final
    Open(InitialJamo, MedialJamo),
    /// Syllable could end here or input must be final
    OpenFinal(InitialJamo, MedialJamo, Option<FinalJamo>),
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
            (Some(i), None, _) => State::Medial(i),
            (Some(i), Some(m), None) if !m.combine_possible().is_empty() => {
                State::Open(i, m)
            }
            (Some(i), Some(m), None) => State::OpenFinal(i, m, None),
            (Some(i), Some(m), Some(f)) if !f.append_possible().is_empty() => {
                State::OpenFinal(i, m, Some(f))
            }
            _ => State::End,
        }
    }

    pub fn possible(self) -> Vec<Jamo> {
        match self.state() {
            State::Start => Jamo::all_multi(true, true, false),
            State::Medial(_) => Jamo::all_medial(),
            State::Open(_, _) => Jamo::all_or_possible(
                false,
                (true, self.medial),
                (true, self.finale),
            ),
            State::OpenFinal(_, _, _) => {
                Jamo::all_or_possible(false, (false, None), (true, self.finale))
            }
            State::End => vec![],
        }
    }

    pub fn append(self, jamo: Jamo) -> SyllableResult<Self> {
        match self.state() {
            State::Start => {
                if let Ok(ij) = InitialJamo::try_from(jamo) {
                    self.append_initial(ij)
                } else if let Ok(mj) = MedialJamo::try_from(jamo) {
                    self.append_medial(mj)
                } else {
                    Err(SyllableError::UnexpectedJamo(jamo, State::Start))
                }
            }
            State::Medial(i) => {
                if let Ok(mj) = MedialJamo::try_from(jamo) {
                    self.append_medial(mj)
                } else {
                    Err(SyllableError::UnexpectedJamo(jamo, State::Medial(i)))
                }
            }
            State::Open(i, m) => {
                if let Ok(ij) = FinalJamo::try_from(jamo) {
                    self.append_final(ij)
                } else if let Ok(mj) = MedialJamo::try_from(jamo) {
                    self.append_medial(mj)
                } else {
                    Err(SyllableError::UnexpectedJamo(jamo, State::Open(i, m)))
                }
            }
            State::OpenFinal(i, m, j) => {
                if let Ok(ij) = FinalJamo::try_from(jamo) {
                    self.append_final(ij)
                } else {
                    Err(SyllableError::UnexpectedJamo(
                        jamo,
                        State::OpenFinal(i, m, j),
                    ))
                }
            }
            State::End => Err(SyllableError::UnexpectedJamo(jamo, State::End)),
        }
    }

    pub fn append_initial(self, jamo: InitialJamo) -> SyllableResult<Self> {
        match self.state() {
            State::Start => Ok(Self {
                initial: Some(jamo),
                ..self
            }),
            s => Err(SyllableError::UnexpectedInitial(jamo, s)),
        }
    }

    pub fn append_medial(self, jamo: MedialJamo) -> SyllableResult<Self> {
        match self.state() {
            State::Start => Ok(Self {
                initial: Some(InitialJamo::Silent),
                medial: Some(jamo),
                ..self
            }),
            State::Medial(_) => Ok(Self {
                medial: Some(jamo),
                ..self
            }),
            State::Open(_, mj) => Ok(Self {
                medial: Some(mj.combine(jamo)?),
                ..self
            }),
            s => Err(SyllableError::UnexpectedMedial(jamo, s)),
        }
    }

    pub fn append_final(self, jamo: FinalJamo) -> SyllableResult<Self> {
        match self.state() {
            State::Open(_, _) | State::OpenFinal(_, _, None) => Ok(Self {
                finale: Some(jamo),
                ..self
            }),
            State::OpenFinal(_, _, Some(fj)) => Ok(Self {
                finale: Some(fj.append(jamo)?),
                ..self
            }),
            s => Err(SyllableError::UnexpectedFinal(jamo, s)),
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
    #[error("Initial {0} <{0:?}> not expected in state {1:?}")]
    UnexpectedInitial(InitialJamo, State),
    #[error("Medial {0} <{0:?}> not expected in state {1:?}")]
    UnexpectedMedial(MedialJamo, State),
    #[error("Final {0} <{0:?}> not expected in state {1:?}")]
    UnexpectedFinal(FinalJamo, State),
    #[error("Jamo {0} <{0:?}> not expected in state {1:?}")]
    UnexpectedJamo(Jamo, State),
    #[error(transparent)]
    Jamo(#[from] JamoError),
}
pub type SyllableResult<T> = Result<T, SyllableError>;
