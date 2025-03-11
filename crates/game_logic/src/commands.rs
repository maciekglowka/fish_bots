use anyhow::{anyhow, Context, Result};
use rogalik::math::vectors::{Vector2i, ORTHO_DIRECTIONS};

use crate::{events::GameEvent, utils::is_on_board, World};

pub trait Command: core::fmt::Debug {
    fn execute(&self, world: &mut World) -> Result<CommandOutput>;
    fn is_valid(&self, _world: &World) -> bool {
        true
    }
}

#[derive(Default)]
pub struct CommandOutput {
    pub event: Option<GameEvent>,
    pub commands: Vec<Box<dyn Command>>,
}
impl CommandOutput {
    pub fn new(event: Option<GameEvent>, commands: Vec<Box<dyn Command>>) -> Self {
        Self { event, commands }
    }
    pub fn empty() -> Result<Self> {
        Ok(Self::default())
    }
    pub fn event(event: GameEvent) -> Result<Self> {
        Ok(Self {
            event: Some(event),
            commands: Vec::new(),
        })
    }
    pub fn command(command: Box<dyn Command>) -> Result<Self> {
        Ok(Self {
            event: None,
            commands: vec![command],
        })
    }
}

#[derive(Debug)]
pub(crate) struct MovePlayerCommand {
    pub idx: usize,
    pub dir: Vector2i,
}
impl Command for MovePlayerCommand {
    fn execute(&self, world: &mut World) -> Result<CommandOutput> {
        world.players[self.idx].v += self.dir;

        let pick = PickFishCommand {
            player_idx: self.idx,
            target: world.players[self.idx].v,
        };
        if pick.is_valid(world) {
            return CommandOutput::command(Box::new(pick));
        }

        CommandOutput::empty()
    }
    fn is_valid(&self, world: &World) -> bool {
        let Some(p) = world.players.get(self.idx) else {
            return false;
        };
        is_on_board(p.v + self.dir)
    }
}

#[derive(Debug)]
pub(crate) struct PickFishCommand {
    pub player_idx: usize,
    pub target: Vector2i,
}
impl Command for PickFishCommand {
    fn execute(&self, world: &mut World) -> Result<CommandOutput> {
        let fish = world.fish.remove(&self.target).context("")?;
        world.players[self.player_idx].loaded = Some(fish);
        CommandOutput::empty()
    }
    fn is_valid(&self, world: &World) -> bool {
        world.fish.contains_key(&self.target) && world.players.len() > self.player_idx
    }
}
