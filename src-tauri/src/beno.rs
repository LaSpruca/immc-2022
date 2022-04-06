#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use crate::common::{Direction, Point};

#[derive(Serialize, Deserialize)]
pub enum Kind {
    Adult,
    ParentChild,
    OtherSpecialistSupport,
}

impl Kind {
    pub fn stow_time(&self) -> u32 {
        match self {
            Self::Adult => 50,
            Self::ParentChild => 100,
            Self::OtherSpecialistSupport => 25,
        }
    }

    pub fn walk_speed(&self) -> u32 {
        match self {
            Self::Adult => 10,
            Self::ParentChild => 15,
            Self::OtherSpecialistSupport => 10,
        }
    }

    pub fn reaction_time(&self) -> u32 {
        match self {
            Self::Adult => 3,
            Self::ParentChild => 6,
            Self::OtherSpecialistSupport => 10,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Task {
    Waiting,
    Walking,
    WaitingShuffle,
    Stowing,
    Sitting,
    ShufflingIn,
    ShufflingOut,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Move(Direction),
    NotifyRowAndMove(Direction),
    StowBags,
    SeatShuffle,
    None,
    SeatMove(Point<usize>),
}

#[derive(Serialize, Deserialize)]
pub struct Beno {
    cool_down: u32,
    task: Task,
    luggage: u32,
    kind: Kind,
    seat: Point<usize>,
    path: Vec<Direction>,
    done_action: bool,
}

impl Beno {
    pub fn get_action(&mut self) -> Action {
        if self.cool_down != 0 {
            // If the waiting, the can't react
            if self.task != Task::Waiting {
                self.cool_down -= 1;
            }

            self.done_action = true;
            return Action::None;
        }
        
        if self.task == Task::Stowing {
            if self.luggage > 0 {
                return Action::StowBags;
            } else {
                return Action::SeatMove(self.seat);
            }
        }

        if self.path.is_empty() {
            return Action::StowBags;
        } else if self.path.len() == 1 {
            return Action::NotifyRowAndMove(self.path[0]);
        } else {
            return Action::Move(self.path[0]);
        }
    }

    pub fn stow(&mut self) {
        self.cool_down = self.kind.stow_time();
        self.task = Task::Stowing;

        self.done_action = true;
    }

    pub fn walk(&mut self) {
        self.cool_down = self.kind.walk_speed();
        self.path.remove(0);
        self.task = Task::Walking;
        self.done_action = true;
    }

    pub fn stop(&mut self) {
        self.cool_down = self.kind.reaction_time();
        self.task = Task::Waiting;
        self.done_action = true;
    }

    pub fn shuffle_in(&mut self) {
        self.cool_down = self.kind.walk_speed() * 2;
        self.task = Task::ShufflingIn;
        self.done_action = true;
    }

    pub fn shuffle_out(&mut self) {
        self.cool_down = self.kind.walk_speed() * 2;
        self.task = Task::ShufflingOut;
        self.done_action = true;
    }

    pub fn is_seated(&self) -> bool {
        self.task == Task::Sitting
    }
}
