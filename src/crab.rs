use crate::map::Map;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct Crab {
    image: graphics::Image,
    pos: na::Vector2<f32>,
    velocity: na::Vector2<f32>,
}

impl Crab {
    pub const SPEED: f32 = 250.0;
    pub const GRAVITY: f32 = 50.0;
    //    pub const

    pub fn new(ctx: &mut Context, path: String, pos: na::Point2<f32>) -> GameResult<Self> {
        let image = graphics::Image::new(ctx, path)?;
        Ok(Crab {
            image,
            pos: Vector2::new(pos.x, pos.y),
            velocity: Vector2::new(Self::SPEED, 0.0),
        })
    }

    pub fn update(&mut self, mut direction: Vector2<f32>, seconds: f32, map: &Map) {
        let rect = self.bounding_rect();
        let mut steps: f32 = 0.0;
        if map.on_ground(Point2::new(self.pos.x + rect.w / 2.0, self.pos.y + rect.h)) {
            //            self.velocity.y = 0.0;
            while map.on_ground(Point2::new(
                self.pos.x + rect.w / 2.0,
                self.pos.y + rect.h - steps - 1.0,
            )) {
                steps += 1.0;
            }
        } else {
            while !map.on_ground(Point2::new(self.pos.x + rect.w / 2.0, self.pos.y + rect.h)) {
                self.pos.y += 1.0;
            }
            //            self.velocity.y = Self::GRAVITY;
            direction.y = 1.0;
        }
        if steps < self.image.height() as f32 / 2.0 {
            self.pos.y -= steps;
        } else {
            if direction.x == 1.0
                && map.on_ground(Point2::new(self.pos.x + rect.w, self.pos.y + rect.h))
            {
                return;
            }
            if direction.x == -1.0 && map.on_ground(Point2::new(self.pos.x, self.pos.y + rect.h)) {
                return;
            }
        }
        self.pos.x = na::clamp(
            self.pos.x + self.velocity.x * direction.x * seconds,
            0.0,
            map.dimensions().w - rect.w,
        );
        //        self.pos.y = self.pos.y + self.velocity.y * direction.y * seconds;

        //        println!("new pos: {:?}", self.pos);
        //        println!("new vel: {:?}", self.velocity);
        //        println!("new dir: {:?}", direction);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.image,
            DrawParam::default()
                //            .src(graphics::Rect::new_i32(0, 2000, 1200, 700))
                .dest(na::Point2::new(self.pos.x, self.pos.y)),
        )
    }

    pub fn draw_bbox(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            self.bounding_rect(),
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, DrawParam::default())
    }

    pub fn bounding_rect(&self) -> graphics::Rect {
        let left = self.pos.x;
        let right = self.pos.x + self.image.width() as f32;
        let top = self.pos.y;
        let bottom = self.pos.y + self.image.height() as f32;

        graphics::Rect::new(left, top, right - left, bottom - top)
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
