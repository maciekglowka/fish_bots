use rogalik::prelude::*;
use std::collections::VecDeque;

use game_logic::World;

use crate::{
    globals::{
        BLUE_COLOR, DIGITS_TEXT_SIZE, MOVE_THRESH, PRIMARY_COLOR, RED_COLOR, SPRITE_SPEED,
        TILE_SIZE,
    },
    utils::{move_towards, tile_to_world},
};

#[derive(Default)]
pub(crate) struct Sprite {
    pub origin: Vector2f,
    pub frame: usize,
}
impl Sprite {
    pub fn update_position(&mut self, tile: Vector2i, delta: f32) -> bool {
        let target = tile_to_world(tile);
        if (target - self.origin).len_sq() < 0.1 {
            self.origin = target;
            return false;
        }

        self.origin = move_towards(self.origin, target, SPRITE_SPEED * delta);
        true
    }
}
