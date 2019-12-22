use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::fmt;

pub struct Map {
    image: graphics::Image,
    mask: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(ctx: &mut Context, path: String) -> GameResult<Self> {
        let image = graphics::Image::new(ctx, path)?;
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
        Ok(Map { image, mask })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.image,
            DrawParam::default()
                //            .src(graphics::Rect::new_i32(0, 2000, 1200, 700))
                .dest(Point2::new(0.0, 0.0)),
        )
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&u8> {
        return self.mask.get(y).and(self.mask[y].get(x));
    }

    pub fn dimensions(&self) -> graphics::Rect {
        self.image.dimensions()
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
        write!(f, "dimensions: {:?}", self.image.dimensions())
    }
}
