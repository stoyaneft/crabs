use crate::config::{PlayerConfig, Screen};
use crate::crab::Crab;
use crate::map::Map;
use crate::shot::Shot;
use crate::weapon::WeaponType;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;
use rand::{self, Rng};
use std::collections::HashSet;

pub struct Player {
    pub name: String,
    pub crabs: Vec<Crab>,
    active_crab_idx: usize,
}

impl Player {
    pub fn new(cfg: &PlayerConfig, screen: &Screen) -> Player {
        let mut crabs = vec![];
        let mut rng = rand::thread_rng();

        for i in 0..cfg.crabs_count {
            let crab = Crab::new(
                &format!("{}:{}", cfg.name, i),
                graphics::Rect::new(
                    rng.gen::<f32>() * screen.width - 1.0,
                    100.0,
                    cfg.crab.width as f32,
                    cfg.crab.height as f32,
                ),
            );
            crabs.push(crab);
        }

        Player {
            name: String::from(cfg.name),
            crabs,
            active_crab_idx: 0,
        }
    }

    pub fn update_crab(&mut self, direction: Vector2<f32>, seconds: f32, map: &Map) {
        if self.crabs.len() > 0 {
            self.crabs[self.active_crab_idx].update(direction, seconds, map)
        }
    }

    pub fn set_weapon(&mut self, weapon: WeaponType) {
        self.crabs[self.active_crab_idx].set_weapon(weapon)
    }

    pub fn fire(&mut self) -> Option<Vec<Box<dyn Shot>>> {
        self.crabs[self.active_crab_idx].fire()
    }

    pub fn kill_crab(&mut self, name: String) {
        self.crabs.retain(|crab| crab.name != name);
    }

    pub fn handle_collisions(&mut self, shot: Box<dyn Shot>, skip_active: bool) -> bool {
        let mut hit = false;
        let mut killed = HashSet::new();
        let active_crab = self.crabs[self.active_crab_idx].name.clone();
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
}
