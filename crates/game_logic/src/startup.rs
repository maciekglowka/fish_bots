use anyhow::{anyhow, Result};
use piccolo::Lua;

use crate::World;

pub fn init_game(state: &mut crate::LogicState) {
    state.lua = Lua::full();
    crate::scripting::init(&mut state.lua).unwrap();
    state.world = World::default();
    state.score = 0;
}
