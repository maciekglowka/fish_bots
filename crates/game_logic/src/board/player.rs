use rogalik::math::vectors::Vector2i;

use crate::{
    commands::{Command, MovePlayerCommand},
    globals::BOARD_SIZE,
    scripting::get_player_action,
    world::{Player, World},
};

pub(super) fn player_board_init(world: &mut World) {
    world.players.clear();
    // TEMP
    world.players.push(Player {
        v: Vector2i::new(BOARD_SIZE as i32 / 2 as i32, BOARD_SIZE as i32 / 2 - 1),
        ..Default::default()
    })
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
            Err(e) => println!("{:?}", e),
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
