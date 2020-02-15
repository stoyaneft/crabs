use crate::config::GameConfig;
use crate::gui::{self, GUI};
use crate::map::Map;
use crate::player::Player;
use crate::shot::{Shot, ShotKind};
use ggez::graphics::Rect;
use ggez::input::mouse::MouseButton;
use ggez::nalgebra::{Vector2};
use ggez::{event, timer};
use ggez::{graphics, Context, GameResult};

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    weapons_menu_open: bool,
    weapon_direction: f32,
}

pub struct Game {
    cfg: &'static GameConfig,
    gui: GUI,
    map: Map,
    players: Vec<Player>,
    input: InputState,
    active_player_idx: usize,
    shots: Vec<GameShot>,
    shooting_in_progress: bool,
    winner: String,
    map_hits: Vec<GameShot>,
}

impl Game {
    pub fn new(ctx: &mut Context, cfg: &'static GameConfig) -> GameResult<Game> {
        let mut players = vec![];
        let mut players_cfg = vec![];
        for i in 0..cfg.players_count {
            let i = i as usize;
            let player = Player::new(&cfg.players[i], &cfg.screen);
            players.push(player);
            players_cfg.push(gui::PlayerConfig {
                name: cfg.players[i].name,
                crab_image: cfg.players[i].crab.image,
                crab_firing_image: cfg.players[i].crab.image_firing,
            });
        }
        let mut gui = GUI::new(
            ctx,
            gui::Config {
                images: gui::ImagesConfig {
                    map: cfg.map.image,
                    weapons: cfg.weapons.image,
                    shots: gui::ShotsConfig {
                        pistol: cfg.shots.pistol.image,
                    },
                },
                players: players_cfg,
            },
        )?;
        gui.init_weapons_menu(graphics::Rect::new(350.0, 250.0, 32.0, 32.0));
        let map_image = gui.get_map();
        let data = map_image.to_rgba8(ctx)?;
        let map = Map::new(&data, map_image.width() as u16, map_image.height() as u16);
        //        println!("map: {:?}", map);

        // Necessary for placing players on the ground.
        for player in players.iter_mut() {
            for crab in player.crabs.iter_mut() {
                crab.update(Vector2::new(0.0, 0.0), 0.0, &map);
            }
        }

        Ok(Self {
            cfg,
            gui,
            map,
            players,
            input: InputState::default(),
            active_player_idx: 0,
            shots: vec![],
            shooting_in_progress: false,
            winner: String::new(),
            map_hits: vec![],
        })
    }
}

impl Game {
    fn spawn_shots(&mut self, shots: Vec<Box<dyn Shot>>) {
        self.shots = shots
            .iter()
            .map(|shot| GameShot {
                is_alive: true,
                shot: shot.clone(),
            })
            .collect();
        self.shooting_in_progress = true;
    }

    fn switch_turn(&mut self) {
        self.active_player_idx = (self.active_player_idx + 1) % self.players.len()
    }

    fn handle_collisions(&mut self) {
        for shot in &mut self.shots {
            //            self.players[self.active_player_idx].handle_collisions(Box::new(shot.clone()), true);
            for (i, player) in self.players.iter_mut().enumerate() {
                let player_hit =
                    player.handle_collisions(Box::new(shot.clone()), i == self.active_player_idx);
                let map_hit = self.map.handle_collisions(Box::new(shot.clone()));
                if map_hit {
                    self.map_hits.push(shot.clone());
                    //                    self.map_hits.push(Point2::new(100.0, 100.0));
                }
                if player_hit || map_hit {
                    shot.is_alive = false;
                }
            }
        }
    }

    fn is_outside(rect: Rect, width: f32, height: f32) -> bool {
        rect.top() < 0.0 || rect.left() < 0.0 || rect.bottom() > height || rect.right() > width
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self
            .players
            .iter()
            .any(|player| player.total_health() <= 0.0)
        {
            let player = self
                .players
                .iter()
                .find(|p| p.total_health() > 0.0)
                .unwrap();
            self.winner = player.name.clone();
            return Ok(());
        }

        const FPS: u32 = 30;
        let seconds = 1.0 / (FPS as f32);

        while timer::check_update_time(ctx, FPS) {
            self.players[self.active_player_idx].update_crab(
                Vector2::new(self.input.movement, 0.0),
                seconds,
                &self.map,
            );

            self.players[self.active_player_idx]
                .set_weapon_direction(self.input.weapon_direction * seconds);

            for shot in self.shots.iter_mut() {
                shot.update(seconds);
            }

            if self.shooting_in_progress && self.shots.len() == 0 {
                self.switch_turn();
                self.shooting_in_progress = false;
            }

            self.handle_collisions();
            //            println!("map hits: {:?}", self.map_hits);

            let width = self.cfg.screen.width;
            let height = self.cfg.screen.height;
            self.shots
                .retain(|shot| !Self::is_outside(shot.get_rect(), width, height) && shot.is_alive);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        self.gui.draw_map(
            ctx,
            graphics::Rect::new(0.0, 0.0, self.cfg.screen.width, self.cfg.screen.height),
        )?;
        self.gui.draw_map_hits(ctx, &self.map_hits)?;

        for player in self.players.iter() {
            for crab in player.crabs.iter() {
                let rect = crab.get_rect();
                self.gui.draw_crab(ctx, &player.name, crab)?;
                self.gui.draw_rect(ctx, rect)?;
            }
        }

        for shot in self.shots.iter() {
            self.gui.draw_shot(ctx, &shot.shot)?;
        }

        if self.input.weapons_menu_open {
            self.gui.draw_weapons_menu(ctx)?;
        }

        if self.winner.len() > 0 {
            self.gui.draw_winner(ctx, &self.winner)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Left => self.input.movement = -1.0,
            event::KeyCode::Right => self.input.movement = 1.0,
            event::KeyCode::Up => self.input.weapon_direction = -1.0,
            event::KeyCode::Down => self.input.weapon_direction = 1.0,
            event::KeyCode::Escape => event::quit(ctx),
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
            event::KeyCode::Up | event::KeyCode::Down => self.input.weapon_direction = 0.0,
            event::KeyCode::Space => {
                if !self.shooting_in_progress {
                    match self.players[self.active_player_idx].fire() {
                        None => return,
                        Some(shots) => self.spawn_shots(shots),
                    }
                }
            }
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Right => {
                self.input.weapons_menu_open = !self.input.weapons_menu_open;
            }
            MouseButton::Left => {
                if self.input.weapons_menu_open {
                    match self.gui.is_weapon_activated(x, y) {
                        None => (),
                        Some(weapon) => self.players[self.active_player_idx].set_weapon(weapon),
                    }
                }
            }
            _ => (),
        }
    }
}

#[derive(Clone)]
pub struct GameShot {
    shot: Box<dyn Shot>,
    is_alive: bool,
}

impl Shot for GameShot {
    fn kind(&self) -> ShotKind {
        self.shot.kind()
    }

    fn update(&mut self, seconds: f32) {
        self.shot.update(seconds)
    }

    fn damage(&self) -> f32 {
        self.shot.damage()
    }

    fn get_rect(&self) -> Rect {
        self.shot.get_rect()
    }
}
