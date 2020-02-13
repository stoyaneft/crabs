use crate::shot::Shot;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::fmt;

pub struct Map {
    mask: Vec<Vec<i8>>,
    width: u16,
    height: u16,
}

impl Map {
    pub fn new(ctx: &mut Context, image: &graphics::Image) -> GameResult<Self> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let data = image.to_rgba8(ctx)?;
        let alphas = data.iter().enumerate().filter(|(idx, _)| idx % 4 == 3);
        let mut mask: Vec<Vec<i8>> = vec![vec![0; width]; height];
        for (idx, (_, &val)) in alphas.enumerate() {
            let x = idx % width;
            let y = idx / width;
            if val > 0 {
                mask[y][x] = 1;
            }
        }
        Ok(Map {
            mask,
            width: width as u16,
            height: height as u16,
        })
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&i8> {
        self.mask.get(y)?.get(x)
    }

    pub fn on_ground(&self, pos: Point2<f32>) -> bool {
        if let Some(&land) = self.get(pos.x as usize, pos.y as usize) {
            return land == 1i8;
        }
        false
    }

    pub fn handle_collisions(&mut self, shot: Box<dyn Shot>) -> bool {
        let shot_rect = shot.get_rect();
        let hit_point = Point2::new(shot_rect.x, shot_rect.y);
        let hit = self.on_ground(hit_point);
        if hit {
            let r = shot.damage() as isize;
            for i in -r..r + 1 {
                for j in -r..r + 1 {
                    let p = Point2::new(hit_point.x + i as f32, hit_point.y + j as f32);
                    if ggez::nalgebra::distance(&hit_point, &p) <= r as f32 {
                        if self.on_ground(p) {
                            self.mask[p.y as usize][p.x as usize] = -1;
                        }
                    }
                }
            }
        }

        hit
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.mask.iter() {
            write!(f, "{:?}\n", v)?;
        }
        write!(f, "dimensions: {:?} x {:?}", self.width, self.height)
    }
}
