use crate::shot::{new_pistol_shot, Shot};
use ggez::graphics::Rect;
use ggez::nalgebra::{Point2, Vector2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Weapon {
    None,
    Bazooka,
    Granade,
    Pistol,
    Skip,
}

pub trait Fireable {
    fn fire(&self, _: Point2<f32>) -> Option<Vec<Box<dyn Shot>>>;
    fn kind(&self) -> Weapon;
}

pub fn New(weapon: Weapon) -> Box<dyn Fireable> {
    match weapon {
        Weapon::Skip => Box::new(Skip { kind: weapon }),
        Weapon::Pistol => Box::new(Pistol {
            kind: weapon,
            direction: Vector2::new(1.0, 0.0),
        }),
        _ => Box::new(NoWeapon { kind: weapon }),
        // Weapon::Bazooka =>  {type: weapon }
        // Weapon::None => NoWeapon {type: weapon }
    }
}

pub struct NoWeapon {
    kind: Weapon,
}
impl Fireable for NoWeapon {
    fn fire(&self, _: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("none firing");
        None
    }

    fn kind(&self) -> Weapon {
        self.kind
    }
}

pub struct Skip {
    kind: Weapon,
}
impl Fireable for Skip {
    fn fire(&self, _: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("skip firing");
        Some(vec![])
    }

    fn kind(&self) -> Weapon {
        self.kind
    }
}

pub struct Pistol {
    kind: Weapon,
    direction: Vector2<f32>,
}

impl Fireable for Pistol {
    fn fire(&self, pos: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("pistol firing");
        Some(vec![Box::new(new_pistol_shot(
            Rect::new(pos.x, pos.y, 15.0, 12.0),
            self.direction,
        ))])
    }

    fn kind(&self) -> Weapon {
        self.kind
    }
}
