use rand::prelude::*;
use rogalik::prelude::*;

use crate::{
    globals::{BOARD_SIZE, SPAWN_INTERVAL},
    LogicState,
};

pub(super) fn end_turn_systems(state: &mut LogicState) {
    check_fish_deliver(state);
    handle_spawn(state);
    handle_fish_life(state);
    // handle_timed(state, world);
    // check_board_win(state, world);
    // check_gameover(state, world);
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
    if state.last_spawn > 0 {
        state.last_spawn -= 1;
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
        state
            .world
            .fish
            .insert(v, crate::world::Fish { life: 20, value: 1 });
        break;
    }

    state.last_spawn = SPAWN_INTERVAL;
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

// fn check_board_win(state: &mut BoardLogic, world: &World) {
//     // if query_iter!(world, With(collectible))
//     //     .filter(|(_, c)| **c == CollectibleKind::Key)
//     //     .collect::<Vec<_>>()
//     //     .is_empty()
//     // {
//     //     log::debug!("Board won!");
//     //     state.status = super::BattleStatus::Descend;
//     // }
// }

// fn check_gameover(state: &mut BoardLogic, world: &World) {
//     // if query!(world, With(player)).is_empty() {
//     //     state.status = super::BattleStatus::Gameover;
//     // }
// }
