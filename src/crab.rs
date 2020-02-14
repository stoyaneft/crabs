use crate::map::Map;
use crate::shot::Shot;
use crate::weapon::{new_weapon, Weapon, WeaponType};
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::nalgebra::{Point2, Vector1, Vector2};

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
        if map.on_ground(self.bottom_middle()) {
            //            self.velocity.y = 0.0;
            while map.on_ground(Point2::new(
                self.rect.x + self.rect.w / 2.0,
                self.rect.bottom() - steps - 1.0,
            )) {
                steps += 1.0;
            }
        } else {
            while !map.on_ground(self.bottom_middle()) && (!map.on_ground(self.bottom_left()))
                || !map.on_ground(self.bottom_right())
            {
                self.rect.y += 1.0;
            }
            //            self.velocity.y = Self::GRAVITY;
            direction.y = 1.0;
        }
        if steps < self.rect.h / 2.0 {
            self.rect.y -= steps;
        } else {
            if direction.x == 1.0 && map.on_ground(self.bottom_right()) {
                return;
            }
            if direction.x == -1.0 && map.on_ground(self.bottom_left()) {
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

    pub fn has_weapon(&self) -> bool {
        match self.weapon.kind() {
            WeaponType::None => false,
            _ => true,
        }
    }

    pub fn set_weapon_direction(&mut self, seconds: f32) {
        let rot = ggez::nalgebra::geometry::Rotation2::new(seconds * 1.0);
        self.weapon.set_direction(rot * self.weapon.direction());
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

    fn top_left(&self) -> Point2<f32> {
        Point2::new(self.rect.left(), self.rect.top())
    }

    fn bottom_left(&self) -> Point2<f32> {
        Point2::new(self.rect.left(), self.rect.bottom())
    }

    fn top_right(&self) -> Point2<f32> {
        Point2::new(self.rect.right(), self.rect.top())
    }

    fn bottom_right(&self) -> Point2<f32> {
        Point2::new(self.rect.right(), self.rect.bottom())
    }

    fn bottom_middle(&self) -> Point2<f32> {
        Point2::new(self.rect.left() + self.rect.w / 2.0, self.rect.bottom())
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
