use crate::shot::Shot;
use ggez::nalgebra::Point2;
use std::fmt;

pub struct Map {
    mask: Vec<Vec<i8>>,
    width: u16,
    height: u16,
}

impl Map {
    pub fn new(data: &Vec<u8>, width: u16, height: u16) -> Map {
        let width = width as usize;
        let height = height as usize;
        let alphas = data.iter().enumerate().filter(|(idx, _)| idx % 4 == 3);
        let mut mask: Vec<Vec<i8>> = vec![vec![0; width]; height];
        for (idx, (_, &val)) in alphas.enumerate() {
            let x = idx % width;
            let y = idx / width;
            if val > 0 {
                mask[y][x] = 1;
            }
        }
        Map {
            mask,
            width: width as u16,
            height: height as u16,
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<i8> {
        self.mask.get(y)?.get(x).map(|v|*v)
    }

    pub fn on_ground(&self, pos: Point2<f32>) -> bool {
        if let Some(land) = self.get(pos.x.round() as usize, pos.y.round() as usize) {
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


#[cfg(test)]
mod tests {

    use super::*;
    use crate::shot::new_pistol_shot;
    use ggez::nalgebra::{Vector2};
    use ggez::graphics::Rect;

    fn new_map() -> Map {
        let x = vec![1, 1, 1, 1];
        let o = vec![0, 0, 0, 0];

        let image: Vec<&Vec<u8>> = vec![
            &o, &o, &o, &o,
            &o, &x, &x, &o,
        ];
        let data: Vec<u8> = image.iter().flat_map(|color| color.iter().cloned()).collect();
        Map::new(&data, 4, 2)
    }

    fn new_shot(rect: Rect) -> Box<dyn Shot> {
        Box::new(new_pistol_shot(rect, Vector2::new(0.0, 0.0)))
    }

    #[test]
    fn map_new() {
        let map = new_map();
        assert_eq!(map.get_width(), 4);
        assert_eq!(map.get_height(), 2);
        assert_eq!(map.get(0, 1), Some(0));
        assert_eq!(map.get(1, 1), Some(1));
        assert_eq!(map.get(2, 1), Some(1));
        assert_eq!(map.get(3, 1), Some(0));
        assert_eq!(map.get(4, 1), None);
    }

    #[test]
    fn map_on_ground() {
        let map = new_map();
        assert!(map.on_ground(Point2::new(0.0, 0.0)) == false);
        assert!(map.on_ground(Point2::new(1.0, 1.0)) == true);
        assert!(map.on_ground(Point2::new(2.0, 1.0)) == true);
        assert!(map.on_ground(Point2::new(-1.0, 1.0)) == false);
    }

    #[test]
    fn map_handle_collisions() {
        let mut map = new_map();
        assert!(map.handle_collisions(new_shot(Rect::new(0.0, 0.0, 1.0, 1.0))) == false);
        assert!(map.handle_collisions(new_shot(Rect::new(-1.0, 0.0, 1.0, 1.0))) == false);

        assert!(map.handle_collisions(new_shot(Rect::new(1.0, 1.0, 1.0, 1.0))) == true);
        assert_eq!(map.get(0, 1), Some(0));
        assert_eq!(map.get(1, 1), Some(-1));
        assert_eq!(map.get(2, 1), Some(-1));
        assert_eq!(map.get(3, 1), Some(0));
        assert_eq!(map.get(4, 1), None);
    }
}
