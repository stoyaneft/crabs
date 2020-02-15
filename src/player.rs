use crate::crab::Crab;
use crate::map::Map;
use crate::shot::Shot;
use crate::weapon::WeaponType;
use ggez::nalgebra::Vector2;
use std::collections::HashSet;

pub struct Player {
    pub name: String,
    pub crabs: Vec<Crab>,
    active_crab_idx: usize,
}

impl Player {
    pub fn new(name: &str, crabs: Vec<Crab>) -> Player {
        Player {
            name: String::from(name),
            crabs,
            active_crab_idx: 0,
        }
    }

    pub fn update_crab(&mut self, direction: Vector2<f32>, seconds: f32, map: &Map) {
        if self.crabs.len() > 0 {
            self.active_crab().update(direction, seconds, map)
        }
    }

    pub fn switch_crab(&mut self) {
        self.active_crab_idx = (self.active_crab_idx + 1) % self.crabs.len();
    }

    pub fn set_weapon(&mut self, weapon: WeaponType) {
        self.active_crab().set_weapon(weapon)
    }

    pub fn set_weapon_direction(&mut self, seconds: f32) {
        self.active_crab().set_weapon_direction(seconds)
    }

    pub fn has_weapon(&mut self) -> bool {
        self.active_crab().has_weapon()
    }

    pub fn fire(&mut self) -> Vec<Box<dyn Shot>> {
        self.active_crab().fire()
    }

    pub fn kill_crab(&mut self, name: String) {
        self.crabs.retain(|crab| crab.name != name);
    }

    pub fn handle_collisions(&mut self, shot: Box<dyn Shot>, skip_active: bool) -> bool {
        let mut hit = false;
        let mut killed = HashSet::new();
        let active_crab = self.active_crab().name.clone();
        self.crabs.iter_mut().for_each(|crab| {
            if skip_active && crab.name == active_crab {
                return;
            }
            if crab.get_rect().overlaps(&shot.get_rect()) {
                crab.reduce_health(shot.damage());
                hit = true;
                if crab.get_health() <= 0.0 {
                    killed.insert(crab.name.clone());
                }
            }
        });
        self.crabs.retain(|crab| !killed.contains(&crab.name));
        hit
    }

    pub fn total_health(&self) -> f32 {
        self.crabs.iter().map(|crab| crab.get_health()).sum()
    }

    pub fn is_crab_active(&self, name: &str) -> bool {
        self.crabs[self.active_crab_idx].name == name
    }

    fn active_crab(&mut self) -> &mut Crab {
        &mut self.crabs[self.active_crab_idx]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shot::new_pistol_shot;
    use ggez::nalgebra::Point2;
    use ggez::graphics::Rect;

    fn new_player() -> Player {
        let crabs = vec![
            Crab::new("gosho", Rect::new(2.0, 2.0, 2.0, 2.0)),
            Crab::new("pesho", Rect::new(50.0, 50.0, 1.0, 1.0)),
        ];
        Player::new("ivan", crabs)
    }

    fn new_shot(pos: Point2<f32>) -> Box<dyn Shot> {
        Box::new(new_pistol_shot(pos, Vector2::new(0.0, 0.0)))
    }

    #[test]
    fn player_new() {
        let mut player = new_player();
        assert_eq!(player.active_crab().name, "gosho");
        assert_eq!(player.name, "ivan");
        assert_eq!(player.crabs.len(), 2);
    }

    #[test]
    fn player_total_health() {
        assert_eq!(new_player().total_health(), 2.0 * Crab::HEALTH);
    }

    #[test]
    fn player_switch_crab() {
        let mut player = new_player();
        assert_eq!(player.active_crab().name, "gosho");
        player.switch_crab();
        assert_eq!(player.active_crab().name, "pesho");
        player.switch_crab();
        assert_eq!(player.active_crab().name, "gosho");
    }

    #[test]
    fn player_kill_crab() {
        let mut player = new_player();
        player.kill_crab("pesho".to_owned());
        assert_eq!(player.total_health(), Crab::HEALTH);
    }

    #[test]
    fn player_handle_collisions_no() {
        let mut player = new_player();
        assert_eq!(player.handle_collisions(new_shot(Point2::new(100.0, 100.0)), false), false);
        assert!(player.crabs[0].get_health() == Crab::HEALTH);
        assert!(player.crabs[1].get_health() == Crab::HEALTH);

        assert_eq!(player.handle_collisions(new_shot(Point2::new(2.0, 2.0)), true), false);
        assert!(player.crabs[0].get_health() == Crab::HEALTH);
        assert!(player.crabs[1].get_health() == Crab::HEALTH);
    }

    #[test]
    fn player_handle_collisions_overlapping() {
        let mut player = new_player();
        assert_eq!(player.handle_collisions(new_shot(Point2::new(3.0, 3.0)), false), true);
        assert!(player.crabs[0].get_health() < Crab::HEALTH);
        assert!(player.crabs[1].get_health() == Crab::HEALTH);
    }

    #[test]
    fn player_handle_collisions_kills() {
        let mut player = new_player();
        player.active_crab().reduce_health(Crab::HEALTH);
        assert_eq!(player.handle_collisions(new_shot(Point2::new(2.0, 2.0)), false), true);
        assert_eq!(player.crabs.len(), 1);
        assert_eq!(player.active_crab().name, "pesho")
    }
}
