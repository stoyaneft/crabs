use ggez;
use ggez::event;
use ggez::GameResult;

use std::env;
use std::path;

use crabs::config;
use crabs::game::Game;

const SCREEN_SIZE: (f32, f32) = (500.0, 300.0);

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("resources")
    };

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("crabs", "Stoyan Eftimov")
        .window_setup(ggez::conf::WindowSetup::default().title("Crabs!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;

    let config = config::GameConfig {
        crab: config::CrabConfig {
            image: String::from("/crab.png"),
            width: 48,
            height: 32,
        },
        map: config::MapConfig {
            image: String::from("/large-hill.png"),
        },
    };
    let game = &mut Game::new(ctx, config)?;
    event::run(ctx, event_loop, game)
}
