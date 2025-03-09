use rogalik::{events::SubscriberHandle, prelude::*};

mod player;
mod tiles;

use crate::{
    draw::{bubbles::Bubble, sprites::Sprite},
    globals::{BLUE_COLOR, RED_COLOR},
    input::InputState,
    utils::tile_to_world,
};
use game_logic::{LogicState, World};

#[derive(Default)]
pub struct BoardGraphics {
    player_sprites: Vec<Sprite>,
    bubbles: Vec<Bubble>,
}

pub fn board_init(state: &mut BoardGraphics, logic: &LogicState) {
    init_sprites(state, &logic.world);
}

pub fn board_exit(state: &mut BoardGraphics, logic: &LogicState) {
    // if let Some(events) = state.game_events.take() {
    //     world.game_events.unsubscribe(events);
    // }
}

pub fn board_draw(
    state: &mut BoardGraphics,
    logic: &LogicState,
    context: &mut Context,
    input_state: &InputState,
) -> bool {
    tiles::draw_tiles(state, logic, context);

    let mut animating = false;
    animating |= player::draw_player(&logic.world, state, context);
    // let _ = handle_events(state, world);
    // player::draw_status(world, context);
    // for sprite in state.sprites.iter_mut() {
    //     is_animating |= draw_entity_sprite(sprite, world, context);
    // }
    // crate::draw::bubbles::update_bubbles(&mut state.bubbles, context);
    // if !is_animating {
    //     player::handle_player_ui(world, state, context, input_state);
    // }
    animating
}

fn init_sprites(state: &mut BoardGraphics, world: &World) {
    // player sprites
    state.player_sprites = world
        .players
        .iter()
        .map(|p| Sprite {
            origin: tile_to_world(p.v),
            ..Default::default()
        })
        .collect();
}

// fn spawn_entity_bubble(
//     color: Color,
//     text: Option<String>,
//     icon: Option<usize>,
//     state: &mut BoardGraphics,
// ) {
//     let Some(sprite) = get_entity_sprite(entity, &state.sprites) else {
//         return;
//     };
//     let mut rng = thread_rng();
//     let origin = sprite.origin
//         + Vector2f::new(
//             rng.gen_range(0f32..0.5 * SPRITE_SIZE),
//             rng.gen_range(0.75 * SPRITE_SIZE..SPRITE_SIZE),
//         );
//     state.bubbles.push(Bubble::new(origin, color, text, icon));
// }
