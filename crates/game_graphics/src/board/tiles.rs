use rogalik::prelude::*;

use game_logic::{globals::BOARD_SIZE, LogicState};

use super::BoardGraphics;
use crate::globals::{FIXTURE_Z, MAP_Z, TILE_SIZE};

pub(crate) fn draw_tiles(state: &mut BoardGraphics, logic: &LogicState, context: &mut Context) {
    draw_map(logic, context);
}

fn draw_map(logic: &LogicState, context: &mut Context) {
    // draw water
    for y in 0..BOARD_SIZE as i32 {
        for x in 0..BOARD_SIZE as i32 {
            let _ = context.graphics.draw_atlas_sprite(
                "sprites",
                253,
                TILE_SIZE * Vector2i::new(x, y).as_f32(),
                MAP_Z,
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    }

    // draw home
    let _ = context.graphics.draw_atlas_sprite(
        "sprites",
        509,
        TILE_SIZE * logic.world.home.as_f32(),
        FIXTURE_Z,
        Vector2f::splat(TILE_SIZE),
        SpriteParams::default(),
    );
}
