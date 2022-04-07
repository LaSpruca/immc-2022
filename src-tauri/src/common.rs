use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Serialize, Deserialize)]
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

impl Point<isize> {
    pub const UP: Point<isize> = Point { x: 0, y: -1 };
    pub const DOWN: Point<isize> = Point { x: 0, y: 1 };
    pub const LEFT: Point<isize> = Point { x: -1, y: 0 };
    pub const RIGHT: Point<isize> = Point { x: 1, y: 0 };

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn pos(&self) -> Point<isize> {
        match self {
            Direction::Up => Point::UP,
            Direction::Down => Point::DOWN,
            Direction::Left => Point::LEFT,
            Direction::Right => Point::RIGHT,
        }
    }
}
