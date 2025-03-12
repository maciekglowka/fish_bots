use rogalik::prelude::*;

use game_logic::{globals::BOARD_SIZE, LogicState};

use super::BoardGraphics;
use crate::{
    globals::{ENTITY_Z, HOME_SPRITE, MAP_Z, TILE_HEIGHT, TILE_SIZE, WATER_SPRITE},
    utils::{entity_z, tile_to_world},
};

pub(crate) fn draw_tiles(state: &mut BoardGraphics, logic: &LogicState, context: &mut Context) {
    draw_map(logic, context);
}

fn draw_map(logic: &LogicState, context: &mut Context) {
    // draw water
    for y in 0..BOARD_SIZE as i32 {
        for x in 0..BOARD_SIZE as i32 {
            let _ = context.graphics.draw_atlas_sprite(
                "sprites",
                WATER_SPRITE,
                TILE_SIZE * Vector2i::new(x, y).as_f32(),
                MAP_Z,
                Vector2f::new(TILE_SIZE, TILE_HEIGHT),
                SpriteParams::default(),
            );
        }
    }

    // draw home
    let _ = context.graphics.draw_atlas_sprite(
        "sprites",
        HOME_SPRITE,
        tile_to_world(logic.world.home),
        entity_z(logic.world.home.y),
        Vector2f::new(TILE_SIZE, TILE_HEIGHT),
        SpriteParams::default(),
    );
}
