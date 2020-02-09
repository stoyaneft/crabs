use crate::crab::Crab;
use crate::shot::{Shot, ShotKind};
use crate::weapon::WeaponType;
use ggez::graphics::{self, DrawParam, Rect, Text};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub struct GUI {
    cfg: Config,
    map: graphics::Image,
    players: HashMap<String, Player>,
    weapons: WeaponsMenu,
    shots: ShotImages,
}

pub struct Config {
    pub images: ImagesConfig,
    pub players: Vec<PlayerConfig>,
}

pub struct ImagesConfig {
    pub map: String,
    pub weapons: String,
    pub shots: ShotsConfig,
}

pub struct ShotsConfig {
    pub pistol: String,
}

pub struct PlayerConfig {
    pub name: String,
    pub crab_image: String,
    pub crab_firing_image: String,
}

impl GUI {
    pub fn new(ctx: &mut Context, cfg: Config) -> GameResult<Self> {
        let map = graphics::Image::new(ctx, &cfg.images.map)?;
        let weapons = graphics::Image::new(ctx, &cfg.images.weapons)?;
        let mut players = HashMap::new();
        for player_cfg in cfg.players.iter() {
            let crab_image = graphics::Image::new(ctx, &player_cfg.crab_image)?;
            let crab_firing_image = graphics::Image::new(ctx, &player_cfg.crab_firing_image)?;
            players.insert(
                player_cfg.name.clone(),
                Player {
                    crab_image,
                    crab_firing_image,
                },
            );
        }
        let pistol = graphics::Image::new(ctx, &cfg.images.shots.pistol)?;
        Ok(GUI {
            cfg,
            map,
            players,
            weapons: WeaponsMenu {
                image: weapons,
                rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            },
            shots: ShotImages { pistol },
        })
    }

    pub fn draw_map(&self, ctx: &mut Context, rect: Rect) -> GameResult {
        let pos = Point2::new(rect.x, rect.y);
        graphics::draw(ctx, &self.map, DrawParam::default().dest(pos))
    }

    pub fn get_map(&self) -> &graphics::Image {
        &self.map
    }

    pub fn draw_crab(&self, ctx: &mut Context, player_name: &str, crab: &Crab) -> GameResult {
        let player = self.players.get(player_name).unwrap();
        let crab_rect = crab.get_rect();
        let scale = Vector2::new(
            crab_rect.w / player.crab_image.width() as f32,
            crab_rect.h / player.crab_image.height() as f32,
        );
        let rect = Rect::new(
            crab_rect.x,
            crab_rect.y,
            self.weapons.rect.w,
            self.weapons.rect.h,
        );
        match crab.weapon.kind() {
            WeaponType::None => graphics::draw(
                ctx,
                &player.crab_image,
                DrawParam::default().dest(rect.point()).scale(scale),
            ),
            _ => {
                graphics::draw(
                    ctx,
                    &player.crab_firing_image,
                    // TODO: same rect used for crab and firing_crab. Could be a problem for images
                    // with different dimentions.
                    DrawParam::default().dest(rect.point()).scale(scale),
                )?;
                self.draw_weapon(ctx, crab.weapon.kind(), rect)
            }
        }?;
        self.draw_health(ctx, crab)
    }

    fn draw_health(&self, ctx: &mut Context, crab: &Crab) -> GameResult {
        let health = Text::new(format!("{}", crab.get_health()));
        let rect = crab.get_rect();
        graphics::draw(
            ctx,
            &health,
            DrawParam::default().dest(Point2::new(
                rect.x + rect.w / 4.0,
                crab.get_rect().top() - 20.0,
            )),
        )
    }

    fn draw_weapon(&self, ctx: &mut Context, weapon: WeaponType, rect: Rect) -> GameResult {
        let (idx, _) = WEAPONS_MENU_ITEMS
            .iter()
            .enumerate()
            .find(|(_, w)| w.kind == weapon)
            .unwrap();
        self.draw_weapon_at_idx(ctx, idx as u8, rect, Vector2::new(0.5, 0.5))
    }

