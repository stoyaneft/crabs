use crate::config::GameConfig;
use crate::gui::{self, GUI};
use crate::map::Map;
use crate::player::Player;
use ggez::input::mouse::MouseButton;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{event, timer};
use ggez::{graphics, Context, GameResult};

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    weapons_menu_open: bool,
}

pub struct Game {
    gui: GUI,
    map: Map,
    players: Vec<Player>,
    input: InputState,
    active_player_idx: usize,
}

impl Game {
    pub fn new(ctx: &mut Context, cfg: GameConfig) -> GameResult<Game> {
        let mut players = vec![];
        let mut players_cfg = vec![];
        for i in 0..cfg.players_count {
            let i = i as usize;
            let player = Player::new(&cfg.players[i]);
            players.push(player);
            players_cfg.push(gui::PlayerConfig {
                name: cfg.players[i].name.clone(),
                crab_image: cfg.players[i].crab.image.clone(),
                crab_firing_image: cfg.players[i].crab.image_firing.clone(),
            });
        }
        let mut gui = GUI::new(
            ctx,
            gui::Config {
                images: gui::ImagesConfig {
                    map: cfg.map.image,
                    weapons: cfg.weapons.image,
                },
                players: players_cfg,
            },
        )?;
        gui.init_weapons_menu(graphics::Rect::new(400.0, 250.0, 32.0, 32.0));
        let map = Map::new(ctx, gui.get_map())?;
        //        println!("map: {:?}", map);

        // Necessary for placing players on the ground.
        for player in players.iter_mut() {
            for crab in player.crabs.iter_mut() {
                crab.update(Vector2::new(0.0, 0.0), 0.0, &map);
            }
        }

        Ok(Self {
            gui,
            map,
            players,
            input: InputState::default(),
            active_player_idx: 0,
        })
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 30;
        let seconds = 1.0 / (FPS as f32);

        while timer::check_update_time(ctx, FPS) {
            for crab in self.players[self.active_player_idx].crabs.iter_mut() {
                crab.update(Vector2::new(self.input.movement, 0.0), seconds, &self.map);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        self.gui
            .draw_map(ctx, graphics::Rect::new(0.0, 0.0, 500.0, 300.0))?;

        for player in self.players.iter() {
            for crab in player.crabs.iter() {
                let rect = crab.get_rect();
                self.gui.draw_crab(ctx, &player.name, crab)?;
                self.gui.draw_rect(ctx, rect)?;
            }
        }

        if self.input.weapons_menu_open {
            self.gui.draw_weapons_menu(ctx)?;
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

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Right => {
                self.input.weapons_menu_open = !self.input.weapons_menu_open;
            }
            MouseButton::Left => match self.gui.is_weapon_activated(x, y) {
                None => (),
                Some(weapon) => self.players[self.active_player_idx]
                    .active_crab()
                    .set_weapon(weapon),
            },
            _ => (),
        }
    }
}
