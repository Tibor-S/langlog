use terminal::traits::Block;

#[derive(Debug, Default, Clone)]
pub struct JamoInfo((u16, u16, u16));
impl JamoInfo {
    // Assuming Jamo take 'two' slots
    const LINES: [&str; 15] = [
        " Initials/Finals        Medials         ",
        " ㄱ g                   ㅏ a            ",
        " ㄴ n                   ㅐ ae           ",
        " ㄷ d                   ㅑ ya           ",
        " ㄹ r                   ㅒ yae          ",
        " ㅁ m                   ㅓ eo           ",
        " ㅂ b                   ㅔ e            ",
        " ㅅ s                   ㅕ yeo          ",
        " ㅇ ng                  ㅖ ye           ",
        " ㅈ j                   ㅗ o            ",
        " ㅊ ch                  ㅛ yo           ",
        " ㅋ k                   ㅜ u            ",
        " ㅌ t                   ㅠ yu           ",
        " ㅍ p                   ㅡ eu           ",
        " ㅎ h                   ㅣ i            ",
    ];
    pub fn new(pos: (u16, u16, u16)) -> Self {
        Self(pos)
    }
}
impl Block for JamoInfo {
    fn pos(&self) -> (u16, u16, u16) {
        self.0
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        Self::LINES.get(i as usize).map(|&s| s.into())
    }
}
