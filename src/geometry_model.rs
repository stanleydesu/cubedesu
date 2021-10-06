use crate::{Axis, Face, Move, Movement, Point3, Turn, TOTAL_STICKERS};
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

    pub fn apply_gmove(sticker: Self, gmove: GMove) -> Self {
        if (gmove.predicate)(sticker.current) {
            let Movement(_, turn) = gmove.movement;
            let turns = if gmove.is_clockwise {
                turn as i8
            } else {
                -(turn as i8)
            };
            Sticker {
                current: Point3::rotate_around_axis(sticker.current, gmove.axis, turns),
                initial: sticker.initial,
            }
        } else {
            sticker
        }
    }
}

/// Represents geometric moves around some axis, which only affect Point3s that
/// satisfy the predicate.
/// Angle is based off the Movement's Turn component.
/// The rotation direction around the axis is based off the is_clockwise flag.
///
/// E.g. GMove(_, Axis::Y, |pos| pos.y >= 0) represents a geometric move around
/// the y axis, that should only affect Point3s that have a y value >= 0
#[derive(Copy, Clone, Debug)]
pub struct GMove {
    movement: Movement,
    axis: Axis,
    is_clockwise: bool, // whether rotation around the axis is clockwise
    predicate: fn(Point3) -> bool,
}

impl GMove {
    pub fn new(
        movement: Movement,
        axis: Axis,
        is_clockwise: bool,
        predicate: fn(Point3) -> bool,
    ) -> Self {
        Self {
            movement,
            axis,
            is_clockwise,
            predicate,
        }
    }
}

// length of each cubic piece is 2 units, with cube origin at (0, 0, 0)
// e.g. the U center piece is centered at (0, 2, 0),
// and the U center sticker is on the surface, at (0, 3, 0)
#[derive(Debug, PartialEq, Eq)]
pub struct GCube([Sticker; TOTAL_STICKERS]);

impl GCube {
    // creates a solved cube
    pub fn new() -> Self {
        let mut v: Vec<Sticker> = vec![];
        // each sticker is on a face
        for face in [-3, 3] {
            // and the other 2 coordinates describe its position on that face
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

    // create the GMove that corresponds to the given Movement
    fn create_gmove(movement: Movement) -> GMove {
        let Movement(m, _) = movement;
        match m {
            // typical moves
            Move::U => GMove::new(movement, Axis::Y, true, |pos| pos.y >= 1),
            Move::Uw => GMove::new(movement, Axis::Y, true, |pos| pos.y >= -1),
            Move::L => GMove::new(movement, Axis::X, false, |pos| pos.x <= -1),
            Move::Lw => GMove::new(movement, Axis::X, false, |pos| pos.x <= 1),
            Move::F => GMove::new(movement, Axis::Z, true, |pos| pos.z >= 1),
            Move::Fw => GMove::new(movement, Axis::Z, true, |pos| pos.z >= -1),
            Move::R => GMove::new(movement, Axis::X, true, |pos| pos.x >= 1),
            Move::Rw => GMove::new(movement, Axis::X, true, |pos| pos.x >= -1),
            Move::B => GMove::new(movement, Axis::Z, false, |pos| pos.z <= -1),
            Move::Bw => GMove::new(movement, Axis::Z, false, |pos| pos.z <= 1),
            Move::D => GMove::new(movement, Axis::Y, false, |pos| pos.y <= -1),
            Move::Dw => GMove::new(movement, Axis::Y, false, |pos| pos.y <= 1),
            // slice moves
            Move::E => GMove::new(movement, Axis::Y, false, |pos| pos.y == 0),
            Move::M => GMove::new(movement, Axis::X, false, |pos| pos.x == 0),
            Move::S => GMove::new(movement, Axis::Z, true, |pos| pos.z == 1),
            // rotations
            Move::X => GMove::new(movement, Axis::X, true, |_| true),
            Move::Y => GMove::new(movement, Axis::Y, true, |_| true),
            Move::Z => GMove::new(movement, Axis::Z, true, |_| true),
        }
    }

    pub fn apply_gmove(&mut self, gmove: GMove) {
        for sticker in self.0.iter_mut() {
            *sticker = Sticker::apply_gmove(*sticker, gmove);
        }
    }

    pub fn apply_gmoves(&mut self, gmoves: &[GMove]) {
        for gmove in gmoves {
            self.apply_gmove(*gmove);
        }
    }
}

pub fn get_face(pos: Point3) -> Face {
    match pos {
        Point3 { x: 3, .. } => Face::R,
        Point3 { x: -3, .. } => Face::L,
        Point3 { y: 3, .. } => Face::U,
        Point3 { y: -3, .. } => Face::D,
        Point3 { z: 3, .. } => Face::F,
        Point3 { z: -3, .. } => Face::B,
        _ => Face::X,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn gcube_test() {
        let mut gcube = GCube::new();
        for m in Move::iter() {
            // apply normal move
            let turn = Turn::Single;
            gcube.apply_gmoves(&[GCube::create_gmove(Movement(m, turn))]);
            // apply inverse
            let turn = Turn::Inverse;
            gcube.apply_gmoves(&[GCube::create_gmove(Movement(m, turn))]);
            // apply double twice
            let turn = Turn::Double;
            gcube.apply_gmoves(&[
                GCube::create_gmove(Movement(m, turn)),
                GCube::create_gmove(Movement(m, turn)),
            ]);
        }
        assert_eq!(gcube, GCube::new());
    }

    #[test]
    fn apply_gmove_to_stickers() {
        // RU sticker (R face)
        let ru = Sticker::from_point(Point3::new(3, 2, 0));
        // FD sticker (F face)
        let fd = Sticker::from_point(Point3::new(0, -2, 3));
        // U center
        let uc = Sticker::from_point(Point3::new(0, 3, 0));

        let u = GMove::new(Movement(Move::Uw, Turn::Single), Axis::Y, true, |pos| {
            pos.y >= 0
        });
        let r2 = GMove::new(Movement(Move::Rw, Turn::Double), Axis::X, true, |pos| {
            pos.x >= 0
        });
        let l = GMove::new(Movement(Move::Lw, Turn::Single), Axis::X, false, |pos| {
            pos.x <= 1
        });

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

        // l doesn't affect RU
        assert_eq!(Sticker::apply_gmove(ru, l), ru);
        // l moves U center to F center
        assert_eq!(
            Sticker::apply_gmove(uc, l),
            Sticker::new(uc.initial, Point3::new(0, 0, 3))
        );
        // l moves FD to DB
        assert_eq!(
            Sticker::apply_gmove(fd, l),
            Sticker::new(fd.initial, Point3::new(0, -3, -2))
        );
    }
}
