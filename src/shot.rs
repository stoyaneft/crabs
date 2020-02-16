use ggez::graphics::Rect;
use ggez::nalgebra::{Vector2, Point2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotType {
    Pistol,
    Bazooka,
}

#[derive(Clone)]
pub struct Shot {
    cfg: ShotConfig,
    rect: Rect,
    velocity: Vector2<f32>,
    kind: ShotType,
}

impl Shot {
    pub fn new(cfg: ShotConfig, kind: ShotType, pos: Point2<f32>, direction: Vector2<f32>) -> Self {
        Self {
            cfg,
            rect: Rect::new(pos.x, pos.y, cfg.width, cfg.height),
            velocity: cfg.speed * direction,
            kind,
        }
    }

    pub fn update(&mut self, seconds: f32) {
        self.rect.x += self.velocity.x * seconds;
        self.rect.y += self.velocity.y * seconds;
        self.velocity.y += self.cfg.mass * seconds;
    }

    pub fn damage(&self) -> f32 {
        self.cfg.damage
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn get_kind(&self) -> ShotType {
        self.kind
    }
}

#[derive(Clone, Copy)]
pub struct ShotConfig {
    pub speed: f32,
    pub damage: f32,
    pub mass: f32,
    pub width: f32,
    pub height: f32,
}

// struct ShotConfig {
//     pub SPEED: f32 = 250.0;
//     pub DAMAGE: f32 = 25.0;
//     pub MASS: f32 = 500.0;
//     pub WIDTH: f32 = 20.0;
//     pub HEIGHT: f32 = 10.0;
// }
