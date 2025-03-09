use rogalik::prelude::*;

use super::Span;
use crate::{
    globals::{PRIMARY_COLOR, RED_COLOR, TILE_SIZE},
    input::{ButtonState, InputState},
};

#[derive(Clone)]
pub struct Button<'a> {
    pub origin: Vector2f,
    size: Vector2f,
    z: i32,
    sprite_atlas: &'a str,
    sprite_index: usize,
    span: Option<Span<'a>>,
    slice: Option<(usize, Vector2f)>,
}
impl<'a> Button<'a> {
    pub fn new(origin: Vector2f, size: Vector2f, z: i32) -> Self {
        Button {
            origin,
            size,
            z,
            sprite_atlas: "sprites",
            sprite_index: 725,
            span: None,
            slice: Some((4, Vector2f::splat(TILE_SIZE))),
        }
    }
    pub fn with_span(mut self, span: Span<'a>) -> Self {
        self.span = Some(span);
        self
    }
    pub fn with_sprite(mut self, atlas: &'a str, index: usize) -> Self {
        self.sprite_atlas = atlas;
        self.sprite_index = index;
        self
    }
    // pub fn with_slice(mut self, slice: (usize, Vector2f)) -> Self {
    //     self.slice = Some(slice);
    //     self
    // }
    pub fn draw(&self, context: &mut Context, state: &InputState) {
        let _ = context.graphics.draw_atlas_sprite(
            self.sprite_atlas,
            self.sprite_index,
            self.origin,
            self.z,
            self.size,
            SpriteParams {
                slice: self.slice,
                color: if self.pressed(state) {
                    RED_COLOR
                } else {
                    PRIMARY_COLOR
                },
                ..Default::default()
            },
        );
        if let Some(span) = &self.span {
            let span_offset = Vector2f::new(
                0.5 * (self.size.x - span.width(context)),
                self.size.y - 0.5 * (self.size.y - span.height() as f32),
            );
            span.draw(self.origin + span_offset, self.z + 1, context);
        }
    }

    pub fn clicked(&self, state: &InputState) -> bool {
        state.click == ButtonState::Released && self.mouse_over(state)
    }

    fn pressed(&self, state: &InputState) -> bool {
        state.click == ButtonState::Down && self.mouse_over(state)
    }

    fn mouse_over(&self, state: &InputState) -> bool {
        let v = state.mouse_world_position;
        v.x >= self.origin.x
            && v.y >= self.origin.y
            && v.x <= self.origin.x + self.size.x
            && v.y <= self.origin.y + self.size.y
    }
}
