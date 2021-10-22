use crate::{
    Axis, Face, FaceletModel, Move, Movement, Point3, Turn, ORDERED_FACES, STICKERS_PER_FACE,
    TOTAL_FACES, TOTAL_STICKERS,
};
use std::{cmp::Ordering, convert::TryInto};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sticker {
    pub initial: Point3, // describes the sticker's initial position
    pub current: Point3, // describes the sticker's current position
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
                turn as i16
            } else {
                -(turn as i16)
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GCube<const N: usize>(pub [Sticker; N * N * TOTAL_FACES])
where
    [(); N * N * TOTAL_FACES]: Sized;

impl<const N: usize> GCube<N>
where
    [(); N * N * TOTAL_FACES]: Sized,
{
    /// Returns the range of facelet center coordinates along an arbitrary axis.
    pub fn range() -> [i16; N] {
        let n = N as i16;
        (-n + 1..=n - 1)
            .step_by(2)
            .collect::<Vec<i16>>()
            .try_into()
            .unwrap()
    }

    // creates a solved cube
    pub fn new() -> Self {
        let mut v: Vec<Sticker> = vec![];
        // each sticker is on a face
        let n = N as i16;
        for face in [-n, n] {
            // and the other 2 coordinates describe its position on that face
            // e.g. 0, 0 for the center sticker of that face
            for coord1 in Self::range() {
                for coord2 in Self::range() {
                    v.push(Sticker::from_point(Point3::new(face, coord1, coord2)));
                    v.push(Sticker::from_point(Point3::new(coord1, face, coord2)));
                    v.push(Sticker::from_point(Point3::new(coord1, coord2, face)));
                }
            }
        }
        Self(v.try_into().unwrap())
    }

    // create the GMove that corresponds to the given Movement
    fn create_gmove(movement: Movement) -> GMove {
        let Movement(m, _) = movement;
        match m {
            // typical moves
            Move::U => GMove::new(movement, Axis::Y, true, |pos| pos.y >= (N as i16) - 2),
            Move::Uw => GMove::new(movement, Axis::Y, true, |pos| pos.y >= (N as i16) - 2 * 2),
            Move::L => GMove::new(movement, Axis::X, false, |pos| pos.x <= -(N as i16) + 2),
            Move::Lw => GMove::new(movement, Axis::X, false, |pos| pos.x <= -(N as i16) + 2 * 2),
            Move::F => GMove::new(movement, Axis::Z, true, |pos| pos.z >= (N as i16) - 2),
            Move::Fw => GMove::new(movement, Axis::Z, true, |pos| pos.z >= (N as i16) - 2 * 2),
            Move::R => GMove::new(movement, Axis::X, true, |pos| pos.x >= (N as i16) - 2),
            Move::Rw => GMove::new(movement, Axis::X, true, |pos| pos.x >= (N as i16) - 2 * 2),
            Move::B => GMove::new(movement, Axis::Z, false, |pos| pos.z <= -(N as i16) + 2),
            Move::Bw => GMove::new(movement, Axis::Z, false, |pos| pos.z <= -(N as i16) + 2 * 2),
            Move::D => GMove::new(movement, Axis::Y, false, |pos| pos.y <= -(N as i16) + 2),
            Move::Dw => GMove::new(movement, Axis::Y, false, |pos| pos.y <= -(N as i16) + 2 * 2),
            // slice moves
            Move::E => GMove::new(movement, Axis::Y, false, |pos| pos.y == 0),
            Move::M => GMove::new(movement, Axis::X, false, |pos| pos.x == 0),
            Move::S => GMove::new(movement, Axis::Z, true, |pos| pos.z == 0),
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
        for sticker in self.0.iter_mut() {
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
        let n = N as i16;
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
        let mut facelet_stickers = [Face::U; TOTAL_STICKERS];

        // assumes stickers are on the F face
        let mut set_face = |mut stickers: [Sticker; STICKERS_PER_FACE], mut index: usize| {
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
            let mut c = *self;
            // move the current face to the F face, then transfer the face data
            match face {
                Face::U => c.apply_movement(&Movement(Move::X, Turn::Inverse)),
                Face::R => c.apply_movement(&Movement(Move::Y, Turn::Single)),
                Face::L => c.apply_movement(&Movement(Move::Y, Turn::Inverse)),
                Face::B => c.apply_movement(&Movement(Move::Y, Turn::Double)),
                Face::D => c.apply_movement(&Movement(Move::X, Turn::Single)),
                _ => {}
            };
            let v: Vec<Sticker> =
                c.0.iter()
                    .cloned()
                    .filter(|s| self.get_face(s.current) == Face::F)
                    .collect();
            // guaranteed to be 9 stickers on the F face
            let stickers: [Sticker; STICKERS_PER_FACE] = v.try_into().unwrap();
            set_face(stickers, pos * STICKERS_PER_FACE);
        }
        FaceletModel(facelet_stickers)
    }

    pub fn get_curr_face(&self, sticker: Sticker) -> Face {
        self.get_face(sticker.current)
    }

    pub fn get_initial_face(&self, sticker: Sticker) -> Face {
        self.get_face(sticker.initial)
    }
}

impl<const N: usize> Default for GCube<N>
where
    [(); N * N * TOTAL_FACES]: Sized,
{
    fn default() -> Self {
        Self::new()
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
        let mut gcube = GCube::<3>::new();
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
        assert_eq!(gcube, GCube::<3>::new());

        let mut gcube = GCube::<3>::new(); // 9.46
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
        assert_eq!(gcube, GCube::<3>::new());

        let mut gcube = GCube::<3>::new();
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
        assert_eq!(gcube, GCube::<3>::new());

        let mut gcube = GCube::<3>::new();
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
        assert_eq!(gcube, GCube::<3>::new());
    }

    #[test]
    fn gcube_test() {
        let mut gcube = GCube::<3>::new();
        for m in Move::iter() {
            // apply normal move
            let turn = Turn::Single;
            gcube.apply_movement(&Movement(m, turn));
            // apply inverse
            let turn = Turn::Inverse;
            gcube.apply_movement(&Movement(m, turn));
            // apply double twice
            let turn = Turn::Double;
            gcube.apply_gmoves(&[
                GCube::<3>::create_gmove(Movement(m, turn)),
                GCube::<3>::create_gmove(Movement(m, turn)),
            ]);
        }
        assert_eq!(gcube, GCube::<3>::new());
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
