use std::convert::TryInto;

use crate::{Movement, Point3, Vec3, ORDERED_FACES, STICKERS_PER_FACE, TOTAL_STICKERS};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Sticker {
    initial: Point3, // describes the sticker's initial position
    current: Point3, // describes the sticker's current position
}

impl Sticker {
    pub fn new(initial: Point3, current: Point3) -> Self {
        Self { initial, current }
    }

    pub fn from_point(point: Point3) -> Self {
        Self::new(point, point)
    }
}

// represents geometric moves
#[derive(Copy, Clone, Debug)]
struct GMove {
    movement: Movement,
    axis: Point3,
    predicate: fn(Point3) -> bool,
}

impl GMove {
    pub fn new(movement: Movement, axis: Vec3, predicate: fn(Point3) -> bool) -> Self {
        Self {
            movement,
            axis,
            predicate,
        }
    }
}

// length of each cubic piece is 2 units, with cube origin at (0, 0, 0)
// e.g. the U center piece is centered at (0, 2, 0),
// and the U center sticker is on the surface, at (0, 3, 0)
pub struct GCube([Sticker; TOTAL_STICKERS]);

impl GCube {
    // creates a solved cube
    pub fn new() -> Self {
        let mut v: Vec<Sticker> = vec![];
        // each sticker is on a face
        for face in [-3, 3] {
            // and the following 2 coordinates describe its position on that face
            // e.g. 0, 0 for the center sticker of that face
            for coord1 in [-2, 0, 2] {
                for coord2 in [-2, 0, 2] {
                    v.push(Sticker::from_point(Point3::new(face, coord1, coord2)));
                    v.push(Sticker::from_point(Point3::new(coord1, face, coord2)));
                    v.push(Sticker::from_point(Point3::new(coord1, coord2, face)));
                }
            }
        }
        let stickers: [Sticker; TOTAL_STICKERS] = v.try_into().unwrap();
        Self(stickers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
