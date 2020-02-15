use crate::map::Map;
use crate::shot::Shot;
use crate::weapon::{Weapon, WeaponType, Skip, Pistol, Bazooka};
use ggez::graphics::Rect;
use ggez::nalgebra::{Point2, Vector2};

pub struct Crab {
    pub velocity: na::Vector2<f32>,
    pub weapon: WeaponType,
    pub weapon_direction: Vector2<f32>,
    pub name: String,
    rect: Rect,
    health: f32,
}

impl Crab {
    pub const SPEED: f32 = 250.0;
    pub const GRAVITY: f32 = 50.0;
    pub const HEALTH: f32 = 100.0;

    pub fn new(name: &str, rect: Rect) -> Self {
        Crab {
            rect,
            name: String::from(name),
            velocity: Vector2::new(Self::SPEED, 0.0),
            weapon: WeaponType::None,
            health: Self::HEALTH,
        }
    }

    pub fn update(&mut self, direction: Vector2<f32>, seconds: f32, map: &Map) {
        let old_x = self.rect.x;
        self.rect.x = na::clamp(
            self.rect.x + self.velocity.x * direction.x * seconds,
            0.0,
            map.get_width() as f32 - 1.0  - self.rect.w,
        );

        let mut steps: f32 = 0.0;
        // If crab is underground keep climbing up.
        while map.on_ground(Point2::new(
            self.rect.x + self.rect.w / 2.0,
            self.rect.bottom() - steps - 1.0,
        )) {
            println!("underground");
            steps += 1.0;
        }
        if 0.0 < self.rect.h && steps > self.rect.h {
            println!("cannot climb");
            self.rect.x = old_x;
        } else {
            println!("climbing");
            self.rect.y -= steps;
        }

        // If crab is above the ground fall to the ground.
        while !map.on_ground(self.bottom_middle()) &&
            (!map.on_ground(self.bottom_left()) || !map.on_ground(self.bottom_right())) &&
                self.rect.bottom() < map.get_height() as f32
            {
                self.rect.y += 1.0;
                println!("falling")
            }
    }

    pub fn set_weapon(&mut self, weapon: WeaponType) {
        println!("weapon set: {:?}", weapon);
        self.weapon = weapon
    }

    pub fn has_weapon(&self) -> bool {
       self.weapon != WeaponType::None
    }

    pub fn set_weapon_direction(&mut self, seconds: f32) {
        let rot = ggez::nalgebra::geometry::Rotation2::new(seconds * 1.0);
        self.weapon.set_direction(rot * self.weapon.direction());
    }

