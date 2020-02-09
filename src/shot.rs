use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotKind {
    Pistol,
}

pub trait Shot: ShotClone {
    fn kind(&self) -> ShotKind;
    fn update(&mut self, seconds: f32);
    fn damage(&self) -> f32;
    fn get_rect(&self) -> Rect;
}

pub trait ShotClone {
    fn clone_box(&self) -> Box<dyn Shot>;
}

impl<T> ShotClone for T
where
    T: 'static + Shot + Clone,
{
    fn clone_box(&self) -> Box<dyn Shot> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shot> {
    fn clone(&self) -> Box<dyn Shot> {
        self.clone_box()
    }
}

#[derive(Clone)]
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

    fn damage(&self) -> f32 {
        Self::DAMAGE
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
