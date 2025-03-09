use rogalik::math::vectors::{Vector2f, Vector2i};

#[derive(Default)]
pub struct InputState {
    pub dir: Option<Vector2i>,
    pub mouse_world_position: Vector2f,
    pub click: ButtonState,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released,
}
