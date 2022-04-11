mod error;

#[cfg(test)]
mod tests;

pub use error::Error;
use image::io::Reader as ImageReader;
use pathfinding::prelude::*;
use std::{collections::VecDeque, path::Path};

use crate::common::{Direction, Point};

pub type Grid = Vec<Vec<GridTile>>;

#[derive(Clone, Debug, PartialEq)]
pub enum GridTile {
    None,
    Walkway,
    Seat,
    Entry,
}

impl GridTile {
    pub fn is_walkable(&self) -> bool {
        self == &Self::Walkway || self == &Self::Entry
    }
}

pub fn path_find_for_seat(
    start: &Point<usize>,
    seat: &Point<usize>,
    grid: &Grid,
) -> Option<VecDeque<Direction>> {
    let width = grid.len() as isize;
    let height = grid[0].len() as isize;

    let Point { x: gx, y: gy }: Point<isize> = find_seat_entry(seat, grid).into();
    let goal = (gx, gy);

    let Point { x: sx, y: sy }: Point<isize> = (*start).into();

    let result = astar(
        &(sx, sy),
        |&(x, y)| {
            let mut positions = vec![];

            if x + 1 < width && grid[x as usize + 1][y as usize].is_walkable() {
                positions.push((x + 1, y));
            }

            if x >= 1 && grid[x as usize - 1][y as usize].is_walkable() {
                positions.push((x - 1, y));
            }

            if y + 1 < height && grid[x as usize][y as usize + 1].is_walkable() {
                positions.push((x, y + 1));
            }

            if y >= 1 && grid[x as usize][y as usize - 1].is_walkable() {
                positions.push((x, y - 1));
            }

            positions.into_iter().map(|x| (x, 1))
        },
        |&(x, y)| (x - sx).abs() + (y - sy).abs(),
        |&p| p == (goal.0, goal.1),
    )?;

    Some(
        result
            .0
            .iter()
            .skip(1)
            .zip(result.0.iter())
            .map(|(point2, point1)| {
                (
                    Point {
                        x: point1.0,
                        y: point1.1,
                    },
                    Point {
                        x: point2.0,
                        y: point2.1,
                    },
                )
            })
            .map(|(point1, point2)| {
                let dif = point2 - point1;

                if dif == Point::RIGHT {
                    Direction::Right
                } else if dif == Point::LEFT {
                    Direction::Left
                } else if dif == Point::UP {
                    Direction::Up
                } else if dif == Point::DOWN {
                    Direction::Down
                } else {
                    unreachable!("Pathfinding shat itself")
                }
            })
            .collect(),
    )
}

pub fn load_grid_from_img<T>(img: T) -> Result<Grid, Error>
where
    T: AsRef<Path>,
{
    let img = ImageReader::open(img)?.decode()?.to_rgb8();

    let (width, height) = img.dimensions();

    let mut grid = Vec::with_capacity(width as usize);

    for x in 0..width {
        let mut col = vec![GridTile::None; height as usize];
        for y in 0..height {
            match img.get_pixel(x, y).0 {
                [255, 255, 255] => col[y as usize] = GridTile::Walkway,
                [150, 150, 150] => col[y as usize] = GridTile::Seat,
                [0, 255, 0] => col[y as usize] = GridTile::Entry,
                _ => {}
            }
        }
        grid.push(col);
    }

    Ok(grid)
}

pub fn find_seat_entry(seat: &Point<usize>, grid: &Grid) -> Point<usize> {
    let width = grid.len();

    let Point { y: sy, x: sx } = *seat;

    if sy >= 1 {
        if let GridTile::Walkway | GridTile::Entry = grid[sx][sy - 1] {
            return Point { x: sx, y: sy - 1 };
        }
    }

    for i in 1..width as isize {
        if sx as isize - i >= 0 {
            if let GridTile::Walkway | GridTile::Entry = grid[sx - i as usize][sy] {
                return Point {
                    x: sx - i as usize,
                    y: sy,
                };
            }
        }

        if (sx + i as usize) < width {
            if let GridTile::Walkway | GridTile::Entry = grid[sx + i as usize][sy] {
                return Point {
                    x: sx + i as usize,
                    y: sy,
                };
            }
        }
    }

    panic!("WTF are you trying to do?")
}
