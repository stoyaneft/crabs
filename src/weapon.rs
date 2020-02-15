use crate::shot::{new_pistol_shot, new_bazooka_shot, Shot};
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

pub trait Fireable {
    fn fire(&self, _: Point2<f32>) -> Vec<Box<dyn Shot>>;
}

pub struct Skip {
    kind: WeaponType,
}
impl Fireable for Skip {
    fn fire(&self, _: Point2<f32>) -> Vec<Box<dyn Shot>> {
        vec![]
    }
}

pub struct Pistol {
    kind: WeaponType,
    direction: Vector2<f32>,
}

impl Fireable for Pistol {
    fn fire(&self, pos: Point2<f32>) -> Vec<Box<dyn Shot>> {
        println!("pistol firing");
        vec![Box::new(new_pistol_shot(
            Rect::new(pos.x, pos.y, 15.0, 12.0),
            self.direction,
        ))]
    }
}

pub struct Bazooka {
    kind: WeaponType,
    direction: Vector2<f32>,
}

impl Fireable for Bazooka {
    fn fire(&self, pos: Point2<f32>) -> Vec<Box<dyn Shot>> {
        println!("bazooka firing");
        Some(vec![Box::new(new_bazooka_shot(
            Rect::new(pos.x, pos.y, 20.0, 10.0),
            self.direction,
        ))])
    }
}

pub struct Weapon<T: Fireable> {
    weapon: T,
    kind: WeaponType,
    direction: Vector2<f32>,
}

impl<T: Fireable> Weapon<T> {
    fn new(&self, weapon: WeaponType) -> Weapon<T> {
        Weapon {
            kind: weapon,
            direction: Vector2::new(1.0, 0.0),
            weapon: Box<dyn T>::new(),
        }
    }

    fn kind(&self) -> WeaponType {
        self.kind
    }

    fn fire(&self, pos: Point2<f32>) -> Vec<Box<dyn Shot>> {
        self.weapon.fire(pos)
    }
}


pub struct Bazooka {
    kind: WeaponType,
    direction: Vector2<f32>,
}

impl Weapon for Bazooka {
    fn fire(&self, pos: Point2<f32>) -> Option<Vec<Box<dyn Shot>>> {
        println!("Bazooka firing");
        Some(vec![Box::new(new_bazooka_shot(
            Rect::new(pos.x, pos.y, 20.0, 10.0),
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
