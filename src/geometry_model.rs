use crate::{Axis, Face, FaceletModel, Move, Movement, Point3, Turn, ORDERED_FACES, TOTAL_FACES};
use std::{cmp::Ordering, convert::TryInto};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sticker {
    pub initial: Point3, // describes the sticker's initial position
    pub current: Point3, // describes the sticker's current position
    pub size: usize,
}

impl Sticker {
    pub fn new(size: usize, initial: Point3, current: Point3) -> Self {
        Self {
            size,
            initial,
            current,
        }
    }

    pub fn from_point(size: usize, point: Point3) -> Self {
        Self::new(size, point, point)
    }

    pub fn apply_gmove(sticker: Self, gmove: GMove) -> Self {
        if (gmove.predicate)(sticker) {
            let Movement(_, turn) = gmove.movement;
            let turns = if gmove.is_clockwise {
                turn as i16
            } else {
                -(turn as i16)
            };
            Sticker {
                current: Point3::rotate_around_axis(sticker.current, gmove.axis, turns),
                ..sticker
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
    predicate: fn(Sticker) -> bool,
}

impl GMove {
    pub fn new(
        movement: Movement,
        axis: Axis,
        is_clockwise: bool,
        predicate: fn(Sticker) -> bool,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GCube {
    pub size: usize,
    pub stickers: Vec<Sticker>,
}

impl GCube {
    /// Returns the range of facelet center coordinates along an arbitrary axis.
    pub fn range(size: usize) -> Vec<i16> {
        let n = size as i16;
        (-n + 1..=n - 1).step_by(2).collect()
    }

    // creates a solved cube
    pub fn new(mut size: usize) -> Self {
        if size == 0 {
            size = 1;
        }
        let mut stickers: Vec<Sticker> = vec![];
        // each sticker is on a face
        let n = size as i16;
        for face in [-n, n] {
            // and the other 2 coordinates describe its position on that face
            // e.g. 0, 0 for the center sticker of that face
            for coord1 in Self::range(size) {
                for coord2 in Self::range(size) {
                    stickers.push(Sticker::from_point(size, Point3::new(face, coord1, coord2)));
                    stickers.push(Sticker::from_point(size, Point3::new(coord1, face, coord2)));
                    stickers.push(Sticker::from_point(size, Point3::new(coord1, coord2, face)));
                }
            }
        }
        Self { size, stickers }
    }

    pub fn change_size(&mut self, size: usize) {
        if size != self.size && size > 0 {
            *self = Self::new(size);
        }
    }

    // increases cube size by 1
    pub fn grow(&mut self) {
        self.change_size(self.size + 1);
    }

    // shrinks cube size by 1
    pub fn shrink(&mut self) {
        if self.size > 1 {
            self.change_size(self.size - 1);
        }
    }

    // create the GMove that corresponds to the given Movement
    fn create_gmove(movement: Movement) -> GMove {
        let Movement(m, _) = movement;
        match m {
            // typical moves
            Move::U => GMove::new(movement, Axis::Y, true, |s| {
                s.current.y >= (s.size as i16) - 2
            }),
            Move::Uw => GMove::new(movement, Axis::Y, true, |s| {
                s.current.y >= (s.size as i16) - 2 * 2
            }),
            Move::L => GMove::new(movement, Axis::X, false, |s| {
                s.current.x <= -(s.size as i16) + 2
            }),
            Move::Lw => GMove::new(movement, Axis::X, false, |s| {
                s.current.x <= -(s.size as i16) + 2 * 2
            }),
            Move::F => GMove::new(movement, Axis::Z, true, |s| {
                s.current.z >= (s.size as i16) - 2
            }),
            Move::Fw => GMove::new(movement, Axis::Z, true, |s| {
                s.current.z >= (s.size as i16) - 2 * 2
            }),
            Move::R => GMove::new(movement, Axis::X, true, |s| {
                s.current.x >= (s.size as i16) - 2
            }),
            Move::Rw => GMove::new(movement, Axis::X, true, |s| {
                s.current.x >= (s.size as i16) - 2 * 2
            }),
            Move::B => GMove::new(movement, Axis::Z, false, |s| {
                s.current.z <= -(s.size as i16) + 2
            }),
            Move::Bw => GMove::new(movement, Axis::Z, false, |s| {
                s.current.z <= -(s.size as i16) + 2 * 2
            }),
            Move::D => GMove::new(movement, Axis::Y, false, |s| {
                s.current.y <= -(s.size as i16) + 2
            }),
            Move::Dw => GMove::new(movement, Axis::Y, false, |s| {
                s.current.y <= -(s.size as i16) + 2 * 2
            }),
            // slice moves
            Move::E => GMove::new(movement, Axis::Y, false, |s| s.current.y == 0),
            Move::M => GMove::new(movement, Axis::X, false, |s| s.current.x == 0),
            Move::S => GMove::new(movement, Axis::Z, true, |s| s.current.z == 0),
            // rotations
            Move::X => GMove::new(movement, Axis::X, true, |_| true),
            Move::Y => GMove::new(movement, Axis::Y, true, |_| true),
            Move::Z => GMove::new(movement, Axis::Z, true, |_| true),
        }
    }

    pub fn create_gmoves(movements: &[Movement]) -> Vec<GMove> {
        movements
            .iter()
            .map(|movement| Self::create_gmove(*movement))
            .collect()
    }

    pub fn apply_gmove(&mut self, gmove: GMove) {
        for sticker in self.stickers.iter_mut() {
            *sticker = Sticker::apply_gmove(*sticker, gmove);
        }
    }

    pub fn apply_gmoves(&mut self, gmoves: &[GMove]) {
        for gmove in gmoves {
            self.apply_gmove(*gmove);
        }
    }

    pub fn apply_movement(&mut self, movement: &Movement) {
        self.apply_gmoves(&[Self::create_gmove(*movement)]);
    }

    pub fn apply_movements(&mut self, movements: &[Movement]) {
        self.apply_gmoves(&Self::create_gmoves(movements));
    }

    fn get_face(&self, pos: Point3) -> Face {
        let n = self.size as i16;
        if pos.x == n {
            Face::R
        } else if pos.x == -n {
            Face::L
        } else if pos.y == n {
            Face::U
        } else if pos.y == -n {
            Face::D
        } else if pos.z == n {
            Face::F
        } else if pos.z == -n {
            Face::B
        } else {
            Face::X
        }
    }

    pub fn to_facelet_model(&self) -> FaceletModel {
        let mut facelet_stickers: Vec<Face> =
            Vec::with_capacity(self.size * self.size * TOTAL_FACES);

        // assumes stickers are on the F face
        let mut set_face = |mut stickers: Vec<Sticker>, mut index: usize| {
            // sort the stickers for insertion into the facelet model,
            // where facelets per face are ordered from left to right,
            // then top to bottom. On the F face, top left sticker has the
            // smallest x value (x axis points right),
            // and the highest y value (y axis points up)
            stickers.sort_by(|a, b| {
                match a.current.y > b.current.y
                    || (a.current.y == b.current.y && a.current.x < b.current.x)
                {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                }
            });
            for sticker in stickers {
                facelet_stickers[index] = self.get_face(sticker.initial);
                index += 1;
            }
        };

        for (pos, face) in ORDERED_FACES.iter().enumerate() {
            let mut c = self.clone();
            // move the current face to the F face, then transfer the face data
            match face {
                Face::U => c.apply_movement(&Movement(Move::X, Turn::Inverse)),
                Face::R => c.apply_movement(&Movement(Move::Y, Turn::Single)),
                Face::L => c.apply_movement(&Movement(Move::Y, Turn::Inverse)),
                Face::B => c.apply_movement(&Movement(Move::Y, Turn::Double)),
                Face::D => c.apply_movement(&Movement(Move::X, Turn::Single)),
                _ => {}
            };
            let v: Vec<Sticker> = c
                .stickers
                .iter()
                .cloned()
                .filter(|s| self.get_face(s.current) == Face::F)
                .collect();
            // guaranteed to be 9 stickers on the F face
            set_face(v, pos * self.size * self.size);
        }
        FaceletModel(facelet_stickers.try_into().unwrap())
    }

    pub fn get_curr_face(&self, sticker: Sticker) -> Face {
        self.get_face(sticker.current)
    }

    pub fn get_initial_face(&self, sticker: Sticker) -> Face {
        self.get_face(sticker.initial)
    }
}

#[cfg(test)]
mod tests {
    use crate::scramble_to_movements;

    use super::*;
    use crate::Turn;
    use strum::IntoEnumIterator;

    #[test]
    fn gcube_test_with_my_epic_roux_solutions() {
        let mut gcube = GCube::new(3);
        let scramble = "
        L2 U L' F2 R F2 D2 B U B R2 D2 B2 R2 F' D2 B' U2 B2 L2
        
        x
        U' F' R D r' D U2 F2
        r2 U' F' U' F R'
        U R U2 R2 F R F' R U2 R'
        U M U2 M U' F2 M2 F2

        x2
        ";
        gcube.apply_movements(&scramble_to_movements(scramble).unwrap());
        assert_eq!(gcube, GCube::new(3));

        let mut gcube = GCube::new(3); // 9.46
        let scramble = "
        F2 R' U' B2 L2 D' L2 F2 U B2 U' L2 R2 D2 F' L2 R D' L2 D U
        
        y' x
        D' r' D U2 F2 U' F
        r' U' M' R' U' R
        U r U' r2 D' r U r' D r2 U r'
        U2 M U M' U2 M U' M U' M2 U' M' U2 M U2 M2

        y z2
        ";
        gcube.apply_movements(&scramble_to_movements(scramble).unwrap());
        assert_eq!(gcube, GCube::new(3));

        let mut gcube = GCube::new(3);
        let scramble = "
        R L' U B2 R D2 B' D2 B2 R2 L2 U' L2 U F2 R2 D2 R2 D' L

        y2
        D B' M B'
        r U' R F' U' F
        r2 U M2 U' R
        U' r U' r2 D' r U' r' D r2 U r'
        M' U2 M' U M' U' M2 U M2 U2 M U2 M2

        y2
        ";
        gcube.apply_movements(&scramble_to_movements(scramble).unwrap());
        assert_eq!(gcube, GCube::new(3));

        let mut gcube = GCube::new(3);
        let scramble = "
        R L' U B2 R D2 B' D2 B2 R2 L2 U' L2 U F2 R2 D2 R2 D' L
        
        y2
        D B' M B'
        r U' R F' U' F
        r2 U M2 U' R
        U' r U' r2 D' r U' r' D r2 U r'
        M' U2 M' U M' U' M2 U M2 U2 M U2 M2

        y2
        ";
        gcube.apply_movements(&scramble_to_movements(scramble).unwrap());
        assert_eq!(gcube, GCube::new(3));
    }

    #[test]
    fn gcube_test() {
        let mut gcube = GCube::new(3);
        for m in Move::iter() {
            // apply normal move
            let turn = Turn::Single;
            gcube.apply_movement(&Movement(m, turn));
            // apply inverse
            let turn = Turn::Inverse;
            gcube.apply_movement(&Movement(m, turn));
            // apply double twice
            let turn = Turn::Double;
            gcube.apply_movement(&Movement(m, turn));
            gcube.apply_movement(&Movement(m, turn));
        }
        assert_eq!(gcube, GCube::new(3));
    }
}
