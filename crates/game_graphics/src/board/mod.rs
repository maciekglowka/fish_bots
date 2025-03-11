use rogalik::{events::SubscriberHandle, math::vectors::vector2::Vector2, prelude::*};

mod fish;
mod player;
mod tiles;

use crate::{
    draw::{bubbles::Bubble, sprites::Sprite},
    globals::{BASE_TEXT_SIZE, BLUE_COLOR, GAP, RED_COLOR, UI_Z},
    input::InputState,
    ui::{Button, Span},
    utils::{get_viewport_bounds, tile_to_world},
};
use game_logic::{LogicState, World};

#[derive(Default)]
pub struct BoardGraphics {
    pub reload: bool,
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
    fish::draw_fish(logic, context);
    draw_controls(state, input_state, context);
    draw_status(logic, context);

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

fn draw_controls(state: &mut BoardGraphics, input_state: &InputState, context: &mut Context) {
    let bounds = get_viewport_bounds(context);
    let button = Button::new(
        bounds.0 + Vector2f::splat(GAP),
        Vector2f::new(100., 2. * BASE_TEXT_SIZE),
        UI_Z,
    )
    .with_span(Span::new().with_text_borrowed("Reload"));
    button.draw(context, input_state);
    if button.clicked(input_state) {
        state.reload = true;
    }
}

fn draw_status(logic: &LogicState, context: &mut Context) {
    let bounds = get_viewport_bounds(context);
    let _ = context.graphics.draw_text(
        "default",
        &format!("Score: {}", logic.score),
        Vector2f::new(bounds.0.x + GAP, bounds.1.y - BASE_TEXT_SIZE - GAP),
        UI_Z,
        BASE_TEXT_SIZE,
        SpriteParams::default(),
    );
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
