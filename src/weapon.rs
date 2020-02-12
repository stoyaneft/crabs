use crate::shot::{new_pistol_shot, Shot};
use ggez::graphics::Rect;
use ggez::nalgebra::{Point2, Vector2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WeaponType {
    None,
    Bazooka,
    Grenade,
    Pistol,
    Skip,
}

pub trait Weapon {
    fn fire(&self, _: Point2<f32>) -> Option<Vec<Box<dyn Shot>>>;
    fn kind(&self) -> WeaponType;
    fn direction(&self) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }
    fn set_direction(&mut self, _: Vector2<f32>) {}
}

pub fn new_weapon(weapon: WeaponType) -> Box<dyn Weapon> {
    match weapon {
        WeaponType::Skip => Box::new(Skip { kind: weapon }),
        WeaponType::Pistol => Box::new(Pistol {
            kind: weapon,
            direction: Vector2::new(1.0, 0.0),
        }),
        _ => Box::new(NoWeapon { kind: weapon }),
        // Weapon::Bazooka =>  {kind: weapon }
        // Weapon::None => NoWeapon {kind: weapon }
    }
}

pub struct NoWeapon {
    kind: WeaponType,
}
impl Weapon for NoWeapon {
    fn fire(&self, _: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("none firing");
        None
    }

    fn kind(&self) -> WeaponType {
        self.kind
    }
}

pub struct Skip {
    kind: WeaponType,
}
impl Weapon for Skip {
    fn fire(&self, _: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("skip firing");
        Some(vec![])
    }

    fn kind(&self) -> WeaponType {
        self.kind
    }
}

pub struct Pistol {
    kind: WeaponType,
    direction: Vector2<f32>,
}

impl Weapon for Pistol {
    fn fire(&self, pos: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("pistol firing");
        Some(vec![Box::new(new_pistol_shot(
            Rect::new(pos.x, pos.y, 15.0, 12.0),
            self.direction,
        ))])
    }

    fn kind(&self) -> WeaponType {
        self.kind
    }

    fn direction(&self) -> Vector2<f32> {
        self.direction
    }

    fn set_direction(&mut self, direction: Vector2<f32>) {
        self.direction = direction;
    }
}
