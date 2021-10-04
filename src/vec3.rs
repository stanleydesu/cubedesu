use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Matrix3([Vec3; 3]);

impl Matrix3 {
    /// constructs a 3x3 matrix, where r1, r2, and r3 are rows 1, 2, and 3
    /// of the matrix respectively
    pub fn new(r1: Vec3, r2: Vec3, r3: Vec3) -> Self {
        Self([r1, r2, r3])
    }
}

impl Mul<Vec3> for Matrix3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            Vec3::dot(self.0[0], rhs),
            Vec3::dot(self.0[1], rhs),
            Vec3::dot(self.0[2], rhs),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// specialised vec3 for i8 only (-128..128)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec3 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Vec3 {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn length_squared(self) -> i8 {
        Self::dot(self, self)
    }

    pub fn dot(lhs: Self, rhs: Self) -> i8 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self::new(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x,
        )
    }

    /// returns the vector rotated upon the specified axis by
    /// n_turns 90-degree clockwise turns
    pub fn rotate_around_axis(self, axis: Axis, n_turns: u8) -> Self {
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<i8> for Vec3 {
    fn mul_assign(&mut self, rhs: i8) {
        *self = *self * rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<i8> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: i8) -> Self {
        self * Self::new(rhs, rhs, rhs)
    }
}

impl Mul<Vec3> for i8 {
    type Output = Vec3;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{
        prelude::{prop_compose, proptest},
        prop_assert_eq,
    };

    prop_compose! {
        // generates any i8 values
        fn any_i8()(i in i8::MIN..=i8::MAX) -> i8 {
            i
        }
    }

    prop_compose! {
        // generates restricted i8 values that won't overflow when squared
        fn arb_i8()(i in -11..=11i8) -> i8 {
            i
        }
    }

    prop_compose! {
        pub fn arb_vec3()(x in arb_i8(), y in arb_i8(), z in arb_i8()) -> Vec3 {
            Vec3::new(x, y, z)
        }
    }

    prop_compose! {
        pub fn any_vec3()(x in any_i8(), y in any_i8(), z in any_i8()) -> Vec3 {
            Vec3::new(x, y, z)
        }
    }

    prop_compose! {
        // custom vec3 where x, y, z values range from min to max (inclusive)
        pub fn gen_vec3(min: i8, max: i8)(x in min..=max, y in min..=max, z in min..=max) -> Vec3 {
            Vec3::new(x, y, z)
        }
    }

    proptest! {
        #[test]
        fn matrix_mult(v1 in gen_vec3(-5, 5),
                       v2 in gen_vec3(-5, 5),
                       v3 in gen_vec3(-5, 5),
                       vec in gen_vec3(-5, 5)) {
            let product = Matrix3::new(v1, v2, v3) * vec;
            let expected = Vec3::new(Vec3::dot(v1, vec),
                                     Vec3::dot(v2, vec),
                                     Vec3::dot(v3, vec));
            prop_assert_eq!(product, expected);
        }
    }

    proptest! {
        #[test]
        fn new_constructs_with_parameters(x in any_i8(), y in any_i8(), z in any_i8()) {
            prop_assert_eq!(Vec3::new(x, y, z), Vec3 { x, y, z });
        }

        #[test]
        fn zero_creates_zero_vector(x in 0..=0i8, y in 0..=0i8, z in 0..=0i8) {
            prop_assert_eq!(Vec3::zero(), Vec3 { x, y, z });
        }

        #[test]
        fn neg_op_idempotent(v in gen_vec3(-i8::MAX, i8::MAX)) {
            prop_assert_eq!(v, --v);
        }

        #[test]
        fn neg_op_negates_vec(v in gen_vec3(-i8::MAX, i8::MAX)) {
            prop_assert_eq!(-v, Vec3::new(-v.x, -v.y, -v.z));
        }

        #[test]
        fn add_op_commutative(v1 in arb_vec3(), v2 in arb_vec3()) {
            prop_assert_eq!(v1 + v2, v2 + v1);
        }

        #[test]
        fn add_op_identity(v in arb_vec3()) {
            prop_assert_eq!(v + Vec3::zero(), v);
        }

        #[test]
        fn add_associative(v1 in arb_vec3(), v2 in arb_vec3(), v3 in arb_vec3()) {
            prop_assert_eq!((v1 + v2) + v3, v1 + (v2 + v3));
        }

        #[test]
        fn add_op_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let expected = Vec3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);
            prop_assert_eq!(v1 + v2, expected);
        }

        #[test]
        fn sub_op_identity(v in arb_vec3()) {
            prop_assert_eq!(v - Vec3::zero(), v);
        }

        #[test]
        fn sub_op_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let expected = Vec3::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);
            prop_assert_eq!(v1 - v2, expected);
        }

        #[test]
        fn mul_vec3s_commutative(v1 in arb_vec3(), v2 in arb_vec3()) {
            prop_assert_eq!(v1 * v2, v2 * v1);
        }

        #[test]
        fn mul_vec3s_identity(v in any_vec3()) {
            prop_assert_eq!(v * Vec3::new(1, 1, 1), v);
        }

        #[test]
        fn mul_vec3s_zero(v in arb_vec3()) {
            prop_assert_eq!(v * Vec3::zero(), Vec3::zero());
        }

        #[test]
        fn mul_vec3s_associative(v1 in gen_vec3(-5, 5),
                                 v2 in gen_vec3(-5, 5),
                                 v3 in gen_vec3(-5, 5)) {
            prop_assert_eq!((v1 * v2) * v3, v1 * (v2 * v3));
        }

        #[test]
        fn mul_vec3s_distributive(v1 in gen_vec3(-5, 5),
                                  v2 in gen_vec3(-5, 5),
                                  v3 in gen_vec3(-5, 5)) {
            prop_assert_eq!(v1 * (v2 + v3), (v1 * v2) + (v1 * v3));
        }

        #[test]
        fn mul_vec3s_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let expected = Vec3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z);
            prop_assert_eq!(v1 * v2, expected);
        }

        #[test]
        fn mul_scalar_identity(v in any_vec3()) {
            prop_assert_eq!(v * 1, v);
        }

        #[test]
        fn mul_scalar_zero(v in arb_vec3()) {
            prop_assert_eq!(v * 0, Vec3::zero());
        }

        #[test]
        fn mul_scalar_associative(v1 in gen_vec3(-5, 5), scalar in -5..=5i8, v2 in gen_vec3(-5, 5)) {
            prop_assert_eq!((v1 * scalar) * v2, v1 * (scalar * v2));
        }

        #[test]
        fn mul_scalar_correct(v in arb_vec3(), scalar in arb_i8()) {
            let expected = Vec3::new(v.x * scalar, v.y * scalar, v.z * scalar);
            prop_assert_eq!(v * scalar, expected);
        }

        #[test]
        fn length_squared_correct(v in gen_vec3(-6, 6)) {
            let expected = v.x * v.x + v.y * v.y + v.z * v.z;
            prop_assert_eq!(v.length_squared(), expected);
        }

        #[test]
        fn display_correct(v in any_vec3()) {
            let expected = format!("{} {} {}", v.x, v.y, v.z);
            assert_eq!(format!("{}", v), expected);
        }

        #[test]
        fn dot_product_commutative(v1 in gen_vec3(-6, 6), v2 in gen_vec3(-6, 6)) {
            assert_eq!(Vec3::dot(v1, v2), Vec3::dot(v2, v1));
        }

        #[test]
        fn dot_product_correct(v1 in gen_vec3(-6, 6), v2 in gen_vec3(-6, 6)) {
            let expected = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
            assert_eq!(Vec3::dot(v1, v2), expected);
        }

        #[test]
        fn cross_product_correct(v1 in gen_vec3(-7, 7), v2 in gen_vec3(-7, 7)) {
            let expected = Vec3::new(
                v1.y * v2.z - v1.z * v2.y,
                v1.z * v2.x - v1.x * v2.z,
                v1.x * v2.y - v1.y * v2.x,
            );
            prop_assert_eq!(Vec3::cross(v1, v2), expected);
        }
    }
}
