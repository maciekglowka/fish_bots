use rogalik::prelude::*;
use std::collections::HashMap;

use game_logic::World;

use crate::{
    globals::{BOAT_Z, GAP, RED_COLOR, TILE_SIZE},
    input::{ButtonState, InputState},
    ui::{Button, Span},
    utils::{get_viewport_bounds, move_towards, tile_to_world, world_to_tile},
};

pub(super) fn draw_player(
    world: &World,
    state: &mut super::BoardGraphics,
    context: &mut Context,
) -> bool {
    let delta = context.time.get_delta();
    let mut animating = false;
    for (sprite, player) in state.player_sprites.iter_mut().zip(&world.players) {
        animating |= sprite.update_position(player.v, delta);
        let color = if player.loaded.is_some() {
            Color(0, 255, 0, 255)
        } else {
            Color(255, 255, 255, 255)
        };
        let _ = context.graphics.draw_atlas_sprite(
            "sprites",
            942,
            sprite.origin,
            BOAT_Z,
            Vector2f::splat(TILE_SIZE),
            SpriteParams {
                color,
                ..Default::default()
            },
        );
    }
    animating
}
