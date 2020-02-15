use crate::shot::{new_pistol_shot, new_bazooka_shot, Shot};
use ggez::nalgebra::{Point2, Vector2};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WeaponType {
    Bazooka,
    Pistol,
    Skip,
}

pub trait Fireable {
    fn fire(&self, _: Point2<f32>, _: Vector2<f32>) -> Vec<Box<dyn Shot>>;
}

pub struct Skip {}
impl Fireable for Skip {
    fn fire(&self, _: Point2<f32>, _: Vector2<f32>) -> Vec<Box<dyn Shot>> {
        vec![]
    }
}

pub struct Pistol {}
impl Fireable for Pistol {
    fn fire(&self, pos: Point2<f32>, d: Vector2<f32>) -> Vec<Box<dyn Shot>> {
        vec![Box::new(new_pistol_shot(pos, d))]
    }
}

pub struct Bazooka {}
impl Fireable for Bazooka {
    fn fire(&self, pos: Point2<f32>, d: Vector2<f32>) -> Vec<Box<dyn Shot>> {
        vec![Box::new(new_bazooka_shot(pos, d))]
    }
}

pub struct Weapon {
    weapon: Box<dyn Fireable>,
    kind: WeaponType,
    direction: Vector2<f32>,
}

impl Weapon {
    pub fn new(kind: WeaponType) -> Self {
        Weapon {
            kind,
            direction: Vector2::new(1.0, 0.0),
            weapon: new_weapon(kind),
        }
    }

    pub fn kind(&self) -> WeaponType {
        self.kind
    }

    pub fn fire(&self, pos: Point2<f32>) -> Vec<Box<dyn Shot>> {
        self.weapon.fire(pos, self.direction)
    }

    pub fn set_weapon(&mut self, weapon: Box<dyn Fireable>) {
        self.weapon = weapon
    }

    pub fn direction(&self) -> Vector2<f32> {
        self.direction
    }

    pub fn set_direction(&mut self, d: Vector2<f32>) {
        self.direction = d
    }
}

impl fmt::Debug for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "weapon: {:?}", self.kind)
    }
}

pub fn new_weapon(kind: WeaponType) -> Box<dyn Fireable> {
    match kind {
        WeaponType::Skip => Box::new(Skip{}),
        WeaponType::Bazooka =>  Box::new(Bazooka{}),
        WeaponType::Pistol => Box::new(Pistol{}),
    }
}
