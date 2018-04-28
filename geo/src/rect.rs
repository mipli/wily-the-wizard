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
}
