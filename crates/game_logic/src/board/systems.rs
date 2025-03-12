use rand::prelude::*;
use rogalik::prelude::*;

use crate::{
    globals::{
        BOARD_SIZE, FISH_LIFE_MAX, FISH_LIFE_MIN, MAX_TURNS, SPAWN_INTERVAL_MAX, SPAWN_INTERVAL_MIN,
    },
    LogicState,
};

pub(super) fn end_turn_systems(state: &mut LogicState) {
    check_fish_deliver(state);
    handle_spawn(state);
    handle_fish_life(state);
    state.turns += 1;
    check_gameover(state);
}

fn check_fish_deliver(state: &mut LogicState) {
    for i in 0..state.world.players.len() {
        if state.world.players[i].v != state.world.home {
            continue;
        }
        if let Some(loaded) = state.world.players[i].loaded.take() {
            state.score += loaded.value;
        }
    }
}

fn handle_spawn(state: &mut LogicState) {
    if state.spawn_counter > 0 {
        state.spawn_counter -= 1;
        return;
    }

    let mut rng = thread_rng();

    loop {
        let v = Vector2i::new(
            rng.gen_range(0..BOARD_SIZE) as i32,
            rng.gen_range(0..BOARD_SIZE) as i32,
        );
        if state.world.fish.contains_key(&v) {
            continue;
        }

        // TODO move to a command
        let life = rng.gen_range(FISH_LIFE_MIN..=FISH_LIFE_MAX);
        state
            .world
            .fish
            .insert(v, crate::world::Fish { life, value: 1 });
        break;
    }

    state.spawn_counter = rng.gen_range(SPAWN_INTERVAL_MIN..=SPAWN_INTERVAL_MAX);
}

fn handle_fish_life(state: &mut LogicState) {
    let mut to_remove = Vec::new();
    for (k, v) in state.world.fish.iter_mut() {
        if v.life == 0 {
            to_remove.push(*k);
            continue;
        }
        v.life -= 1;
    }

    for v in to_remove {
        state.world.fish.remove(&v);
    }
}

fn check_gameover(state: &mut LogicState) {
    if state.turns >= MAX_TURNS {
        state.done = true;
    }
}
