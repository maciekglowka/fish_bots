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

pub(super) fn handle_player_ui(
    world: &World,
    state: &mut super::BoardGraphics,
    context: &mut Context,
    input_state: &InputState,
) {
    handle_input_player(state, world, context, input_state);
}

fn handle_input_player(
    state: &super::BoardGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) {
    // let Some(entity) = get_player_entity(world) else {
    //     return;
    // };
    // let Some(position) = world.components.position.get(entity) else {
    //     return;
    // };
    // if let Some(dir) = input_state.dir {
    //     world
    //         .resources
    //         .input_events
    //         .publish(InputEvent::PlayerMove(*position + dir));
    // }
    // let targets = get_valid_player_targets(state.selected_attack, world);
    // draw_cursor(&targets, context);

    // if input_state.click != ButtonState::Released {
    //     return;
    // }
    // let target = world_to_tile(input_state.mouse_world_position);
    // if !targets.contains_key(&target) {
    //     return;
    // }

    // world
    //     .resources
    //     .input_events
    //     .publish(InputEvent::PlayerMove(target, state.selected_attack));
}

pub(super) fn draw_status(_world: &World, context: &mut Context) {
    let _bounds = get_viewport_bounds(context);
    // let _ = context.graphics.draw_text(
    //     "default",
    //     &format!("AP: {}", "*".repeat(world.resources.player.ap)),
    //     bounds.0 + Vector2f::splat(10.),
    //     UI_Z,
    //     16.,
    //     SpriteParams::default(),
    // );
}
