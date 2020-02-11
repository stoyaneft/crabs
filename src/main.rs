use ggez;
use ggez::event;
use ggez::GameResult;

use std::env;
use std::path;

use crabs::config::CONFIG;
use crabs::game::Game;

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
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(CONFIG.screen.width, CONFIG.screen.height),
        )
        .add_resource_path(resource_dir)
        .build()?;

    let game = &mut Game::new(ctx, &CONFIG)?;
    event::run(ctx, event_loop, game)
}
