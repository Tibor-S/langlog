use std::{
    char,
    collections::HashSet,
    fmt::{Debug, Display, Write},
};

/// ## Jamo
/// Can represent all relevant jamo
/// enum values are the unicode value
/// for specific jamo
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Jamo {
    ///ㄱ
    G = 0x3131,
    ///ㄲ
    Gg,
    ///ㄳ
    Gs,
    ///ㄴ
    N,
    ///ㄵ
    Nc,
    ///ㄶ
    Nch,
    ///ㄷ
    D,
    ///ㄸ
    Dd,
    ///ㄹ
    R,
    ///ㄺ
    Lg,
    ///ㄻ
    Lm,
    ///ㄼ
    Lb,
    ///ㄽ
    Ls,
    ///ㄾ
    Lt,
    ///ㄿ
    Lph,
    ///ㅀ
    Lh,
    ///ㅁ
    M,
    ///ㅂ
    B,
    ///ㅃ
    Bb,
    ///ㅄ
    Bs,
    ///ㅅ
    S,
    ///ㅆ
    Ss,
    ///ㅇ
    Silent,
    ///ㅈ
    J,
    ///ㅉ
    Jj,
    ///ㅊ
    Ch,
    ///ㅋ
    K,
    ///ㅌ
    T,
    ///ㅍ
    P,
    ///ㅎ
    H,
    ///ㅏ
    A,
    ///ㅐ
    Ae,
    ///ㅑ
    Ya,
    ///ㅒ
    Yae,
    ///ㅓ
    Eo,
    ///ㅔ
    E,
    ///ㅕ
    Yeo,
    ///ㅖ
    Ye,
    ///ㅗ
    O,
    ///ㅘ
    Wa,
    ///ㅙ
    Wae,
    ///ㅚ
    Oe,
    ///ㅛ
    Yo,
    ///ㅜ
    U,
    ///ㅝ
    Wo,
    ///ㅞ
    We,
    ///ㅟ
    Wi,
    ///ㅠ
    Yu,
    ///ㅡ
    Eu,
    ///ㅢ
    Ui,
    ///ㅣ
    I,
}
impl Jamo {
    pub fn id(self) -> usize {
        self as usize - 0x3131
    }

    pub fn all() -> Vec<Jamo> {
        vec![
            Jamo::G,
            Jamo::Gg,
            Jamo::Gs,
            Jamo::N,
            Jamo::Nc,
            Jamo::Nch,
            Jamo::D,
            Jamo::Dd,
            Jamo::R,
            Jamo::Lg,
            Jamo::Lm,
            Jamo::Lb,
            Jamo::Ls,
            Jamo::Lt,
            Jamo::Lph,
            Jamo::Lh,
            Jamo::M,
            Jamo::B,
            Jamo::Bb,
            Jamo::Bs,
            Jamo::S,
            Jamo::Ss,
            Jamo::Silent,
            Jamo::J,
            Jamo::Jj,
            Jamo::Ch,
            Jamo::K,
            Jamo::T,
            Jamo::P,
            Jamo::H,
            Jamo::A,
            Jamo::Ae,
            Jamo::Ya,
            Jamo::Yae,
            Jamo::Eo,
            Jamo::E,
            Jamo::Yeo,
            Jamo::Ye,
            Jamo::O,
            Jamo::Wa,
            Jamo::Wae,
            Jamo::Oe,
            Jamo::Yo,
            Jamo::U,
            Jamo::Wo,
            Jamo::We,
            Jamo::Wi,
            Jamo::Yu,
            Jamo::Eu,
            Jamo::Ui,
            Jamo::I,
        ]
    }

    pub fn all_initial() -> Vec<Jamo> {
        InitialJamo::all().iter().map(Jamo::from).collect()
    }

    pub fn all_medial() -> Vec<Jamo> {
        MedialJamo::all().iter().map(Jamo::from).collect()
    }

    pub fn possible_medial(jamo: MedialJamo) -> Vec<Jamo> {
        jamo.combine_possible().iter().map(Jamo::from).collect()
    }

    pub fn all_finale() -> Vec<Jamo> {
        FinalJamo::all().iter().map(Jamo::from).collect()
    }

    pub fn possible_finale(jamo: FinalJamo) -> Vec<Jamo> {
        jamo.append_possible().iter().map(Jamo::from).collect()
    }

