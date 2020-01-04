use crate::config::PlayerConfig;
use crate::crab::Crab;
use ggez::graphics;
use rand::{self, Rng};

pub struct Player {
    pub name: String,
    pub crabs: Vec<Crab>,
    active_crab_idx: usize,
}

impl Player {
    pub fn new(cfg: &PlayerConfig) -> Player {
        let mut crabs = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..cfg.crabs_count {
            let crab = Crab::new(graphics::Rect::new(
                rng.gen::<f32>() * 500.0,
                100.0,
                cfg.crab.width as f32,
                cfg.crab.height as f32,
            ));
            crabs.push(crab);
        }

        Player {
            name: cfg.name.clone(),
            crabs,
            active_crab_idx: 0,
        }
    }

    pub fn active_crab(&mut self) -> &mut Crab {
        &mut self.crabs[self.active_crab_idx]
    }
}
