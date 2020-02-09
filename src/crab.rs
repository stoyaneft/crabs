use crate::map::Map;
use crate::shot::Shot;
use crate::weapon::{new_weapon, Weapon, WeaponType};
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::nalgebra::{Point2, Vector2};

pub struct Crab {
    pub velocity: na::Vector2<f32>,
    pub weapon: Box<dyn Weapon>,
    pub name: String,
    rect: Rect,
    health: f32,
}

impl Crab {
    pub const SPEED: f32 = 250.0;
    pub const GRAVITY: f32 = 50.0;

    pub fn new(name: &str, rect: Rect) -> Self {
        Crab {
            rect,
            name: String::from(name),
            velocity: Vector2::new(Self::SPEED, 0.0),
            weapon: new_weapon(WeaponType::None),
            health: 100.0,
        }
    }

    pub fn update(&mut self, mut direction: Vector2<f32>, seconds: f32, map: &Map) {
        let mut steps: f32 = 0.0;
        if map.on_ground(Point2::new(
            self.rect.x + self.rect.w / 2.0,
            self.rect.y + self.rect.h,
        )) {
            //            self.velocity.y = 0.0;
            while map.on_ground(Point2::new(
                self.rect.x + self.rect.w / 2.0,
                self.rect.y + self.rect.h - steps - 1.0,
            )) {
                steps += 1.0;
            }
        } else {
            while !map.on_ground(Point2::new(
                self.rect.x + self.rect.w / 2.0,
                self.rect.y + self.rect.h,
            )) {
                self.rect.y += 1.0;
            }
            //            self.velocity.y = Self::GRAVITY;
            direction.y = 1.0;
        }
        if steps < self.rect.h / 2.0 {
            self.rect.y -= steps;
        } else {
            if direction.x == 1.0
                && map.on_ground(Point2::new(
                    self.rect.x + self.rect.w,
                    self.rect.y + self.rect.h,
                ))
            {
                return;
            }
            if direction.x == -1.0
                && map.on_ground(Point2::new(self.rect.x, self.rect.y + self.rect.h))
            {
                return;
            }
        }
        self.rect.x = na::clamp(
            self.rect.x + self.velocity.x * direction.x * seconds,
            0.0,
            map.get_width() as f32 - self.rect.w,
        );
        //        self.rect.y = self.rect.y + self.velocity.y * direction.y * seconds;

        //        println!("new pos: {:?}", self.rect);
        //        println!("new vel: {:?}", self.velocity);
        //        println!("new dir: {:?}", direction);
    }

    pub fn set_weapon(&mut self, weapon: WeaponType) {
        println!("weapon set: {:?}", weapon);
        self.weapon = new_weapon(weapon)
    }

    pub fn fire(&mut self) -> Option<Vec<Box<dyn Shot>>> {
        self.weapon.fire(Point2::new(self.rect.x, self.rect.y))
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn reduce_health(&mut self, damage: f32) {
        self.health -= damage;
    }
}

//impl fmt::Debug for Crab {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        for v in self.mask.iter() {
//            write!(f, "{:?}\n", v);
//        }
//        write!(f, "dimensions: {:?}", self.image.dimensions())
//    }
//}