    pub fn all_multi(initial: bool, medial: bool, finale: bool) -> Vec<Jamo> {
        let mut s = HashSet::<Jamo>::default();
        if initial {
            s.extend(InitialJamo::all().iter().map(Jamo::from));
        }
        if medial {
            s.extend(MedialJamo::all().iter().map(Jamo::from));
        }
        if finale {
            s.extend(FinalJamo::all().iter().map(Jamo::from));
        }
        s.into_iter().collect()
    }

    pub fn all_or_possible(
        initial: bool,
        medial: (bool, Option<MedialJamo>),
        finale: (bool, Option<FinalJamo>),
    ) -> Vec<Jamo> {
        let mut s = HashSet::<Jamo>::default();
        if initial {
            s.extend(InitialJamo::all().iter().map(Jamo::from));
        }
        if medial.0
            && let Some(jamo) = medial.1
        {
            s.extend(jamo.combine_possible().iter().map(Jamo::from));
        } else if medial.0 {
            s.extend(MedialJamo::all().iter().map(Jamo::from));
        }
        if finale.0
            && let Some(jamo) = finale.1
        {
            s.extend(jamo.append_possible().iter().map(Jamo::from));
        } else if finale.0 {
            s.extend(FinalJamo::all().iter().map(Jamo::from));
        }
        s.into_iter().collect()
    }
}
impl From<Jamo> for char {
    fn from(value: Jamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(value as u32) }
    }
}
impl From<&Jamo> for char {
    fn from(value: &Jamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(*value as u32) }
    }
}
impl Debug for Jamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::G => write!(f, "G({:#x})", *self as u32),
            Self::Gg => write!(f, "Gg({:#x})", *self as u32),
            Self::Gs => write!(f, "Gs({:#x})", *self as u32),
            Self::N => write!(f, "N({:#x})", *self as u32),
            Self::Nc => write!(f, "Nc({:#x})", *self as u32),
            Self::Nch => write!(f, "Nch({:#x})", *self as u32),
            Self::D => write!(f, "D({:#x})", *self as u32),
            Self::Dd => write!(f, "Dd({:#x})", *self as u32),
            Self::R => write!(f, "R({:#x})", *self as u32),
            Self::Lg => write!(f, "Lg({:#x})", *self as u32),
            Self::Lm => write!(f, "Lm({:#x})", *self as u32),
            Self::Lb => write!(f, "Lb({:#x})", *self as u32),
            Self::Ls => write!(f, "Ls({:#x})", *self as u32),
            Self::Lt => write!(f, "Lt({:#x})", *self as u32),
            Self::Lph => write!(f, "Lph({:#x})", *self as u32),
            Self::Lh => write!(f, "Lh({:#x})", *self as u32),
            Self::M => write!(f, "M({:#x})", *self as u32),
            Self::B => write!(f, "B({:#x})", *self as u32),
            Self::Bb => write!(f, "Bb({:#x})", *self as u32),
            Self::Bs => write!(f, "Bs({:#x})", *self as u32),
            Self::S => write!(f, "S({:#x})", *self as u32),
            Self::Ss => write!(f, "Ss({:#x})", *self as u32),
            Self::Silent => write!(f, "Silent({:#x})", *self as u32),
            Self::J => write!(f, "J({:#x})", *self as u32),
            Self::Jj => write!(f, "Jj({:#x})", *self as u32),
            Self::Ch => write!(f, "Ch({:#x})", *self as u32),
            Self::K => write!(f, "K({:#x})", *self as u32),
            Self::T => write!(f, "T({:#x})", *self as u32),
            Self::P => write!(f, "P({:#x})", *self as u32),
            Self::H => write!(f, "H({:#x})", *self as u32),
            Self::A => write!(f, "A({:#x})", *self as u32),
            Self::Ae => write!(f, "Ae({:#x})", *self as u32),
            Self::Ya => write!(f, "Ya({:#x})", *self as u32),
            Self::Yae => write!(f, "Yae({:#x})", *self as u32),
            Self::Eo => write!(f, "Eo({:#x})", *self as u32),
            Self::E => write!(f, "E({:#x})", *self as u32),
            Self::Yeo => write!(f, "Yeo({:#x})", *self as u32),
            Self::Ye => write!(f, "Ye({:#x})", *self as u32),
            Self::O => write!(f, "O({:#x})", *self as u32),
            Self::Wa => write!(f, "Wa({:#x})", *self as u32),
            Self::Wae => write!(f, "Wae({:#x})", *self as u32),
            Self::Oe => write!(f, "Oe({:#x})", *self as u32),
            Self::Yo => write!(f, "Yo({:#x})", *self as u32),
            Self::U => write!(f, "U({:#x})", *self as u32),
            Self::Wo => write!(f, "Wo({:#x})", *self as u32),
            Self::We => write!(f, "We({:#x})", *self as u32),
            Self::Wi => write!(f, "Wi({:#x})", *self as u32),
            Self::Yu => write!(f, "Yu({:#x})", *self as u32),
            Self::Eu => write!(f, "Eu({:#x})", *self as u32),
            Self::Ui => write!(f, "Ui({:#x})", *self as u32),
            Self::I => write!(f, "I({:#x})", *self as u32),
        }
    }
}
impl Display for Jamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InitialJamo {
    ///ㄱ
    G = 0x1100,
    ///ㄲ
    Gg,
    ///ㄴ
    N,
    ///ㄷ
    D,
    ///ㄸ
    Dd,
    ///ㄹ
    R,
    ///ㅁ
    M,
    ///ㅂ
    B,
    ///ㅃ
    Bb,
    ///ㅅ
    S,
    ///ㅆ
    Ss,
    ///ㅇ
    Silent,
    ///ㅈ
    J,
    ///ㅉ
    Jj,
    ///ㅊ
    Ch,
    ///ㅋ
    K,
    ///ㅌ
    T,
    ///ㅍ
    P,
    ///ㅎ
    H,
}
impl InitialJamo {
    pub fn id(self) -> usize {
        self as usize - 0x1100
    }

