use ggez;
use ggez::graphics;
use ggez::{event, timer};
use ggez::{Context, GameResult};

use std::env;
use std::path;

use crabs::crab::Crab;
use crabs::map::Map;
use ggez::nalgebra::{Point2, Vector2};

const SCREEN_SIZE: (f32, f32) = (500.0, 300.0);

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
}

struct MainState {
    map: Map,
    crabs: Vec<Crab>,
    input: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let map = Map::new(ctx, String::from("/steep-hill.png"))?;
        //        println!("map: {:?}", map);
        println!("{:?}", graphics::drawable_size(ctx));
        let crab = Crab::new(ctx, String::from("/crab.png"), Point2::new(50.0, 100.0))?;
        Ok(MainState {
            map,
            crabs: vec![crab],
            input: InputState::default(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 30;
        let seconds = 1.0 / (FPS as f32);

        while timer::check_update_time(ctx, FPS) {
            self.crabs[0].update(Vector2::new(self.input.movement, 0.0), seconds, &self.map);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        self.map.draw(ctx).unwrap();

        for crab in self.crabs.iter() {
            crab.draw(ctx).unwrap();
            //            crab.draw_bbox(ctx).unwrap();
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Left => self.input.movement = -1.0,
            event::KeyCode::Right => self.input.movement = 1.0,
            //            event::KeyCode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
        match keycode {
            event::KeyCode::Left | event::KeyCode::Right => self.input.movement = 0.0,
            _ => (), // Do nothing
        }
    }
}

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

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
