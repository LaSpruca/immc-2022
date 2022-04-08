use crate::beano::{Beano, Beanoz, Kind};
use crate::common::{Direction, Point};
use crate::simulation::run_iteration;
use std::collections::HashMap;

#[test]
fn test_sim() {
    let mut spawner = HashMap::new();
    spawner.insert(
        Point { x: 2, y: 3 },
        vec![
            Beano::new(
                Kind::Adult,
                Point { x: 1, y: 2 },
                vec![Direction::Up],
                1,
                false,
            ),
            Beano::new(
                Kind::Adult,
                Point { x: 3, y: 2 },
                vec![Direction::Up],
                1,
                false,
            ),
            Beano::new(
                Kind::Adult,
                Point { x: 4, y: 2 },
                vec![Direction::Up],
                1,
                false,
            ),
            Beano::new(
                Kind::Adult,
                Point { x: 1, y: 2 },
                vec![Direction::Up, Direction::Up],
                1,
                false,
            ),
        ],
    );

    let mut beanoz = Beanoz::from(vec![]);

    while !beanoz.all_seated() {
        run_iteration(&mut beanoz, &mut spawner)
    }
}
