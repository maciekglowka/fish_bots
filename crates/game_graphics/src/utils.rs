use crate::globals::TILE_SIZE;
use rogalik::prelude::*;

pub(crate) fn get_viewport_bounds(context: &Context) -> (Vector2f, Vector2f) {
    let camera = context.graphics.get_current_camera();
    camera.get_bounds()
}

// pub(crate) fn is_vertical(bounds: &(Vector2f, Vector2f)) -> bool {
//     bounds.1.x - bounds.0.x < bounds.1.y - bounds.0.y
// }

pub(crate) fn move_towards(origin: Vector2f, target: Vector2f, max_delta: f32) -> Vector2f {
    let a = target - origin;
    let l = a.len();
    if l <= max_delta || l == 0. {
        return target;
    }
    origin + a / l * max_delta
}

pub(super) fn tile_to_world(v: Vector2i) -> Vector2f {
    TILE_SIZE * v.as_f32()
}

pub(super) fn world_to_tile(v: Vector2f) -> Vector2i {
    Vector2i::new(
        (v.x / TILE_SIZE).floor() as i32,
        (v.y / TILE_SIZE).floor() as i32,
    )
}

pub(super) fn entity_z(y: i32) -> i32 {
    crate::globals::ENTITY_Z - y
}
