use rogalik::{events::EventBus, math::vectors::Vector2i};

#[derive(Default)]
pub struct World {
    pub home: Vector2i,
    pub players: Vec<Player>,
}

pub struct Player {
    pub v: Vector2i,
}
