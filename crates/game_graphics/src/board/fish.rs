use rogalik::prelude::*;

use game_logic::{globals::BOARD_SIZE, LogicState};

use crate::globals::{FISH_Z, TILE_SIZE};

pub(crate) fn draw_fish(logic: &LogicState, context: &mut Context) {
    for (k, v) in logic.world.fish.iter() {
        let _ = context.graphics.draw_atlas_sprite(
            "sprites",
            269,
            TILE_SIZE * k.as_f32(),
            FISH_Z,
            Vector2f::splat(TILE_SIZE),
            SpriteParams::default(),
        );
    }
}
