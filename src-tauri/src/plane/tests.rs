use super::{GridTile::*, *};
use crate::common::Direction::*;

#[test]
fn test_load() {
    let loaded = load_grid_from_img("./beno express.png").unwrap();
    let expected = vec![
        vec![
            None, None, None, None, None, Entry, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, None, None, None, Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, None, None, Walkway, Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, None, Walkway, Walkway, Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway,
            Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway,
            Walkway,
        ],
        vec![
            Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway,
            Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway, Walkway,
            Walkway,
        ],
        vec![
            None, None, Walkway, Walkway, Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, None, None, Walkway, Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, None, None, None, Walkway, Walkway, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat, Seat,
        ],
        vec![
            None, None, None, None, None, Entry, Seat, Seat, Seat, Seat, Seat, Seat, Seat, Seat,
            Seat, Seat, Seat, Seat, Seat, Seat,
        ],
    ];

    assert_eq!(loaded, expected);
}

#[test]
fn test_find_nearest_seat() {
    let grid = load_grid_from_img("./beno express.png").unwrap();

    let case1 = find_seat_entry(&Point { x: 0, y: 6 }, &grid);
    let expected1 = Point { x: 0, y: 5 };
    assert_eq!(case1, expected1);

    let case2 = find_seat_entry(&Point { x: 3, y: 6 }, &grid);
    let expected2 = Point { x: 3, y: 5 };
    assert_eq!(case2, expected2);

    let case3 = find_seat_entry(&Point { x: 2, y: 8 }, &grid);
    let expected3 = Point { x: 4, y: 8 };
    assert_eq!(case3, expected3);

    let case4 = find_seat_entry(&Point { x: 6, y: 3 }, &grid);
    let expected4 = Point { x: 4, y: 3 };
    assert_eq!(case4, expected4);

    let case5 = find_seat_entry(&Point { x: 9, y: 2 }, &grid);
    let expected5 = Point { x: 9, y: 1 };
    assert_eq!(case5, expected5);

    let case5 = find_seat_entry(&Point { x: 11, y: 8 }, &grid);
    let expected5 = Point { x: 10, y: 8 };
    assert_eq!(case5, expected5);
}

#[test]
fn test_find_path() {
    let grid = load_grid_from_img("./beno express.png").unwrap();

    let case1 = path_find_for_seat(&Point { x: 0, y: 5 }, &Point { x: 5, y: 2 }, &grid).unwrap();
    let expected1 = VecDeque::from(vec![Right, Right, Right, Right, Up, Up, Up, Up, Right]);
    assert_eq!(case1, expected1);

    let case2 = path_find_for_seat(&Point { x: 14, y: 5 }, &Point { x: 12, y: 10 }, &grid).unwrap();
    let expected2 = VecDeque::from(vec![
        Left, Left, Left, Left, Down, Down, Down, Down, Down,
    ]);
    assert_eq!(case2, expected2);
}
