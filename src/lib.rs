use std::{fmt, str::FromStr};
use strum_macros::{Display, EnumIter, EnumString};

mod facelet_model;
pub use facelet_model::*;
mod vec3;
use vec3::*;
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

#[derive(Debug, Clone)]
pub struct ParseMovementError {
    message: String,
}

impl fmt::Display for ParseMovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseMovementError {
                message: "Empty movement.".to_string(),
            });
        }
        // adjust where Turn is expected to start (Move is 1 or 2 characters)
        let turn_start_index = if s.len() > 1 && s.as_bytes()[1].is_ascii_alphabetic() {
            2
        } else {
            1
        };
        let move_type =
            Move::from_str(&s[0..turn_start_index]).map_err(|_| ParseMovementError {
                message: format!("Failed to parse Move part in {}", s),
            })?;
        let turn_type = Turn::from_str(&s[turn_start_index..]).map_err(|_| ParseMovementError {
            message: format!("Failed to parse Turn part in {}", s),
        })?;
        Ok(Movement(move_type, turn_type))
    }
}

pub fn scramble_to_movements(scramble: &str) -> Result<Vec<Movement>, ParseMovementError> {
    scramble
        .split_whitespace()
        .map(|token| Movement::from_str(token))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn valid_str_to_movement() {
        // test all valid string inputs
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

    #[test]
    fn valid_scramble_to_movements() {
        let scramble = "f L U2 D' r S";
        let movements = scramble_to_movements(scramble).unwrap();
        assert_eq!(
            movements,
            vec![
                Movement(Move::Fw, Turn::Single),
                Movement(Move::L, Turn::Single),
                Movement(Move::U, Turn::Double),
                Movement(Move::D, Turn::Inverse),
                Movement(Move::Rw, Turn::Single),
                Movement(Move::S, Turn::Single),
            ]
        );
    }

    #[test]
    fn invalid_scramble_to_movements() {
        let invalid_scrambles = [
            "f L U2 D' r3 S",
            "FF",
            "u2'",
            "2",
            "F2 D2  D2 d e",
            "2D F2 Z2",
            "Z' z' X' M'2",
        ];
        for scramble in invalid_scrambles {
            assert!(scramble_to_movements(scramble).is_err());
        }
    }
}
