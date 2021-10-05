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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    // typical moves
    U,
    Uw,
    L,
    Lw,
    F,
    Fw,
    R,
    Rw,
    B,
    Bw,
    D,
    Dw,
    // slice moves
    E,
    M,
    S,
    // rotations
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    Single = 1, // one clockwise turn
    Double,     // double turn
    Prime,      // inverse of normal, equivalent to one anti-clockwise turn
                // or three normal turns
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Movement(Move, Turn);
