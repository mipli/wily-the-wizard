use std::cmp::min;
use super::Point;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
            width: w,
            height: h
        }
    }
    pub fn center(&self) -> Point {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2).into()
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2) && (self.x2 >= other.x1) && (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }

    pub fn distance(&self, other: &Rect) -> i32 {
        if self.intersect(other) {
            0
        } else {
            if other.y1 >= self.y1 && other.y1 <= self.y2 {
                // horizontal overlap
                if other.x2 < self.x1 {
                    // to the left
                    self.x1 - other.x2
                } else {
                    // to the right
                    other.x1 - self.x2
                }
            } else if other.x1 >= self.x1 && other.x1 <= self.x2 {
                // vertical overlap
                if other.y2 < self.y1 {
                    // above
                    self.y1 - other.y2
                } else {
                    // beneath
                    other.y1 - self.y2
                }
            } else if self.x1 > other.x2 && self.y1 > other.y2 {
                // top left
                Point::new(self.x1, self.y1).tile_distance((other.x2, other.y2))
            } else if self.x1 > other.x2 && self.y1 > other.y2 {
                // bottom left
                Point::new(self.x2, self.y1).tile_distance((other.x1, other.y2))
            } else if other.x1 > self.x2 && self.y2 < other.y1 {
                // top right
                Point::new(self.x1, self.y2).tile_distance((other.x1, other.y2))
            } else if other.x1 > self.x2 && self.y2 < other.y1 {
                // bottom right
                Point::new(self.x2, self.y2).tile_distance((other.x1, other.y1))
            } else {
                unreachable!("Error calculating distance from {:?} - {:?}", self, other);
            }
        }
    }

    pub fn move_to(&mut self, point: Point) {
        *self = Rect::new(point.x, point.y, self.width, self.height);
    }

    pub fn grow(&mut self, delta: Point) -> Result<(), ()> {
        let width = self.width + delta.x;
        let height = self.height + delta.y;
        if width < 0 || height < 0 {
            return Err(());
        }
        self.width = width;
        self.height = height;
        self.x2 = self.x1 + self.width;
        self.y2 = self.y1 + self.height;
        Ok(())
    }
}