    pub fn fire(&mut self) -> Vec<Box<dyn Shot>> {
        // let weapon = match self.weapon {
        //     WeaponType::Skip => Weapon::new(self.weapon),
        //     WeaponType::Pistol => Some(Pistol {
        //         kind: weapon,
        //         direction: Vector2::new(1.0, 0.0),
        //     }),
        //     WeaponType::Bazooka => Some(Pistol {
        //         kind: weapon,
        //         direction: Vector2::new(1.0, 0.0),
        //     }),
        //     _ => None,
        // }
        let weapon = Weapon::new(self.weapon);
        weapon.fire(Point2::new(self.rect.x, self.rect.y))
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn get_pos(&self) -> Point2<f32> {
        Point2::new(self.rect.x, self.rect.y)
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn reduce_health(&mut self, damage: f32) {
        self.health -= damage;
    }

    #[allow(dead_code)]
    fn top_left(&self) -> Point2<f32> {
        Point2::new(self.rect.left(), self.rect.top())
    }

    fn bottom_left(&self) -> Point2<f32> {
        Point2::new(self.rect.left(), self.rect.bottom())
    }

    #[allow(dead_code)]
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


#[cfg(test)]
mod tests {

    use super::*;

    fn new_crab(rect: Rect) -> Crab {
        Crab::new("gosho", rect)
    }

    fn left() -> Vector2<f32> {
        Vector2::new(-1.0, 0.0)
    }

    fn right() -> Vector2<f32> {
        Vector2::new(1.0, 0.0)
    }

    fn none() -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }

    fn new_hills_map() -> Map {
        let x = vec![1, 1, 1, 1];
        let o = vec![0, 0, 0, 0];

        let image: Vec<&Vec<u8>> = vec![
            &o, &o, &o, &o,
            &o, &x, &x, &o,
            &x, &x, &x, &x,
        ];
        let data: Vec<u8> = image.iter().flat_map(|color| color.iter().cloned()).collect();
        Map::new(&data, 4, 3)
    }

    fn new_large_hill() -> Map {
        let x = vec![1, 1, 1, 1];
        let o = vec![0, 0, 0, 0];

        let image: Vec<&Vec<u8>> = vec![
            &o, &o, &o,
            &o, &o, &x,
            &o, &o, &x,
            &x, &x, &x,
        ];
        let data: Vec<u8> = image.iter().flat_map(|color| color.iter().cloned()).collect();
        Map::new(&data, 3, 4)
    }

    fn new_flat_map() -> Map {
        let x = vec![1, 1, 1, 1];
        let o = vec![0, 0, 0, 0];

        let image: Vec<&Vec<u8>> = vec![
            &o, &o, &o,
            &x, &x, &x,
        ];
        let data: Vec<u8> = image.iter().flat_map(|color| color.iter().cloned()).collect();
        Map::new(&data, 3, 2)
    }

    #[test]
    fn crab_new() {
        let crab = new_crab(Rect::default());
        assert_eq!(crab.name, "gosho");
        assert_eq!(crab.health, Crab::HEALTH);
        assert_eq!(crab.rect, Rect::default());
        assert_eq!(crab.velocity, Vector2::new(Crab::SPEED, 0.0));
        assert_eq!(crab.weapon.kind(), WeaponType::None);
    }

    #[test]
    fn crab_walk_horizontally() {
        let map = new_flat_map();
        let mut crab = new_crab(Rect::new(0.0, 0.0, 0.0, 0.0));
        crab.velocity = Vector2::new(1.0, 0.0);
        let seconds = 1.0;

        crab.update(none(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(0.0, 1.0));

        crab.update(left(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(0.0, 1.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(1.0, 1.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(2.0, 1.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(2.0, 1.0));
    }

    #[test]
    fn crab_walks_hills() {
        let map = new_hills_map();
        let mut crab = new_crab(Rect::new(0.0, 0.0, 0.0, 0.0));
        crab.velocity = Vector2::new(1.0, 0.0);
        let seconds = 1.0;

        crab.update(none(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(0.0, 2.0));

        crab.update(left(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(0.0, 2.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(1.0, 1.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(2.0, 1.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(3.0, 2.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(3.0, 2.0));
    }

    #[test]
    fn crab_cant_walk_too_large_hills() {
        let map = new_large_hill();
        let mut crab = new_crab(Rect::new(0.0, 0.0, 1.0, 1.0));
        crab.velocity = Vector2::new(1.0, 0.0);
        let seconds = 1.0;

        crab.update(none(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(0.0, 2.0));

        crab.update(right(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(0.0, 2.0));
    }

    #[test]
    fn crab_climbs_above_the_ground() {
        let map = new_large_hill();
        let mut crab = new_crab(Rect::new(2.0, 3.0, 0.0, 0.0));
        crab.velocity = Vector2::new(1.0, 0.0);
        let seconds = 1.0;

        crab.update(none(), seconds, &map);
        assert_eq!(crab.get_pos(), Point2::new(2.0, 1.0));
    }

    #[test]
    fn crab_loses_health() {
        let mut crab = new_crab(Rect::default());
        crab.reduce_health(10.0);
        assert_eq!(crab.get_health(), Crab::HEALTH - 10.0);
    }
}

