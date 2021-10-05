use crate::{
    Axis, Face, Move, Movement, Point3, Turn, ORDERED_FACES, STICKERS_PER_FACE, TOTAL_STICKERS,
};
use std::convert::TryInto;

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

    pub fn original_face(&self) -> Face {
        match self.initial {
            Point3 { x: 3, .. } => Face::R,
            Point3 { x: -3, .. } => Face::L,
            Point3 { y: 3, .. } => Face::U,
            Point3 { y: -3, .. } => Face::D,
            Point3 { z: 3, .. } => Face::F,
            Point3 { z: -3, .. } => Face::B,
            _ => Face::X,
        }
    }

    pub fn apply_gmove(sticker: Self, gmove: GMove) -> Self {
        if (gmove.predicate)(sticker.current) {
            let Movement(_, turn) = gmove.movement;
            Sticker {
                current: Point3::rotate_around_axis(sticker.current, gmove.axis, turn as i8),
                initial: sticker.initial,
            }
        } else {
            sticker
        }
    }
}

// represents geometric moves
#[derive(Copy, Clone, Debug)]
pub struct GMove {
    movement: Movement,
    axis: Axis,
    predicate: fn(Point3) -> bool,
}

impl GMove {
    pub fn new(movement: Movement, axis: Axis, predicate: fn(Point3) -> bool) -> Self {
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

    pub fn apply_movement(&mut self, movement: Movement) {
        let Movement(m, t) = movement;
        match m {
            // typical moves
            Move::U => {}
            Move::Uw => {}
            Move::L => {}
            Move::Lw => {}
            Move::F => {}
            Move::Fw => {}
            Move::R => {}
            Move::Rw => {}
            Move::B => {}
            Move::Bw => {}
            Move::D => {}
            Move::Dw => {}
            // slice moves
            Move::E => {}
            Move::M => {}
            Move::S => {}
            // rotations
            Move::X => {}
            Move::Y => {}
            Move::Z => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apply_gmove_to_stickers() {
        // RU sticker (R face)
        let ru = Sticker::from_point(Point3::new(3, 2, 0));
        // FD sticker (F face)
        let fd = Sticker::from_point(Point3::new(0, -2, 3));
        // U center
        let uc = Sticker::from_point(Point3::new(0, 3, 0));

        let u = GMove::new(Movement(Move::Uw, Turn::Single), Axis::Y, |pos| pos.y >= 0);
        let r2 = GMove::new(Movement(Move::Rw, Turn::Double), Axis::X, |pos| pos.x >= 0);

        // u moves RU to FU
        assert_eq!(
            Sticker::apply_gmove(ru, u),
            Sticker::new(ru.initial, Point3::new(0, 2, 3))
        );
        // u doesn't affect FD
        assert_eq!(Sticker::apply_gmove(fd, u), fd);
        // u moves U center to... the same place :3
        assert_eq!(Sticker::apply_gmove(uc, u), uc);

        // r2 moves RU to RD
        assert_eq!(
            Sticker::apply_gmove(ru, r2),
            Sticker::new(ru.initial, Point3::new(3, -2, 0))
        );
        // r2 moves U center to D center
        assert_eq!(
            Sticker::apply_gmove(uc, r2),
            Sticker::new(uc.initial, Point3::new(0, -3, 0))
        );
        // r2 moves FD to BU
        assert_eq!(
            Sticker::apply_gmove(fd, r2),
            Sticker::new(fd.initial, Point3::new(0, 2, -3))
        );
    }
}
