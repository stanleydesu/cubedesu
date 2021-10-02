mod facelet_model;
pub use facelet_model::*;
mod vec3;
pub use vec3::*;
mod geometry_model;
pub use geometry_model::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
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
    D,
    // slice moves
    E,
    M,
    S,
    // rotations
    X,
    Y,
    Z,
}
