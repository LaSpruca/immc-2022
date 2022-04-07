use crate::common::{Direction, Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Kind {
    Adult,
    Kid,
    OtherSpecialistSupport,
}

impl Kind {
    pub fn stow_time(&self) -> u32 {
        match self {
            Self::Adult => 50,
            Self::Kid => 60,
            Self::OtherSpecialistSupport => 25,
        }
    }

    pub fn walk_speed(&self) -> u32 {
        match self {
            Self::Adult => 10,
            Self::Kid => 15,
            Self::OtherSpecialistSupport => 10,
        }
    }

    pub fn reaction_time(&self) -> u32 {
        match self {
            Self::Adult => 3,
            Self::Kid => 6,
            Self::OtherSpecialistSupport => 10,
        }
    }
}
