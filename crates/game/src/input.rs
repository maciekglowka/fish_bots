use rogalik::{
    engine::input::{KeyCode, MouseButton},
    prelude::*,
};

use game_graphics::input::{ButtonState, InputState};

pub fn get_input_state(camera: ResourceId, context: &Context) -> InputState {
    let mut click = ButtonState::Up;

    if context.input.is_mouse_button_down(MouseButton::Left) {
        click = ButtonState::Down
    }
    if context.input.is_mouse_button_released(MouseButton::Left) {
        click = ButtonState::Released
    }
    if context.input.is_mouse_button_pressed(MouseButton::Left) {
        click = ButtonState::Pressed
    }

    let m = context.input.get_mouse_physical_position();
    let mut w = Vector2f::ZERO;
    if let Some(camera) = context.graphics.get_camera(camera) {
        w = camera.camera_to_world(m);
    }

    let mut dir = None;
    if context.input.is_key_pressed(KeyCode::KeyA) {
        dir = Some(Vector2i::LEFT);
    }
    if context.input.is_key_pressed(KeyCode::KeyD) {
        dir = Some(Vector2i::RIGHT);
    }
    if context.input.is_key_pressed(KeyCode::KeyW) {
        dir = Some(Vector2i::UP);
    }
    if context.input.is_key_pressed(KeyCode::KeyS) {
        dir = Some(Vector2i::DOWN);
    }

    // let wait = context.input.is_key_released(KeyCode::Space);

    InputState {
        click,
        mouse_world_position: w,
        dir,
    }
}
