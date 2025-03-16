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

#[derive(Debug)]
pub struct Config {
    pub bot_count: usize,
    pub obstacles: bool,
    pub variable_fish_value: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            bot_count: 3,
            obstacles: false,
            variable_fish_value: false,
        }
    }
}

#[derive(Default)]
pub struct LogicState {
    config: Config,
    command_queue: VecDeque<Vec<Box<dyn Command>>>,
    spawn_timer: u32,
    fish_counter: u32,
    pub console: Option<console::Console>,
    pub done: bool,
    pub lua: piccolo::Lua,
    pub score: u32,
    pub turns: u32,
    pub world: World,
}
impl LogicState {
    pub fn reset(&mut self) {
        self.console = Some(crate::console::Console::new());
        self.world = crate::World::default();
        self.done = false;
        self.score = 0;
        self.turns = 0;
        self.fish_counter = 0;
        self.spawn_timer = 0;
    }
}
