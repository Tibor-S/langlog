use terminal::{
    Scene, SceneType, TerminalResult,
    elements::{Rectangle, TextLine},
};

pub fn help_menu_scene() -> TerminalResult<Scene> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (57, 21), true),
    )?;
    scene.insert_block(
        "heading".into(),
        TextLine::default()
            .with_pos(1, 1)
            .with_width(" Help:".len() as u16)
            .with_value(" Help:".into())
            .clone(),
    )?;
    scene.insert_block(
        "esc-pop-ups".into(),
        TextLine::default()
            .with_pos(1, 3)
            .with_width(
                " - Esc . . . . . . Exit pop-ups like \"Help\"".len() as u16
            )
            .with_value(" - Esc . . . . . . Exit pop-ups like \"Help\"".into())
            .clone(),
    )?;
    scene.insert_block(
        "next-input".into(),
        TextLine::default()
            .with_pos(1, 5)
            .with_width(" - Tab . . . . . . Go to next input".len() as u16)
            .with_value(" - Tab . . . . . . Go to next input".into())
            .clone(),
    )?;
    scene.insert_block(
        "prev-input".into(),
        TextLine::default()
            .with_pos(1, 7)
            .with_width(" - Shift + Tab . . Go to previous input".len() as u16)
            .with_value(" - Shift + Tab . . Go to previous input".into())
            .clone(),
    )?;
    scene.insert_block(
        "submit-rr".into(),
        TextLine::default()
            .with_pos(1, 9)
            .with_width(
                " - Enter . . . . . Submit RR characters as syllable".len()
                    as u16,
            )
            .with_value(
                " - Enter . . . . . Submit RR characters as syllable".into(),
            )
            .clone(),
    )?;
    Ok(scene)
}