    pub fn all() -> Vec<InitialJamo> {
        vec![
            InitialJamo::G,
            InitialJamo::Gg,
            InitialJamo::N,
            InitialJamo::D,
            InitialJamo::Dd,
            InitialJamo::R,
            InitialJamo::M,
            InitialJamo::B,
            InitialJamo::Bb,
            InitialJamo::S,
            InitialJamo::Ss,
            InitialJamo::Silent,
            InitialJamo::J,
            InitialJamo::Jj,
            InitialJamo::Ch,
            InitialJamo::K,
            InitialJamo::T,
            InitialJamo::P,
            InitialJamo::H,
        ]
    }
}
impl From<InitialJamo> for Jamo {
    fn from(value: InitialJamo) -> Self {
        match value {
            InitialJamo::G => Jamo::G,
            InitialJamo::Gg => Jamo::Gg,
            InitialJamo::N => Jamo::N,
            InitialJamo::D => Jamo::D,
            InitialJamo::Dd => Jamo::Dd,
            InitialJamo::R => Jamo::R,
            InitialJamo::M => Jamo::M,
            InitialJamo::B => Jamo::B,
            InitialJamo::Bb => Jamo::Bb,
            InitialJamo::S => Jamo::S,
            InitialJamo::Ss => Jamo::Ss,
            InitialJamo::Silent => Jamo::Silent,
            InitialJamo::J => Jamo::J,
            InitialJamo::Jj => Jamo::Jj,
            InitialJamo::Ch => Jamo::Ch,
            InitialJamo::K => Jamo::K,
            InitialJamo::T => Jamo::T,
            InitialJamo::P => Jamo::P,
            InitialJamo::H => Jamo::H,
        }
    }
}
impl From<&InitialJamo> for Jamo {
    fn from(value: &InitialJamo) -> Self {
        Jamo::from(*value)
    }
}
impl TryFrom<Jamo> for InitialJamo {
    type Error = JamoError;

