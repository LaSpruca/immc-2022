#[cfg(test)]
mod tests;

use crate::beano::{Action, Beano, Beanoz};
use crate::common::{Direction, Point};
use std::collections::HashMap;

pub fn run_iteration(beanoz: &mut Beanoz, spawner: &mut HashMap<Point<usize>, Vec<Beano>>) {
    while let Some(next_pos) = beanoz.next_pos() {
        simulate_beano(&next_pos, beanoz);
    }

    for entry in spawner.clone().keys() {
        let spawn_beanoz = spawner.get_mut(entry).unwrap();

        if !spawn_beanoz.is_empty() && check_pos(entry, beanoz).is_none() {
            beanoz.insert(entry, spawn_beanoz[0].clone());
            spawn_beanoz.remove(0);
        }
    }

    println!("{beanoz:#?}");

    println!("---------------------------");

    beanoz.reset();
}

fn simulate_beano(pos: &Point<usize>, beanoz: &mut Beanoz) {
    let init_pos = *pos;
    let mut pos = *pos;
    let ro_beano = beanoz.get(&pos).unwrap().clone();

    match beanoz.get(&pos).unwrap().desired_action() {
        Action::None => {}
        Action::ShuffleToSeat => {
            let pos_isize: Point<isize> = pos.into();
            let dir = pos_isize - beanoz.get(&pos).unwrap().seat().into();
            println!("Shuffle: {dir}, Pos {pos}");

            if dir.x == 0 {
                println!("Sit ass down");
                beanoz.get(&pos).unwrap().sit()
            } else {
                if dir.x == 0 {
                    println!("Sit ass down");
                    beanoz.get(&pos).unwrap().sit()
                } else if dir.x < 0 {
                    pos = (Point::RIGHT + pos.into()).into();
                } else {
                    pos = (Point::LEFT + pos.into()).into();
                }

                beanoz.get(&pos_isize.into()).unwrap().shuffle();
            }
        }
        Action::WaitForWalk(dir) => {
            let point = dir.pos() + pos.into();
            if check_pos(&point.into(), beanoz).is_none() {
                beanoz.get(&pos).unwrap().stop_waiting();
            }
        }
        Action::Move(dir) => move_beano(dir, &pos, beanoz),
        Action::CheckRow => {
            let mut cleared = true;

            let moves = if beanoz.get(&pos).unwrap().seat().x > pos.x {
                // If seat is further right than beano
                (pos.x + 1)..(beanoz.get(&pos).unwrap().seat().x + 1)
            } else {
                // If seat is further left than beano
                beanoz.get(&pos).unwrap().seat().x..(pos.x)
            };

            for (i, x) in (moves).enumerate() {
                if let Some(o) = check_pos(&Point { x, y: pos.y }, beanoz) {
                    if !o.is_in_isle() {
                        cleared = false;
                        if !o.is_shuffling_out() && o.is_seated() {
                            o.move_to_isle(i as u32, ro_beano.seat())
                        }
                    } else {
                        println!("Planking, benzo is in isle, but somehow also blocking seats, fat benzo");
                    }
                }
            }

            if cleared {
                beanoz.get(&pos).unwrap().shuffle()
            } else {
                beanoz.get(&pos).unwrap().wait()
            }
        }
        Action::MovingToIsle(pos) => {
            beanoz.get(&pos).unwrap().wait_for_seat(pos);
        }
        Action::WaitingForSeat(seat) => {
            if let Some(o) = check_pos(&seat, beanoz) {
                if o.is_seated() {
                    beanoz.get(&pos).unwrap().sit();
                }
            }
        }
        Action::ShuffleOut => {
            let pos_isize: Point<isize> = pos.into();
            let dir = pos_isize - beanoz.get(&pos).unwrap().seat().into();
            if dir.x == 0 {
                beanoz.get(&pos).unwrap().walk()
            } else {
                if dir.x < 0 {
                    pos = (Point::RIGHT + pos.into()).into();
                } else {
                    pos = (Point::LEFT + pos.into()).into();
                }
                beanoz.get(&pos_isize.into()).unwrap().shuffle();
            }
        }
        Action::Disembark => {}
    }
    beanoz.update(&init_pos.into(), &pos);
}

fn check_pos<'a>(pos: &Point<usize>, beanoz: &'a mut Beanoz) -> Option<&'a mut Beano> {
    let done_action;
    match beanoz.get(pos) {
        Some(o) => {
            done_action = o.done_action();
        }
        None => return None,
    }

    if !done_action {
        simulate_beano(pos, beanoz);
    }

    beanoz.get(pos)
}

fn move_beano(dir: Direction, pos: &Point<usize>, beanoz: &mut Beanoz) {
    let new: Point<usize> = (dir.pos() + (*pos).into()).into();
    match check_pos(&new, beanoz) {
        None => {
            beanoz.get(pos).unwrap().walk();
            beanoz.update(pos, &new);
        }
        Some(o) => {
            if !o.is_walking() {
                beanoz.get(pos).unwrap().wait()
            }
        }
    }
}
