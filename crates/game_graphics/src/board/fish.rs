use rogalik::prelude::*;

use game_logic::LogicState;

use crate::{
    globals::{FISH_SPRITE, TILE_HEIGHT, TILE_SIZE},
    utils::{entity_z, tile_to_world},
};

pub(crate) fn draw_fish(logic: &LogicState, context: &mut Context) {
    for (k, v) in logic.world.fish.iter() {
        let idx = FISH_SPRITE - 1 + v.value as usize;
        let _ = context.graphics.draw_atlas_sprite(
            "sprites",
            idx,
            tile_to_world(*k),
            entity_z(k.y),
            Vector2f::new(TILE_SIZE, TILE_HEIGHT),
            SpriteParams::default(),
        );
    }
}
