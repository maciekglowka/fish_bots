use std::collections::VecDeque;

pub mod board;
mod commands;
mod console;
mod events;
pub mod globals;
mod scripting;
mod utils;
mod world;

pub use events::GameEvent;
pub use world::World;

use commands::Command;

#[derive(Default)]
pub struct LogicState {
    command_queue: VecDeque<Vec<Box<dyn Command>>>,
    pub console: Option<console::Console>,
    pub lua: piccolo::Lua,
    pub score: u32,
    pub world: World,
}
