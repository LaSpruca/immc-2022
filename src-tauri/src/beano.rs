mod beanoz;
mod kind;
#[cfg(test)]
mod tests;

use crate::common::{Direction, Point};
pub use beanoz::Beanoz;
pub use kind::Kind;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Action {
    None,
    ShuffleToSeat,
    Move(Direction),
    CheckRow,
    MovingToIsle(Point<usize>),
    WaitingForSeat(Point<usize>),
    ShuffleOut,
    Disembark,
    WaitForWalk(Direction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Task {
    ShuffleIn,
    Stow,
    Move,
    MovingToIsle(Point<usize>),
    WaitingForSeat(Point<usize>),
    Sitting,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Beano {
    cool_down: u32,
    waiting: bool,
    disembarking: bool,
    path: Vec<Direction>,
    luggage: u32,
    done_action: bool,
    task: Task,
    kind: Kind,
    seat: Point<usize>,
}

impl Beano {
    pub fn new(
        kind: Kind,
        seat: Point<usize>,
        path: Vec<Direction>,
        luggage: u32,
        disembarking: bool,
    ) -> Self {
        Self {
            cool_down: 0,
            waiting: false,
            disembarking,
            path,
            luggage,
            done_action: false,
            task: Task::Move,
            kind,
            seat,
        }
    }

    pub fn desired_action(&mut self) -> Action {
        self.done_action = true;

        if self.cool_down > 0 {
            if !self.waiting {
                self.cool_down -= 1
            }

            if self.waiting && self.task == Task::Move {
                if self.path.is_empty() {
                    self.task = Task::Stow;
                    self.waiting = false;
                    Action::None
                } else {
                    Action::WaitForWalk(self.path[0])
                }
            } else {
                Action::None
            }
        } else {
            match self.task {
                Task::ShuffleIn => Action::ShuffleToSeat,
                Task::Stow => {
                    if self.luggage >= 1 {
                        self.luggage -= 1;
                        self.cool_down = self.kind.stow_time();
                        Action::None
                    } else {
                        Action::CheckRow
                    }
                }
                Task::Move => {
                    if self.path.is_empty() {
                        if self.disembarking {
                            Action::Disembark
                        } else {
                            self.task = Task::Stow;
                            Action::None
                        }
                    } else {
                        Action::Move(self.path[0])
                    }
                }
                Task::MovingToIsle(seat) => {
                    self.task = Task::WaitingForSeat(seat);
                    Action::WaitingForSeat(seat)
                }
                Task::WaitingForSeat(seat) => Action::WaitingForSeat(seat),
                Task::Sitting => Action::None,
            }
        }
    }

    pub fn seat(&self) -> Point<usize> {
        self.seat
    }

    pub fn done_action(&self) -> bool {
        self.done_action
    }

    pub fn reset(&mut self) {
        self.done_action = false;
    }

    pub fn sit(&mut self) {
        println!("I sat my ass down");
        self.task = Task::Sitting;
        self.waiting = false;
    }

    pub fn shuffle_in(&mut self) {
        self.task = Task::ShuffleIn;
        self.waiting = false;
        self.cool_down = self.kind.walk_speed() * 2;
    }

    pub fn wait(&mut self) {
        self.cool_down = self.kind.reaction_time();
        self.waiting = true;
    }

    pub fn move_to_seat(&mut self) {
        self.task = Task::Sitting;
        self.cool_down = self.kind.walk_speed() * 2;
        self.waiting = false;
    }

    pub fn move_to_isle(&mut self, i: u32, seat: Point<usize>) {
        self.task = Task::MovingToIsle(seat);
        self.cool_down = (2 * i + 1) * self.kind.walk_speed();
        self.waiting = false;
    }

    pub fn shuffle(&mut self) {
        self.task = Task::ShuffleIn;
        self.cool_down = 2 * self.kind.walk_speed();
        self.waiting = false;
    }

    pub fn is_walking(&self) -> bool {
        self.task == Task::Move
    }

    pub fn is_shuffling_out(&self) -> bool {
        if let Task::MovingToIsle(_) = self.task {
            true
        } else {
            false
        }
    }

    pub fn is_in_isle(&self) -> bool {
        if let Task::WaitingForSeat(_) = self.task {
            true
        } else {
            false
        }
    }

    pub fn is_seated(&self) -> bool {
        self.task == Task::Sitting
    }

    pub fn wait_for_seat(&mut self, seat: Point<usize>) {
        self.task = Task::WaitingForSeat(seat);
        self.waiting = false;
    }

    pub fn walk(&mut self) {
        self.task = Task::Move;
        self.waiting = false;
        self.cool_down = self.kind.walk_speed();
        self.path.remove(0);
    }

    pub fn stop_waiting(&mut self) {
        self.waiting = false;
    }
}