    fn try_from(value: Jamo) -> JamoResult<InitialJamo> {
        match value {
            Jamo::G => Ok(InitialJamo::G),
            Jamo::Gg => Ok(InitialJamo::Gg),
            Jamo::N => Ok(InitialJamo::N),
            Jamo::D => Ok(InitialJamo::D),
            Jamo::Dd => Ok(InitialJamo::Dd),
            Jamo::R => Ok(InitialJamo::R),
            Jamo::M => Ok(InitialJamo::M),
            Jamo::B => Ok(InitialJamo::B),
            Jamo::Bb => Ok(InitialJamo::Bb),
            Jamo::S => Ok(InitialJamo::S),
            Jamo::Ss => Ok(InitialJamo::Ss),
            Jamo::Silent => Ok(InitialJamo::Silent),
            Jamo::J => Ok(InitialJamo::J),
            Jamo::Jj => Ok(InitialJamo::Jj),
            Jamo::Ch => Ok(InitialJamo::Ch),
            Jamo::K => Ok(InitialJamo::K),
            Jamo::T => Ok(InitialJamo::T),
            Jamo::P => Ok(InitialJamo::P),
            Jamo::H => Ok(InitialJamo::H),
            _ => Err(JamoError::UnexpectedJamo(value)),
        }
    }
}
impl From<InitialJamo> for char {
    fn from(value: InitialJamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(value as u32) }
    }
}
impl From<&InitialJamo> for char {
    fn from(value: &InitialJamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(*value as u32) }
    }
}
impl Debug for InitialJamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::G => write!(f, "G({:#x})", *self as u32),
            Self::Gg => write!(f, "Gg({:#x})", *self as u32),
            Self::N => write!(f, "N({:#x})", *self as u32),
            Self::D => write!(f, "D({:#x})", *self as u32),
            Self::Dd => write!(f, "Dd({:#x})", *self as u32),
            Self::R => write!(f, "R({:#x})", *self as u32),
            Self::M => write!(f, "M({:#x})", *self as u32),
            Self::B => write!(f, "B({:#x})", *self as u32),
            Self::Bb => write!(f, "Bb({:#x})", *self as u32),
            Self::S => write!(f, "S({:#x})", *self as u32),
            Self::Ss => write!(f, "Ss({:#x})", *self as u32),
            Self::Silent => write!(f, "Silent({:#x})", *self as u32),
            Self::J => write!(f, "J({:#x})", *self as u32),
            Self::Jj => write!(f, "Jj({:#x})", *self as u32),
            Self::Ch => write!(f, "Ch({:#x})", *self as u32),
            Self::K => write!(f, "K({:#x})", *self as u32),
            Self::T => write!(f, "T({:#x})", *self as u32),
            Self::P => write!(f, "P({:#x})", *self as u32),
            Self::H => write!(f, "H({:#x})", *self as u32),
        }
    }
}
impl Display for InitialJamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MedialJamo {
    ///ㅏ
    A = 0x1161,
    ///ㅐ
    Ae,
    ///ㅑ
    Ya,
    ///ㅒ
    Yae,
    ///ㅓ
    Eo,
    ///ㅔ
    E,
    ///ㅕ
    Yeo,
    ///ㅖ
    Ye,
    ///ㅗ
    O,
    ///ㅘ
    Wa,
    ///ㅙ
    Wae,
    ///ㅚ
    Oe,
    ///ㅛ
    Yo,
    ///ㅜ
    U,
    ///ㅝ
    Wo,
    ///ㅞ
    We,
    ///ㅟ
    Wi,
    ///ㅠ
    Yu,
    ///ㅡ
    Eu,
    ///ㅢ
    Ui,
    ///ㅣ
    I,
}
#[derive(Debug, Clone, Copy)]
pub enum MedialKind {
    Tall,
    Wide,
    Full,
}
impl MedialJamo {
    pub fn id(self) -> usize {
        self as usize - 0x1161
    }

    pub fn all() -> Vec<MedialJamo> {
        vec![
            MedialJamo::A,
            MedialJamo::Ae,
            MedialJamo::Ya,
            MedialJamo::Yae,
            MedialJamo::Eo,
            MedialJamo::E,
            MedialJamo::Yeo,
            MedialJamo::Ye,
            MedialJamo::O,
            MedialJamo::Wa,
            MedialJamo::Wae,
            MedialJamo::Oe,
            MedialJamo::Yo,
            MedialJamo::U,
            MedialJamo::Wo,
            MedialJamo::We,
            MedialJamo::Wi,
            MedialJamo::Yu,
            MedialJamo::Eu,
            MedialJamo::Ui,
            MedialJamo::I,
        ]
    }

