use rogalik::prelude::*;
use std::collections::HashMap;

use game_logic::World;

use crate::{
    globals::{BOAT_LOADED_SPRITE, BOAT_SPRITE, ENTITY_Z, TILE_HEIGHT, TILE_SIZE},
    utils::entity_z,
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
        let idx = if player.loaded.is_some() {
            BOAT_LOADED_SPRITE
        } else {
            BOAT_SPRITE
        };
        let _ = context.graphics.draw_atlas_sprite(
            "sprites",
            idx,
            sprite.origin,
            entity_z(player.v.y),
            Vector2f::new(TILE_SIZE, TILE_HEIGHT),
            SpriteParams::default(),
        );
    }
    animating
}