    pub fn draw_shot(&self, ctx: &mut Context, shot: &Box<dyn Shot>) -> GameResult {
        let image = match shot.kind() {
            ShotKind::Pistol => &self.shots.pistol,
            //            _ => not_implemented!(),
        };
        let rect = shot.get_rect();
        let scale = Vector2::new(
            rect.w / self.shots.pistol.width() as f32,
            rect.h / self.shots.pistol.height() as f32,
        );
        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest(shot.get_rect().point())
                .scale(scale),
        )
    }

    pub fn draw_rect(&self, ctx: &mut Context, rect: Rect) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, DrawParam::default())
    }

    pub fn init_weapons_menu(&mut self, rect: Rect) {
        self.weapons.rect = rect;
    }

    pub fn draw_weapons_menu(&self, ctx: &mut Context) -> GameResult {
        // Currently we draw only one fixed weapon.
        for idx in 0..WEAPONS_MENU_ITEMS.len() {
            self.draw_weapon_at_idx(
                ctx,
                idx as u8,
                Rect::new(
                    self.weapons.rect.x as f32 + idx as f32 * self.weapons.rect.w,
                    self.weapons.rect.y,
                    self.weapons.rect.w,
                    self.weapons.rect.h,
                ),
                Vector2::new(1.0, 1.0),
            )?;
        }
        Ok(())
    }

    fn draw_weapon_at_idx(
        &self,
        ctx: &mut Context,
        idx: u8,
        dest: Rect,
        scale: Vector2<f32>,
    ) -> GameResult {
        let weapon = &WEAPONS_MENU_ITEMS[idx as usize];
        graphics::draw(
            ctx,
            &self.weapons.image,
            DrawParam::default()
                .src(Rect::new(
                    weapon.image_pos.0 as f32 * WEAPONS_IMAGE_WIDTH
                        / self.weapons.image.width() as f32,
                    weapon.image_pos.1 as f32 * WEAPONS_IMAGE_HEIGHT
                        / self.weapons.image.height() as f32,
                    dest.w / self.weapons.image.width() as f32,
                    dest.h / self.weapons.image.height() as f32,
                ))
                .dest(Point2::new(dest.x, dest.y))
                .scale(scale),
        )
    }

    pub fn is_weapon_activated(&self, x: f32, y: f32) -> Option<WeaponType> {
        // Change when rows and columns are introduced.
        if y < self.weapons.rect.y || y > self.weapons.rect.y + self.weapons.rect.h {
            return None;
        }

        // Change when rows and columns are introduced.
        if x < self.weapons.rect.x
            || x > self.weapons.rect.x + WEAPONS_MENU_ITEMS.len() as f32 * self.weapons.rect.w
        {
            return None;
        }

        let idx = (x - self.weapons.rect.x) as usize / self.weapons.rect.w as usize;
        Some(WEAPONS_MENU_ITEMS[idx].kind)
    }
}

struct Player {
    crab_image: graphics::Image,
    crab_firing_image: graphics::Image,
}

struct WeaponInfo {
    kind: WeaponType,
    image_pos: (u8, u8),
}

struct WeaponsMenu {
    image: graphics::Image,
    rect: Rect,
}

struct ShotImages {
    pistol: graphics::Image,
}

static WEAPONS_MENU_ITEMS: &'static [WeaponInfo; 4] = &[
    WeaponInfo {
        kind: WeaponType::Grenade,
        image_pos: (0, 0),
    },
    WeaponInfo {
        kind: WeaponType::Bazooka,
        image_pos: (0, 2),
    },
    WeaponInfo {
        kind: WeaponType::Skip,
        image_pos: (1, 3),
    },
    WeaponInfo {
        kind: WeaponType::Pistol,
        image_pos: (0, 9),
    },
];
const WEAPONS_IMAGE_WIDTH: f32 = 32.0;
const WEAPONS_IMAGE_HEIGHT: f32 = 32.0;
