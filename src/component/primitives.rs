use core::f32;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub r: f32,
}

impl Rotation {
    pub fn new(r: f32) -> Self {
        Rotation { r }
    }
}

impl From<f32> for Rotation {
    fn from(val: f32) -> Self {
        Rotation::new(val)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Point {
    fn from(val: (f32, f32)) -> Self {
        Point::new(val.0, val.1)
    }
}

impl From<Point> for (f32, f32) {
    fn from(val: Point) -> Self {
        (val.x, val.y)
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn from_tuple(p: (f32, f32)) -> Self {
        Point { x: p.0, y: p.1 }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for Rotation {
    type Output = Rotation;

    fn add(self, rhs: Self) -> Self::Output {
        Rotation::new(self.r + rhs.r)
    }
}
