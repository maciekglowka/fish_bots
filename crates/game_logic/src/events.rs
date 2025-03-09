use rogalik::math::vectors::Vector2i;

#[derive(Clone, Copy, Debug)]
pub enum GameEvent {
    SpawnFish(Vector2i),
    // Spawn(Entity),
    // Despawn(Entity),
    // Move(Entity),
    // Explode(Vector2i),
}
