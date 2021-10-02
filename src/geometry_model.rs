use crate::{Face, Face::*, Vec3};

struct Sticker {
    initial: Vec3, // describes the sticker's initial position
    current: Vec3, // describes the sticker's current position
}

impl Sticker {
    pub fn new(initial: Vec3, current: Vec3) -> Self {
        Self { initial, current }
    }
}

struct GMove {
    name: String,
    axis: Vec3,
    angle: i16,
    predicate: fn(Vec3) -> bool,
}

impl GMove {
    pub fn create_gmove(name: String, axis: Vec3, angle: i16, predicate: fn(Vec3) -> bool) -> Self {
        Self {
            name,
            axis,
            angle,
            predicate,
        }
    }
}