    pub fn kind(&self) -> MedialKind {
        match self {
            MedialJamo::A => MedialKind::Tall,
            MedialJamo::Ae => MedialKind::Tall,
            MedialJamo::Ya => MedialKind::Tall,
            MedialJamo::Yae => MedialKind::Tall,
            MedialJamo::Eo => MedialKind::Tall,
            MedialJamo::E => MedialKind::Tall,
            MedialJamo::Yeo => MedialKind::Tall,
            MedialJamo::Ye => MedialKind::Tall,
            MedialJamo::O => MedialKind::Wide,
            MedialJamo::Wa => MedialKind::Full,
            MedialJamo::Wae => MedialKind::Full,
            MedialJamo::Oe => MedialKind::Full,
            MedialJamo::Yo => MedialKind::Wide,
            MedialJamo::U => MedialKind::Wide,
            MedialJamo::Wo => MedialKind::Full,
            MedialJamo::We => MedialKind::Full,
            MedialJamo::Wi => MedialKind::Full,
            MedialJamo::Yu => MedialKind::Wide,
            MedialJamo::Eu => MedialKind::Wide,
            MedialJamo::Ui => MedialKind::Full,
            MedialJamo::I => MedialKind::Tall,
        }
    }

    pub fn combine_possible(self) -> Vec<MedialJamo> {
        match self {
            MedialJamo::A => vec![MedialJamo::O],
            MedialJamo::Ae => vec![MedialJamo::O],
            MedialJamo::Eo => vec![MedialJamo::U],
            MedialJamo::E => vec![MedialJamo::U],
            MedialJamo::O => vec![MedialJamo::A, MedialJamo::Ae, MedialJamo::I],
            MedialJamo::U => vec![MedialJamo::Eo, MedialJamo::E, MedialJamo::I],
            MedialJamo::Eu => vec![MedialJamo::I],
            MedialJamo::I => vec![MedialJamo::O, MedialJamo::U, MedialJamo::Eu],
            _ => vec![],
        }
    }

    pub fn combine(self, other: MedialJamo) -> JamoResult<Self> {
        // Correction: Wide before Tall
        match (self.kind(), other.kind()) {
            (MedialKind::Wide, MedialKind::Tall) => (), // continue
            (MedialKind::Tall, MedialKind::Wide) => {
                return other.combine(self);
            }
            _ => {
                return Err(JamoError::IncompatibleCombine(
                    self.into(),
                    other.into(),
                ));
            }
        };

        match (self, other) {
            (MedialJamo::O, MedialJamo::A) => Ok(MedialJamo::Wa),
            (MedialJamo::O, MedialJamo::Ae) => Ok(MedialJamo::Wae),
            (MedialJamo::O, MedialJamo::I) => Ok(MedialJamo::Oe),
            (MedialJamo::U, MedialJamo::Eo) => Ok(MedialJamo::Wo),
            (MedialJamo::U, MedialJamo::E) => Ok(MedialJamo::We),
            (MedialJamo::U, MedialJamo::I) => Ok(MedialJamo::Wi),
            (MedialJamo::Eu, MedialJamo::I) => Ok(MedialJamo::Ui),
            _ => Err(JamoError::IncompatibleCombine(self.into(), other.into())),
        }
    }
}
impl From<MedialJamo> for Jamo {
    fn from(value: MedialJamo) -> Self {
        match value {
            MedialJamo::A => Jamo::A,
            MedialJamo::Ae => Jamo::Ae,
            MedialJamo::Ya => Jamo::Ya,
            MedialJamo::Yae => Jamo::Yae,
            MedialJamo::Eo => Jamo::Eo,
            MedialJamo::E => Jamo::E,
            MedialJamo::Yeo => Jamo::Yeo,
            MedialJamo::Ye => Jamo::Ye,
            MedialJamo::O => Jamo::O,
            MedialJamo::Wa => Jamo::Wa,
            MedialJamo::Wae => Jamo::Wae,
            MedialJamo::Oe => Jamo::Oe,
            MedialJamo::Yo => Jamo::Yo,
            MedialJamo::U => Jamo::U,
            MedialJamo::Wo => Jamo::Wo,
            MedialJamo::We => Jamo::We,
            MedialJamo::Wi => Jamo::Wi,
            MedialJamo::Yu => Jamo::Yu,
            MedialJamo::Eu => Jamo::Eu,
            MedialJamo::Ui => Jamo::Ui,
            MedialJamo::I => Jamo::I,
        }
    }
}
impl From<&MedialJamo> for Jamo {
    fn from(value: &MedialJamo) -> Self {
        Jamo::from(*value)
    }
}
impl TryFrom<Jamo> for MedialJamo {
    type Error = JamoError;

