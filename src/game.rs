use crate::config::GameConfig;
use crate::crab::Crab;
use crate::gui::{self, GUI};
use crate::map::Map;
use ggez::nalgebra::Vector2;
use ggez::{event, timer};
use ggez::{graphics, Context, GameResult};

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
}

pub struct Game {
    gui: GUI,
    map: Map,
    crabs: Vec<Crab>,
    input: InputState,
}

impl Game {
    pub fn new(ctx: &mut Context, cfg: GameConfig) -> GameResult<Game> {
        let crab = Crab::new(graphics::Rect::new(
            50.0,
            100.0,
            cfg.crab.width as f32,
            cfg.crab.height as f32,
        ));
        let gui = GUI::new(
            ctx,
            gui::Config {
                images: gui::ImagesConfig {
                    crab: cfg.crab.image,
                    map: cfg.map.image,
                },
            },
        )?;
        let map = Map::new(ctx, gui.get_map())?;
        //        println!("map: {:?}", map);

        Ok(Self {
            gui,
            map,
            crabs: vec![crab],
            input: InputState::default(),
        })
    }
}

impl event::EventHandler for Game {
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
        self.gui
            .draw_map(ctx, graphics::Rect::new(0.0, 0.0, 500.0, 300.0))?;

        for crab in self.crabs.iter() {
            let rect = crab.get_rect();
            self.gui.draw_crab(ctx, rect)?;
            self.gui.draw_rect(ctx, rect)?;
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
            _ => (),
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
            _ => (),
        }
    }
}
