use crate::beano::{Action, Beano, Beanoz};
use crate::common::{Direction, Point};
use std::collections::HashMap;

pub fn run_iteration(beanoz: &mut Beanoz, spawner: &mut HashMap<Point<usize>, Vec<Beano>>) {
    while let Some(next_pos) = beanoz.next_pos() {
        simulate_beano(&next_pos, beanoz);
    }

    for entry in spawner.keys() {
        let spawn_beanoz = spawner.get_mut(entry).unwrap();

        if !spawn_beanoz.is_empty() && check_pos(entry, beanoz).is_none() {
            beanoz.insert(entry, spawn_beanoz[0].clone());
            spawn_beanoz.remove(0);
        }
    }

    beanoz.reset();
}

fn simulate_beano(pos: &Point<usize>, beanoz: &mut Beanoz) {
    let beano = beanoz.get(pos).unwrap();
    let mut pos = *pos;
    match beano.desired_action() {
        Action::None => {}
        Action::ShuffleToSeat => {
            let dir = pos - beano.seat();
            if dir.x == 0 {
                beano.sit()
            } else {
                if dir.x < 0 {
                    pos = (Point::RIGHT + pos.into()).into();
                } else {
                    pos = (Point::LEFT + pos.into()).into();
                }
                beano.shuffle();
            }
        }
        Action::Move(dir) => move_beano(dir, &pos, beanoz),
        Action::CheckRow => {
            let mut cleared = true;
            for (i, x) in if beano.seat().x > pos.x {
                (pos.x + 1)..beano.seat().x
            } else {
                beano.seat().x..(pos.x - 1)
            }
            .enumerate()
            {
                if let Some(o) = check_pos(&Point { x, y: pos.y }, beanoz) {
                    if !o.is_in_isle() {
                        cleared = false;
                        if !o.is_shuffling_out() {
                            o.move_to_isle(i as u32, beano.seat())
                        }
                    }
                }
            }

            if cleared {
                beano.move_to_seat()
            } else {
                beano.wait()
            }
        }
        Action::MovingToIsle(pos) => {
            beano.wait_for_seat(pos);
        }
        Action::WaitingForSeat(seat) => {
            if let Some(o) = check_pos(&seat, beanoz) {
                if o.is_seated() {
                    beano.sit();
                }
            }
        }
        Action::ShuffleOut => {
            let dir = pos - beano.seat();
            if dir.x == 0 {
                beano.walk()
            } else {
                if dir.x < 0 {
                    pos = (Point::RIGHT + pos.into()).into();
                } else {
                    pos = (Point::LEFT + pos.into()).into();
                }
                beano.shuffle();
            }
        }
        Action::Disembark => {}
    }

    beanoz.update(beano, &pos);
}

fn check_pos<'a>(pos: &Point<usize>, beanoz: &'a mut Beanoz) -> Option<&'a mut Beano> {
    if let Some(o) = beanoz.get(pos) {
        if o.done_action() {
            Some(o)
        } else {
            let mut pos = pos;
            simulate_beano(pos, beanoz);
            beanoz.get(pos)
        }
    } else {
        None
    }
}

fn move_beano(dir: Direction, pos: &Point<usize>, beanoz: &mut Beanoz) {
    let beano = beanoz.get(pos).unwrap();
    let new: Point<usize> = (dir.pos() + (*pos).into()).into();
    match check_pos(&new, beanoz) {
        None => beanoz.update(beano, &new),
        Some(o) => {
            if !o.is_walking() {
                beano.wait()
            }
        }
    }
}
