use super::*;
use crate::common::Direction::*;

#[test]
fn test_beno_walk() {
    let mut beno_boi = Beno {
        cool_down: 0,
        task: Task::Walking,
        luggage: 1,
        kind: Kind::Adult,
        seat: Point { x: 1, y: 5 },
        path: vec![Left, Left, Down, Down, Down, Down, Down],
        done_action: false,
    };

    let mut beno_pos: Point<usize> = Point { x: 0, y: 0};

    while !beno_boi.is_seated() {
        let action = beno_boi.get_action();

        match action {
            Action::None => {}
            Action::Move(direction) => {
                let temp: Point<isize> = beno_pos.into();
                beno_pos = (temp + match direction {
                    Up => Point::UP,
                    Down => Point::DOWN,
                    Left => Point::LEFT,
                    Right => Point::RIGHT,
                }).into();

                beno_boi.walk();
            },
            Action::NotifyRowAndMove(direction) => {
                beno_pos = (match direction {
                    Up => Point::UP,
                    Down => Point::DOWN,
                    Left => Point::LEFT,
                    Right => Point::RIGHT,
                } + beno_pos.into()).into();

                beno_boi.walk();
            },
            Action::StowBags => {
                beno_boi.stow()
            },
            Action::SeatShuffle => {
                beno_boi.shuffle_in();
                beno_pos = (Point::LEFT + beno_pos.into()).into();

            },
            Action::SeatMove(_) => {},
        }
    }
    
}