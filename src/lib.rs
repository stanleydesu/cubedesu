mod facelet_model;
pub use facelet_model::*;
mod vec3;
pub use vec3::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
}
