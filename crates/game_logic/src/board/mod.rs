use rogalik::prelude::*;

use crate::globals::BOARD_SIZE;

pub(crate) mod player;
mod systems;

use crate::LogicState;

pub fn board_init(state: &mut LogicState, code: String) {
    state.lua = piccolo::Lua::core();
    crate::scripting::init(&mut state.lua, code).unwrap();
    state.world = crate::World::default();
    state.score = 0;
    player::player_board_init(&mut state.world);
    state.world.home = Vector2i::new(BOARD_SIZE as i32 / 2, BOARD_SIZE as i32 / 2);
}

pub fn board_exit(state: &mut LogicState) {
    // player::player_board_exit(world);
    // board::clear_board(world);
}

pub fn board_update(state: &mut LogicState) {
    if handle_command_queue(state) {
        return;
    };
    next_turn(state);
}

fn next_turn(state: &mut LogicState) {
    player::handle_player_turn(state);
    // systems::end_turn_systems(state);
}

fn handle_command_queue(state: &mut LogicState) -> bool {
    let Some(commands) = state.command_queue.pop_front() else {
        return false;
    };
    for command in commands {
        if !command.is_valid(&state.world) {
            continue;
        };
        if let Ok(output) = command.execute(&mut state.world) {
            state.command_queue.push_front(output.commands);
            if let Some(event) = output.event {
                // state.world.resources.game_events.publish(event);
            }
            log::debug!("Executed command: {:?}", command);
        } else {
            log::debug!("Failed command: {:?}", command);
        }
    }
    true
}
