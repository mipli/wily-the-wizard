use std::cmp::{Ordering, max};
use std::fmt::{self, Display, Error, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn from_i32(x: i32) -> Self {
        Point::new(x, x)
    }

    pub fn distance<P: Into<Point>>(&self, other: P) -> f32 {
        let other = other.into();
        let a = (self.x - other.x).pow(2);
        let b = (self.y - other.y).pow(2);
        ((a + b) as f32).sqrt()
    }

    pub fn tile_distance<P: Into<Point>>(&self, other: P) -> i32 {
        let other = other.into();
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    pub fn tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction_to<P: Into<Point>>(&self, other: P) -> (i32, i32) {
        let other = other.into();
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dist = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        let dx = (dx as f32 / dist).round() as i32;
        let dy = (dy as f32 / dist).round() as i32;
        (dx, dy)
    }
}

impl Into<Point> for (i32, i32) {
    fn into(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}

impl Into<Point> for (usize, usize) {
    fn into(self) -> Point {
        Point {
            x: self.0 as i32,
            y: self.1 as i32,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Ord for Point {
    fn cmp(&self, _other: &Point) -> Ordering {
        // NOTE: I don't know that's the difference between this one
        // and the more explicit fn below. So let's just crash here
        // and see if and when we ever hit this.
        unimplemented!();
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, _other: &Point) -> Option<Ordering> {
        // NOTE: I don't know that's the difference between this one
        // and the more explicit fn below. So let's just crash here
        // and see if and when we ever hit this.
        unimplemented!();
    }

    fn lt(&self, other: &Point) -> bool {
        self.x < other.x && self.y < other.y
    }

    fn le(&self, other: &Point) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    fn gt(&self, other: &Point) -> bool {
        self.x > other.x && self.y > other.y
    }

    fn ge(&self, other: &Point) -> bool {
        self.x >= other.x && self.y >= other.y
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self {
        let rhs: Point = rhs.into();
        self + rhs
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        let rhs: Point = rhs.into();
        *self = self.add(rhs);
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self {
        let rhs: Point = rhs.into();
        self - rhs
    }
}

impl PartialEq<(i32, i32)> for Point {
    fn eq(&self, other: &(i32, i32)) -> bool {
        let other: Point = (*other).into();
        self == &other
    }
}

impl PartialOrd<(i32, i32)> for Point {
    fn partial_cmp(&self, other: &(i32, i32)) -> Option<Ordering> {
        let other: Point = (*other).into();
        self.partial_cmp(&other)
    }

    fn lt(&self, other: &(i32, i32)) -> bool {
        let other: Point = (*other).into();
        *self < other
    }

    fn le(&self, other: &(i32, i32)) -> bool {
        let other: Point = (*other).into();
        *self <= other
    }

    fn gt(&self, other: &(i32, i32)) -> bool {
        let other: Point = (*other).into();
        *self > other
    }

    fn ge(&self, other: &(i32, i32)) -> bool {
        let other: Point = (*other).into();
        *self >= other
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Point::new(self.x / rhs, self.y / rhs)
    }
}


#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h
        }
    }
    pub fn center(&self) -> Point {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2).into()
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2) && (self.x2 >= other.x1) && (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }
}
