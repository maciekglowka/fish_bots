use rogalik::prelude::*;

use crate::World;

// pub(super) fn pick_collectibles(state: &mut super::BattleLogic, world: &mut World) {
//     let Some(player) = crate::player::get_player_entity(world) else {
//         return;
//     };
//     let Some(player_position) = world.components.position.get(player) else {
//         return;
//     };
//     query_execute!(
//         world,
//         With(collectible, position),
//         |entity, _, position: &Vector2i| {
//             if *player_position == *position {
//                 state
//                     .command_queue
//                     .push_back(vec![Box::new(PickCollectible(entity))]);
//             }
//         }
//     );
// }

// pub(super) fn end_turn_systems(state: &mut BoardLogic, world: &mut World) {
//     // handle_timed(state, world);
//     // check_board_win(state, world);
//     // check_gameover(state, world);
// }

// fn check_board_win(state: &mut BoardLogic, world: &World) {
//     // if query_iter!(world, With(collectible))
//     //     .filter(|(_, c)| **c == CollectibleKind::Key)
//     //     .collect::<Vec<_>>()
//     //     .is_empty()
//     // {
//     //     log::debug!("Board won!");
//     //     state.status = super::BattleStatus::Descend;
//     // }
// }

// fn check_gameover(state: &mut BoardLogic, world: &World) {
//     // if query!(world, With(player)).is_empty() {
//     //     state.status = super::BattleStatus::Gameover;
//     // }
// }