    fn try_from(value: Jamo) -> JamoResult<MedialJamo> {
        match value {
            Jamo::A => Ok(MedialJamo::A),
            Jamo::Ae => Ok(MedialJamo::Ae),
            Jamo::Ya => Ok(MedialJamo::Ya),
            Jamo::Yae => Ok(MedialJamo::Yae),
            Jamo::Eo => Ok(MedialJamo::Eo),
            Jamo::E => Ok(MedialJamo::E),
            Jamo::Yeo => Ok(MedialJamo::Yeo),
            Jamo::Ye => Ok(MedialJamo::Ye),
            Jamo::O => Ok(MedialJamo::O),
            Jamo::Wa => Ok(MedialJamo::Wa),
            Jamo::Wae => Ok(MedialJamo::Wae),
            Jamo::Oe => Ok(MedialJamo::Oe),
            Jamo::Yo => Ok(MedialJamo::Yo),
            Jamo::U => Ok(MedialJamo::U),
            Jamo::Wo => Ok(MedialJamo::Wo),
            Jamo::We => Ok(MedialJamo::We),
            Jamo::Wi => Ok(MedialJamo::Wi),
            Jamo::Yu => Ok(MedialJamo::Yu),
            Jamo::Eu => Ok(MedialJamo::Eu),
            Jamo::Ui => Ok(MedialJamo::Ui),
            Jamo::I => Ok(MedialJamo::I),
            _ => Err(JamoError::UnexpectedJamo(value)),
        }
    }
}
impl From<MedialJamo> for char {
    fn from(value: MedialJamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(value as u32) }
    }
}
impl From<&MedialJamo> for char {
    fn from(value: &MedialJamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(*value as u32) }
    }
}
impl Debug for MedialJamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A({:#x})", *self as u32),
            Self::Ae => write!(f, "Ae({:#x})", *self as u32),
            Self::Ya => write!(f, "Ya({:#x})", *self as u32),
            Self::Yae => write!(f, "Yae({:#x})", *self as u32),
            Self::Eo => write!(f, "Eo({:#x})", *self as u32),
            Self::E => write!(f, "E({:#x})", *self as u32),
            Self::Yeo => write!(f, "Yeo({:#x})", *self as u32),
            Self::Ye => write!(f, "Ye({:#x})", *self as u32),
            Self::O => write!(f, "O({:#x})", *self as u32),
            Self::Wa => write!(f, "Wa({:#x})", *self as u32),
            Self::Wae => write!(f, "Wae({:#x})", *self as u32),
            Self::Oe => write!(f, "Oe({:#x})", *self as u32),
            Self::Yo => write!(f, "Yo({:#x})", *self as u32),
            Self::U => write!(f, "U({:#x})", *self as u32),
            Self::Wo => write!(f, "Wo({:#x})", *self as u32),
            Self::We => write!(f, "We({:#x})", *self as u32),
            Self::Wi => write!(f, "Wi({:#x})", *self as u32),
            Self::Yu => write!(f, "Yu({:#x})", *self as u32),
            Self::Eu => write!(f, "Eu({:#x})", *self as u32),
            Self::Ui => write!(f, "Ui({:#x})", *self as u32),
            Self::I => write!(f, "I({:#x})", *self as u32),
        }
    }
}
impl Display for MedialJamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}
impl Display for MedialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FinalJamo {
    ///ㄱ
    G = 0x11A8,
    ///ㄲ
    Gg,
    ///ㄳ
    Gs,
    ///ㄴ
    N,
    ///ㄵ
    Nc,
    ///ㄶ
    Nch,
    ///ㄷ
    D,
    ///ㄹ
    R,
    ///ㄺ
    Lg,
    ///ㄻ
    Lm,
    ///ㄼ
    Lb,
    ///ㄽ
    Ls,
    ///ㄾ
    Lt,
    ///ㄿ
    Lph,
    ///ㅀ
    Lh,
    ///ㅁ
    M,
    ///ㅂ
    B,
    ///ㅄ
    Bs,
    ///ㅅ
    S,
    ///ㅆ
    Ss,
    ///ㅇ
    Silent,
    ///ㅈ
    J,
    ///ㅊ
    Ch,
    ///ㅋ
    K,
    ///ㅌ
    T,
    ///ㅍ
    P,
    ///ㅎ
    H,
}
impl FinalJamo {
    pub fn id(self) -> usize {
        // 1-indexed since id is modelled after
        // unicode block, where the 0-index refers
        // to no final jamo.
        self as usize - 0x11A8 + 1
    }

