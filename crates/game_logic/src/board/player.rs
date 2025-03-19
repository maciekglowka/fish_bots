use rogalik::math::vectors::{Vector2i, ORTHO_DIRECTIONS};

use crate::{
    commands::{Command, MovePlayerCommand},
    scripting::get_player_action,
    world::{Player, World},
    LogicState,
};

pub(super) fn player_board_init(state: &mut LogicState) {
    state.world.players.clear();
    for i in 0..state.config.bot_count.min(3) {
        let v = state.world.home + ORTHO_DIRECTIONS[i];
        state.world.players.push(Player {
            v,
            ..Default::default()
        })
    }
}

pub(super) fn handle_player_turn(state: &mut crate::LogicState) {
    for idx in 0..state.world.players.len() {
        match get_player_action(&mut state.lua, idx as u32, &state.world) {
            Ok(action) => {
                match command_from_str(idx, &action) {
                    Some(command) => state.command_queue.push_back(vec![command]),
                    None => (), // TODO log wait
                }
            }
            Err(e) => {
                state
                    .console
                    .as_ref()
                    .expect("Console not found!")
                    .send(format!("Runtime error: {:?}", e));
            }
        }
    }
}

fn command_from_str(idx: usize, s: &str) -> Option<Box<dyn Command>> {
    match s {
        "left" => Some(Box::new(MovePlayerCommand {
            idx,
            dir: Vector2i::LEFT,
        })),
        "right" => Some(Box::new(MovePlayerCommand {
            idx,
            dir: Vector2i::RIGHT,
        })),
        "up" => Some(Box::new(MovePlayerCommand {
            idx,
            dir: Vector2i::UP,
        })),
        "down" => Some(Box::new(MovePlayerCommand {
            idx,
            dir: Vector2i::DOWN,
        })),
        _ => None,
    }
}
