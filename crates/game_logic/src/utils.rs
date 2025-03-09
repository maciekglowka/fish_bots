use rogalik::math::vectors::Vector2i;

use crate::globals::BOARD_SIZE;

pub(crate) fn is_on_board(v: Vector2i) -> bool {
    v.x >= 0 && v.y >= 0 && v.x < BOARD_SIZE as i32 && v.y < BOARD_SIZE as i32
}
