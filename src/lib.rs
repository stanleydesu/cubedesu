use std::str::FromStr;
use strum::ParseError;
use strum_macros::{Display, EnumIter, EnumString};

mod facelet_model;
pub use facelet_model::*;
mod vec3;
pub use vec3::*;
pub type Point3 = vec3::Vec3;
mod geometry_model;
pub use geometry_model::*;

pub const ORDERED_FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];
pub const STICKERS_PER_FACE: usize = 9;
pub const TOTAL_STICKERS: usize = ORDERED_FACES.len() * STICKERS_PER_FACE;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
    X,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, EnumString, Display)]

pub enum Move {
    // typical moves
    U,
    #[strum(serialize = "Uw", serialize = "u")]
    Uw,
    L,
    #[strum(serialize = "Lw", serialize = "l")]
    Lw,
    F,
    #[strum(serialize = "Fw", serialize = "f")]
    Fw,
    R,
    #[strum(serialize = "Rw", serialize = "r")]
    Rw,
    B,
    #[strum(serialize = "Bw", serialize = "b")]
    Bw,
    D,
    #[strum(serialize = "Dw", serialize = "d")]
    Dw,
    // slice moves
    E,
    M,
    S,
    // rotations
    #[strum(ascii_case_insensitive)]
    X,
    #[strum(ascii_case_insensitive)]
    Y,
    #[strum(ascii_case_insensitive)]
    Z,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum Turn {
    #[strum(serialize = "")]
    Single = 1, // one clockwise turn
    #[strum(serialize = "2")]
    Double, // double turn
    #[strum(serialize = "'")]
    Inverse, // inverse of normal, equivalent to one anti-clockwise turn
             // or three normal turns
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Movement(Move, Turn);

impl FromStr for Movement {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::VariantNotFound);
        }
        // Move is up to 2 characters, so adjust where Turn is expected to start
        let turn_start_index = if s.len() > 1 && s.as_bytes()[1].is_ascii_alphabetic() {
            2
        } else {
            1
        };
        let move_type = Move::from_str(&s[0..turn_start_index])?;
        let turn_type = Turn::from_str(&s[turn_start_index..])?;
        Ok(Movement(move_type, turn_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn movement_from_valid_str() {
        // test all possible valid string inputs
        for m in Move::iter() {
            for t in Turn::iter() {
                let movement_string = format!("{}{}", m.to_string(), t.to_string());
                assert_eq!(
                    Movement::from_str(&movement_string).unwrap(),
                    Movement(m, t)
                );
            }
        }
    }
}
