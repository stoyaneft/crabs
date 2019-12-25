use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::fmt;

pub struct Map {
    mask: Vec<Vec<u8>>,
    width: u16,
    height: u16,
}

impl Map {
    pub fn new(ctx: &mut Context, image: &graphics::Image) -> GameResult<Self> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let data = image.to_rgba8(ctx)?;
        let alphas = data.iter().enumerate().filter(|(idx, _)| idx % 4 == 3);
        let mut mask: Vec<Vec<u8>> = vec![vec![0; width]; height];
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

    pub fn get(&self, x: usize, y: usize) -> Option<&u8> {
        return self.mask.get(y).and(self.mask[y].get(x));
    }

    pub fn on_ground(&self, pos: Point2<f32>) -> bool {
        if let Some(&land) = self.get(pos.x as usize, pos.y as usize) {
            return land > 0u8;
        }
        false
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
