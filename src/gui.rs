use crate::crab::Crab;
use crate::game::{GameShot};
use crate::shot::{Shot, ShotKind};
use crate::weapon::WeaponType;
use ggez::graphics::{self, DrawParam, Rect, Text};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub struct GUI {
    map: graphics::Image,
    players: HashMap<&'static str, Player>,
    weapons: WeaponsMenu,
    shots: ShotImages,
    aim: ImageSettings,
    arrow: ImageSettings,
}

pub struct Config {
    pub images: ImagesConfig,
    pub players: Vec<PlayerConfig>,
}

pub struct ImagesConfig {
    pub map: &'static str,
    pub weapons: &'static str,
    pub shots: ShotsConfig,
    pub aim: ImageConfig,
    pub arrow: ImageConfig,
}

pub struct ImageConfig {
    pub image: &'static str,
    pub width: f32,
    pub height: f32,
}

pub struct ShotsConfig {
    pub pistol: &'static str,
    pub bazooka: &'static str,
}

pub struct PlayerConfig {
    pub name: &'static str,
    pub crab_image: &'static str,
    pub crab_firing_image: &'static str,
}

impl GUI {
    const AIM_DISTANCE: f32 = 50.0;
    const ARROW_DISTANCE: f32 = 20.0;
    const HEALTH_DISTANCE: f32 = 20.0;
    const WEAPONS_IMAGE_DISTANCE: f32 = 10.0;
    const WINNER_BANNER_DISTANCE: f32 = 30.0;
    const WEAPONS_IMAGE_WIDTH: f32 = 32.0;
    const WEAPONS_IMAGE_HEIGHT: f32 = 32.0;

    pub fn new(ctx: &mut Context, cfg: Config) -> GameResult<Self> {
        let map = graphics::Image::new(ctx, &cfg.images.map)?;
        let (map_width, map_height) = (map.width() as f32, map.height() as f32);
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
        let bazooka = graphics::Image::new(ctx, &cfg.images.shots.bazooka)?;
        let aim = graphics::Image::new(ctx, &cfg.images.aim.image)?;
        let arrow = graphics::Image::new(ctx, &cfg.images.arrow.image)?;
        Ok(GUI {
            map,
            players,
            weapons: WeaponsMenu {
                image: weapons,
                rect: Rect::new(
                    map_width - WEAPONS_MENU_ITEMS.len() as f32 * GUI::WEAPONS_IMAGE_WIDTH - GUI::WEAPONS_IMAGE_DISTANCE,
                    map_height - GUI::WEAPONS_IMAGE_HEIGHT - GUI::WEAPONS_IMAGE_DISTANCE,
                    GUI::WEAPONS_IMAGE_WIDTH,
                    GUI::WEAPONS_IMAGE_HEIGHT
                ),
            },
            shots: ShotImages { pistol, bazooka },
            aim: ImageSettings{
                image: aim,
                width: cfg.images.aim.width,
                height: cfg.images.aim.height,
            },
            arrow: ImageSettings{
                image: arrow,
                width: cfg.images.arrow.width,
                height: cfg.images.arrow.height,
            },
        })
    }

    pub fn draw_map(&self, ctx: &mut Context, rect: Rect) -> GameResult {
        let pos = Point2::new(rect.x, rect.y);
        let scale = Vector2::new(rect.w/self.map.width() as f32, rect.h/self.map.height() as f32);
        graphics::draw(ctx, &self.map, DrawParam::default().dest(pos).scale(scale))
    }

    pub fn get_map(&self) -> &graphics::Image {
        &self.map
    }

    pub fn draw_crab(&self, ctx: &mut Context, player_name: &str, crab: &Crab, is_active: bool) -> GameResult {
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
                    DrawParam::default().dest(rect.point()).scale(scale),
                )?;
                self.draw_weapon(ctx, crab.weapon.kind(), rect)?;
                if is_active && (crab.weapon.kind() == WeaponType::Pistol || crab.weapon.kind() == WeaponType::Bazooka) {
                    let d = crab.weapon.direction().scale(Self::AIM_DISTANCE);
                    let aim_dest = Point2::new(rect.x + d.x, rect.y + d.y);
                    self.draw_aim(ctx, aim_dest)?;
                }
                Ok(())
            }
        }?;
        if is_active {
            self.draw_arrow(ctx, Point2::new(crab_rect.x + crab_rect.w / 2.0, crab_rect.top() - Self::ARROW_DISTANCE))?;
        }
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
                crab.get_rect().top() - Self::HEALTH_DISTANCE,
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
            ShotKind::Bazooka => &self.shots.bazooka,
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

    pub fn draw_winner(&self, ctx: &mut Context, winner: &str) -> GameResult {
        let winner_banner = Text::new(format!("{} wins", winner));
        graphics::draw(
            ctx,
            &winner_banner,
            DrawParam::default().dest(Point2::new(self.map.width() as f32 / 2.0 - 20.0, Self::WINNER_BANNER_DISTANCE)),
        )
    }

    pub fn draw_weapons_menu(&self, ctx: &mut Context) -> GameResult {
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

    fn draw_aim(&self, ctx: &mut Context, dest: Point2<f32>) -> GameResult {
        let scale = Vector2::new(
            self.aim.width/self.aim.image.width() as f32,
            self.aim.height/self.aim.image.height() as f32,
        );
        graphics::draw(
            ctx,
            &self.aim.image,
            DrawParam::default()
                .dest(dest)
                .scale(scale),
        )
    }

    fn draw_arrow(&self, ctx: &mut Context, dest: Point2<f32>) -> GameResult {
        let scale = Vector2::new(
            self.arrow.width/self.arrow.image.width() as f32,
            self.arrow.height/self.arrow.image.height() as f32,
        );
        graphics::draw(
            ctx,
            &self.arrow.image,
            DrawParam::default()
                .dest(Point2::new(dest.x - self.arrow.width/2.0, dest.y - self.arrow.height as f32))
                .scale(scale),
        )
    }

    pub fn draw_map_hits(&self, ctx: &mut Context, shots: &Vec<GameShot>) -> GameResult {
        for shot in shots {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2::new(0.0, 0.0),
                shot.damage(),
                1.0,
                graphics::BLACK,
            )?;
            let rect = shot.get_rect();
            graphics::draw(
                ctx,
                &circle,
                DrawParam::default().dest(Point2::new(rect.x, rect.y)),
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
                    weapon.image_pos.0 as f32 * Self::WEAPONS_IMAGE_WIDTH
                        / self.weapons.image.width() as f32,
                    weapon.image_pos.1 as f32 * Self::WEAPONS_IMAGE_HEIGHT
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
    bazooka: graphics::Image,
}

struct ImageSettings {
    image: graphics::Image,
    width: f32,
    height: f32,
}

static WEAPONS_MENU_ITEMS: &'static [WeaponInfo; 3] = &[
    // WeaponInfo {
    //     kind: WeaponType::Grenade,
    //     image_pos: (0, 0),
    // },
    WeaponInfo {
        kind: WeaponType::Skip,
        image_pos: (1, 3),
    },
    WeaponInfo {
        kind: WeaponType::Bazooka,
        image_pos: (0, 2),
    },
    WeaponInfo {
        kind: WeaponType::Pistol,
        image_pos: (0, 9),
    },
];

