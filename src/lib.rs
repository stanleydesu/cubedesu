mod facelet_model;
pub use facelet_model::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
}
