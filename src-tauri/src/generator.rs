use crate::beano::Kind;
use crate::{
    beano::Beano,
    common::{Direction, Point},
    plane::{path_find_for_seat, Grid, GridTile},
};
use rand::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

pub fn generate_random(grid: Grid) -> HashMap<Point<usize>, Vec<Beano>> {
    let mut entrances = vec![];
    let mut seats = vec![];

    let mut rng = thread_rng();
    seats.shuffle(&mut rng);

    for (x, column) in grid.iter().enumerate() {
        for (y, tile) in column.iter().enumerate() {
            match tile {
                GridTile::Seat => seats.push(Point { x, y }),
                GridTile::Entry => entrances.push(Point { x, y }),
                _ => {}
            }
        }
    }

    let mut beanoz: HashMap<Point<usize>, Vec<Beano>> = HashMap::new();

    for pos in seats.iter() {
        let mut entrances = entrances
            .iter()
            .map(|entry| {
                (*entry, {
                    let entry: Point<isize> = (*entry).into();
                    let res = (entry - (*pos).into()).abs();
                    ((res.x as f64).powi(2) + (res.y as f64).powi(2)).sqrt()
                })
            })
            .collect::<Vec<_>>();

        entrances.sort_by(|(_, dist_a), (_, dist_b)| {
            if dist_a > dist_b {
                Ordering::Greater
            } else if dist_a < dist_b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        let entry = entrances[0];

        let path: Vec<Direction> = path_find_for_seat(&entry.0, pos, &grid).unwrap().into();
        let kind_rng = rng.next_u32() as f64 / u32::MAX as f64;

        let kind = if kind_rng < 0.8625 {
            Kind::Adult
        } else if kind_rng < 0.985 && kind_rng > 0.8625 {
            Kind::Kid
        } else {
            Kind::OtherSpecialistSupport
        };

        if beanoz.contains_key(&(entry.0)) {
            beanoz
                .get_mut(&(entry.0))
                .unwrap()
                .push(Beano::new(kind, *pos, path, 1, false));
        } else {
            beanoz.insert(entry.0, vec![Beano::new(kind, *pos, path, 1, false)]);
        }
    }

    beanoz
}
