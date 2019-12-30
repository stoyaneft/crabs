use crate::crab::Crab;
use crate::weapon::Weapon;
use ggez::graphics::{self, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct GUI {
    cfg: Config,
    map: graphics::Image,
    crab: graphics::Image,
    crab_firing: graphics::Image,
    weapons: WeaponsMenu,
}

pub struct Config {
    pub images: ImagesConfig,
}

pub struct ImagesConfig {
    pub crab: String,
    pub crab_firing: String,
    pub map: String,
    pub weapons: String,
}

impl GUI {
    pub fn new(ctx: &mut Context, cfg: Config) -> GameResult<Self> {
        let map = graphics::Image::new(ctx, &cfg.images.map)?;
        let crab = graphics::Image::new(ctx, &cfg.images.crab)?;
        let crab_firing = graphics::Image::new(ctx, &cfg.images.crab_firing)?;
        let weapons = graphics::Image::new(ctx, &cfg.images.weapons)?;
        Ok(GUI {
            cfg,
            map,
            crab,
            crab_firing,
            weapons: WeaponsMenu {
                image: weapons,
                rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            },
        })
    }

    pub fn draw_map(&self, ctx: &mut Context, rect: Rect) -> GameResult {
        let pos = Point2::new(rect.x, rect.y);
        graphics::draw(ctx, &self.map, DrawParam::default().dest(pos))
    }

    pub fn get_map(&self) -> &graphics::Image {
        &self.map
    }

    pub fn draw_crab(&self, ctx: &mut Context, crab: &Crab) -> GameResult {
        let scale = Vector2::new(
            crab.rect.w / self.crab.width() as f32,
            crab.rect.h / self.crab.height() as f32,
        );
        let rect = Rect::new(
            crab.rect.x,
            crab.rect.y,
            self.weapons.rect.w,
            self.weapons.rect.h,
        );
        match crab.weapon {
            Weapon::None => graphics::draw(
                ctx,
                &self.crab,
                DrawParam::default().dest(rect.point()).scale(scale),
            ),
            _ => {
                graphics::draw(
                    ctx,
                    &self.crab_firing,
                    DrawParam::default().dest(rect.point()).scale(scale),
                )?;
                self.draw_weapon(ctx, crab.weapon, rect)
            }
        }
    }

    fn draw_weapon(&self, ctx: &mut Context, weapon: Weapon, rect: Rect) -> GameResult {
        let (idx, _) = WEAPONS_MENU_ITEMS
            .iter()
            .enumerate()
            .find(|(_, w)| w.kind == weapon)
            .unwrap();
        self.draw_weapon_at_idx(ctx, idx as u8, rect, Vector2::new(0.5, 0.5))
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

    pub fn is_weapon_activated(&self, x: f32, y: f32) -> Option<Weapon> {
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

struct WeaponInfo {
    kind: Weapon,
    image_pos: (u8, u8),
}

struct WeaponsMenu {
    image: graphics::Image,
    rect: Rect,
}

static WEAPONS_MENU_ITEMS: &'static [WeaponInfo; 2] = &[
    WeaponInfo {
        kind: Weapon::Granade,
        image_pos: (0, 0),
    },
    WeaponInfo {
        kind: Weapon::Bazooka,
        image_pos: (0, 2),
    },
];
const WEAPONS_IMAGE_WIDTH: f32 = 32.0;
const WEAPONS_IMAGE_HEIGHT: f32 = 32.0;