    pub fn all() -> Vec<FinalJamo> {
        vec![
            FinalJamo::G,
            FinalJamo::Gg,
            FinalJamo::Gs,
            FinalJamo::N,
            FinalJamo::Nc,
            FinalJamo::Nch,
            FinalJamo::D,
            FinalJamo::R,
            FinalJamo::Lg,
            FinalJamo::Lm,
            FinalJamo::Lb,
            FinalJamo::Ls,
            FinalJamo::Lt,
            FinalJamo::Lph,
            FinalJamo::Lh,
            FinalJamo::M,
            FinalJamo::B,
            FinalJamo::Bs,
            FinalJamo::S,
            FinalJamo::Ss,
            FinalJamo::Silent,
            FinalJamo::J,
            FinalJamo::Ch,
            FinalJamo::K,
            FinalJamo::T,
            FinalJamo::P,
            FinalJamo::H,
        ]
    }
    pub fn append_possible(self) -> Vec<FinalJamo> {
        match self {
            FinalJamo::G => vec![FinalJamo::S],
            FinalJamo::N => vec![FinalJamo::J, FinalJamo::H],
            FinalJamo::R => vec![
                FinalJamo::G,
                FinalJamo::M,
                FinalJamo::B,
                FinalJamo::S,
                FinalJamo::T,
                FinalJamo::P,
                FinalJamo::H,
            ],
            FinalJamo::B => vec![FinalJamo::S],
            _ => vec![],
        }
    }

    pub fn append(self, other: FinalJamo) -> JamoResult<Self> {
        match (self, other) {
            (FinalJamo::G, FinalJamo::S) => Ok(FinalJamo::Gs),
            (FinalJamo::N, FinalJamo::J) => Ok(FinalJamo::Nc),
            (FinalJamo::N, FinalJamo::H) => Ok(FinalJamo::Nch),
            (FinalJamo::R, FinalJamo::G) => Ok(FinalJamo::Lg),
            (FinalJamo::R, FinalJamo::M) => Ok(FinalJamo::Lm),
            (FinalJamo::R, FinalJamo::B) => Ok(FinalJamo::Lb),
            (FinalJamo::R, FinalJamo::S) => Ok(FinalJamo::Ls),
            (FinalJamo::R, FinalJamo::T) => Ok(FinalJamo::Lt),
            (FinalJamo::R, FinalJamo::P) => Ok(FinalJamo::Lph),
            (FinalJamo::R, FinalJamo::H) => Ok(FinalJamo::Lh),
            (FinalJamo::B, FinalJamo::S) => Ok(FinalJamo::Bs),
            _ => Err(JamoError::IncompatibleCombine(self.into(), other.into())),
        }
    }
}
impl From<FinalJamo> for Jamo {
    fn from(value: FinalJamo) -> Self {
        match value {
            FinalJamo::G => Jamo::G,
            FinalJamo::Gg => Jamo::Gg,
            FinalJamo::Gs => Jamo::Gs,
            FinalJamo::N => Jamo::N,
            FinalJamo::Nc => Jamo::Nc,
            FinalJamo::Nch => Jamo::Nch,
            FinalJamo::D => Jamo::D,
            FinalJamo::R => Jamo::R,
            FinalJamo::Lg => Jamo::Lg,
            FinalJamo::Lm => Jamo::Lm,
            FinalJamo::Lb => Jamo::Lb,
            FinalJamo::Ls => Jamo::Ls,
            FinalJamo::Lt => Jamo::Lt,
            FinalJamo::Lph => Jamo::Lph,
            FinalJamo::Lh => Jamo::Lh,
            FinalJamo::M => Jamo::M,
            FinalJamo::B => Jamo::B,
            FinalJamo::Bs => Jamo::Bs,
            FinalJamo::S => Jamo::S,
            FinalJamo::Ss => Jamo::Ss,
            FinalJamo::Silent => Jamo::Silent,
            FinalJamo::J => Jamo::J,
            FinalJamo::Ch => Jamo::Ch,
            FinalJamo::K => Jamo::K,
            FinalJamo::T => Jamo::T,
            FinalJamo::P => Jamo::P,
            FinalJamo::H => Jamo::H,
        }
    }
}
impl From<&FinalJamo> for Jamo {
    fn from(value: &FinalJamo) -> Self {
        Jamo::from(*value)
    }
}
impl TryFrom<Jamo> for FinalJamo {
    type Error = JamoError;

