use rogalik::engine::log;
use rogalik::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod assets;
mod input;
mod scenes;
#[cfg(target_arch = "wasm32")]
mod web;

#[derive(Default)]
struct GameState {
    main_camera: ResourceId,
    logic_state: game_logic::LogicState,
}

impl Game for GameState {
    fn setup(&mut self, context: &mut Context) {
        assets::load_assets(context);
        self.main_camera = context.graphics.create_camera(1., get_camera_center());
        context
            .graphics
            .set_clear_color(game_graphics::globals::BACKGROUND_COLOR);
    }
    fn resize(&mut self, context: &mut rogalik::engine::Context) {
        let (w, h) = get_target_resolution(context);
        context.graphics.set_rendering_resolution(w, h);
    }
}

#[allow(dead_code)]
fn main() {
    env_logger::init();
    let engine = EngineBuilder::new()
        .with_title("RGLK".to_string())
        .resizable(true)
        .with_logical_size(800., 450.)
        .build(GameState::default(), Box::new(scenes::GameScene));
    engine.run();
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    console_log::init_with_level(log::Level::Info).expect("Can't init the logger!");
    let engine = EngineBuilder::new().build_wasm(GameState::default(), Box::new(scenes::GameScene));
    engine.run();
}

fn get_camera_center() -> Vector2f {
    0.5 * game_graphics::globals::TILE_SIZE
        * Vector2f::splat(game_logic::globals::BOARD_SIZE as f32)
}

fn get_target_resolution(context: &Context) -> (u32, u32) {
    let size = context.get_physical_size();

    let target_dim = game_graphics::globals::TILE_SIZE * (game_logic::globals::BOARD_SIZE as f32);
    let min_dim = size.y.min(size.x);
    let scale = (min_dim / target_dim).floor();
    // only even resolutions
    (
        (size.x / scale).floor() as u32 / 2 * 2,
        (size.y / scale).floor() as u32 / 2 * 2,
    )
}
