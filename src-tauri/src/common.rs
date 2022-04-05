use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: std::fmt::Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: std::ops::Sub> std::ops::Sub for Point<T> {
    type Output = Point<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: std::ops::Add> std::ops::Add for Point<T> {
    type Output = Point<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Into<Point<isize>> for Point<usize> {
    fn into(self) -> Point<isize> {
        Point {
            x: self.x as isize,
            y: self.y as isize,
        }
    }
}

impl Into<Point<usize>> for Point<isize> {
    fn into(self) -> Point<usize> {
        Point {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
