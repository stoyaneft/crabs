use ggez::graphics::{self, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct GUI {
    cfg: Config,
    map: graphics::Image,
    crab: graphics::Image,
}

pub struct Config {
    pub images: ImagesConfig,
}

pub struct ImagesConfig {
    pub crab: String,
    pub map: String,
}

impl GUI {
    pub fn new(ctx: &mut Context, cfg: Config) -> GameResult<Self> {
        let map = graphics::Image::new(ctx, &cfg.images.map)?;
        let crab = graphics::Image::new(ctx, &cfg.images.crab)?;
        Ok(GUI { cfg, map, crab })
    }

    pub fn draw_map(&self, ctx: &mut Context, rect: Rect) -> GameResult {
        let pos = Point2::new(rect.x, rect.y);
        graphics::draw(ctx, &self.map, DrawParam::default().dest(pos))
    }

    pub fn get_map(&self) -> &graphics::Image {
        &self.map
    }

    pub fn draw_crab(&self, ctx: &mut Context, rect: Rect) -> GameResult {
        let scale = Vector2::new(
            rect.w / self.crab.width() as f32,
            rect.h / self.crab.height() as f32,
        );
        let pos = Point2::new(rect.x, rect.y);
        graphics::draw(ctx, &self.crab, DrawParam::default().dest(pos).scale(scale))
    }

    pub fn draw_rect(&mut self, ctx: &mut Context, rect: Rect) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, DrawParam::default())
    }
}
