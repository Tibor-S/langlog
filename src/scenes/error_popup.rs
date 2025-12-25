use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    elements::{Button, Rectangle, TextLine},
};

pub fn error_popup_scene(
    full_wh: (u16, u16),
    heading: String,
    error_msg: &[String],
    bordered: bool,
) -> TerminalResult<Scene> {
    // border + button + heading + msg_height + potential margin
    let height =
        (2 + 2 + 1 + error_msg.len() + usize::from(error_msg.len() > 0)) as u16;
    let width = 11.max(
        Some(heading.len())
            .into_iter()
            .chain(error_msg.iter().map(|s| s.len()))
            .max()
            .expect("Logic error!")
            + 4, // + border + margin
    ) as u16;
    let width = if width & 1 == 1 { width + 1 } else { width };
    let (x, y) = (
        (full_wh.0 / 2).saturating_sub(width / 2),
        (full_wh.1 / 2).saturating_sub(height / 2),
    );
    let mut scene = Scene::new(SceneType::PopUp(x, y));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (width, height), bordered),
    )?;

    let hx = (width / 2).saturating_sub(heading.len() as u16 / 2);
    scene.insert_block(
        "heading".into(),
        TextLine::default()
            .with_pos(hx, 1)
            .with_width(heading.len() as u16)
            .with_value(heading)
            .clone(),
    )?;
    for (i, msg) in error_msg.iter().enumerate() {
        scene.insert_block(
            "msg-".chars().chain(i.to_string().chars()).collect(),
            TextLine::default()
                .with_pos(2, 3 + i as u16)
                .with_width(msg.len() as u16)
                .with_value(msg.clone())
                .clone(),
        )?;
    }

    let bx = (width / 2).saturating_sub(9 / 2);
    let button = Button::new(
        (bx, height as u16 - 2, 0),
        "Close".into(),
        9,
        2,
        Some(|| TerminalCode::PreviousScene),
    );
    scene.insert_input(button);
    Ok(scene)
}
