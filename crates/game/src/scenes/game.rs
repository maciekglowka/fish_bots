use rogalik::engine::{Context, Scene, SceneChange};

use crate::GameState;

pub(crate) struct GameScene;
impl GameScene {
    fn init_game(&mut self, game: &mut GameState) {
        // game_logic::startup::init_game(&mut game.logic_state);
    }
}
impl Scene for GameScene {
    type Game = GameState;

    fn update(
        &mut self,
        _game: &mut Self::Game,
        _context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        Some(SceneChange::Push(Box::new(super::board::Board::default())))
    }
    fn enter(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        self.init_game(game);
    }
    fn restore(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        self.init_game(game);
    }
}
