use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = true;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
