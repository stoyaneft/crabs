use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotKind {
    Pistol,
}

pub trait Shot {
    fn kind(&self) -> ShotKind;
    fn update(&mut self, seconds: f32);
    fn damage(&self);
    fn get_rect(&self) -> Rect;
}

pub struct PistolShot {
    kind: ShotKind,
    rect: Rect,
    velocity: Vector2<f32>,
}

impl PistolShot {
    pub const SPEED: f32 = 250.0;
    pub const DAMAGE: f32 = 5.0;
}

pub fn new_pistol_shot(rect: Rect, direction: Vector2<f32>) -> PistolShot {
    PistolShot {
        kind: ShotKind::Pistol,
        rect,
        velocity: PistolShot::SPEED * direction,
    }
}

impl Shot for PistolShot {
    fn kind(&self) -> ShotKind {
        self.kind
    }

    fn update(&mut self, seconds: f32) {
        self.rect.x += self.velocity.x * seconds;
        self.rect.y += self.velocity.y * seconds;
        println!("shot updated: {:?}", self.rect)
    }

    fn damage(&self) {}

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