    fn try_from(value: Jamo) -> JamoResult<FinalJamo> {
        match value {
            Jamo::G => Ok(FinalJamo::G),
            Jamo::Gg => Ok(FinalJamo::Gg),
            Jamo::Gs => Ok(FinalJamo::Gs),
            Jamo::N => Ok(FinalJamo::N),
            Jamo::Nc => Ok(FinalJamo::Nc),
            Jamo::Nch => Ok(FinalJamo::Nch),
            Jamo::D => Ok(FinalJamo::D),
            Jamo::R => Ok(FinalJamo::R),
            Jamo::Lg => Ok(FinalJamo::Lg),
            Jamo::Lm => Ok(FinalJamo::Lm),
            Jamo::Lb => Ok(FinalJamo::Lb),
            Jamo::Ls => Ok(FinalJamo::Ls),
            Jamo::Lt => Ok(FinalJamo::Lt),
            Jamo::Lph => Ok(FinalJamo::Lph),
            Jamo::Lh => Ok(FinalJamo::Lh),
            Jamo::M => Ok(FinalJamo::M),
            Jamo::B => Ok(FinalJamo::B),
            Jamo::Bs => Ok(FinalJamo::Bs),
            Jamo::S => Ok(FinalJamo::S),
            Jamo::Ss => Ok(FinalJamo::Ss),
            Jamo::Silent => Ok(FinalJamo::Silent),
            Jamo::J => Ok(FinalJamo::J),
            Jamo::Ch => Ok(FinalJamo::Ch),
            Jamo::K => Ok(FinalJamo::K),
            Jamo::T => Ok(FinalJamo::T),
            Jamo::P => Ok(FinalJamo::P),
            Jamo::H => Ok(FinalJamo::H),
            _ => Err(JamoError::UnexpectedJamo(value)),
        }
    }
}
impl From<FinalJamo> for char {
    fn from(value: FinalJamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(value as u32) }
    }
}
impl From<&FinalJamo> for char {
    fn from(value: &FinalJamo) -> Self {
        // Safe ! all variants of jamo have valid unicode values
        unsafe { char::from_u32_unchecked(*value as u32) }
    }
}
impl Debug for FinalJamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::G => write!(f, "G({:#x})", *self as u32),
            Self::Gg => write!(f, "Gg({:#x})", *self as u32),
            Self::Gs => write!(f, "Gs({:#x})", *self as u32),
            Self::N => write!(f, "N({:#x})", *self as u32),
            Self::Nc => write!(f, "Nc({:#x})", *self as u32),
            Self::Nch => write!(f, "Nch({:#x})", *self as u32),
            Self::D => write!(f, "D({:#x})", *self as u32),
            Self::R => write!(f, "R({:#x})", *self as u32),
            Self::Lg => write!(f, "Lg({:#x})", *self as u32),
            Self::Lm => write!(f, "Lm({:#x})", *self as u32),
            Self::Lb => write!(f, "Lb({:#x})", *self as u32),
            Self::Ls => write!(f, "Ls({:#x})", *self as u32),
            Self::Lt => write!(f, "Lt({:#x})", *self as u32),
            Self::Lph => write!(f, "Lph({:#x})", *self as u32),
            Self::Lh => write!(f, "Lh({:#x})", *self as u32),
            Self::M => write!(f, "M({:#x})", *self as u32),
            Self::B => write!(f, "B({:#x})", *self as u32),
            Self::Bs => write!(f, "Bs({:#x})", *self as u32),
            Self::S => write!(f, "S({:#x})", *self as u32),
            Self::Ss => write!(f, "Ss({:#x})", *self as u32),
            Self::Silent => write!(f, "Silent({:#x})", *self as u32),
            Self::J => write!(f, "J({:#x})", *self as u32),
            Self::Ch => write!(f, "Ch({:#x})", *self as u32),
            Self::K => write!(f, "K({:#x})", *self as u32),
            Self::T => write!(f, "T({:#x})", *self as u32),
            Self::P => write!(f, "P({:#x})", *self as u32),
            Self::H => write!(f, "H({:#x})", *self as u32),
        }
    }
}
impl Display for FinalJamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JamoError {
    #[error("Did not expect {0} <{0:?}>")]
    UnexpectedJamo(Jamo),
    #[error("Cannot combine {0} <{0:?}> with {1} <{1:?}>")]
    IncompatibleCombine(Jamo, Jamo),
}
pub type JamoResult<T> = Result<T, JamoError>;
