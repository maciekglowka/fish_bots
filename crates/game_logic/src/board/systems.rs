use rand::prelude::*;
use rogalik::prelude::*;

use crate::{
    globals::{
        BOARD_SIZE, FISH_LIFE_MAX, FISH_LIFE_MIN, MAX_TURNS, SPAWN_INTERVAL_MAX, SPAWN_INTERVAL_MIN,
    },
    LogicState, World,
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

    let Some(v) = get_free_tile(&state.world) else {
        return;
    };
    let life = rng.gen_range(FISH_LIFE_MIN..=FISH_LIFE_MAX);
    state
        .world
        .fish
        .insert(v, crate::world::Fish { life, value: 1 });

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

fn get_free_tile(world: &World) -> Option<Vector2i> {
    let pool = (0..BOARD_SIZE)
        .map(|x| (0..BOARD_SIZE).map(move |y| Vector2i::new(x as i32, y as i32)))
        .flatten()
        .filter(|v| is_tile_free(*v, world))
        .collect::<Vec<_>>();
    let mut rng = thread_rng();
    pool.choose(&mut rng).copied()
}

fn is_tile_free(v: Vector2i, world: &World) -> bool {
    !world.fish.contains_key(&v) && v.manhattan(world.home) > 1
}
