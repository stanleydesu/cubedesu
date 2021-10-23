use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

use crate::{Face, ORDERED_FACES, TOTAL_FACES};

#[derive(Debug)]
pub struct FaceletModel<const N: usize>(pub [Face; N * N * TOTAL_FACES])
where
    [Face; N * N * TOTAL_FACES]: ;

impl<const N: usize> FaceletModel<N>
where
    [Face; N * N * TOTAL_FACES]: ,
{
    pub fn new() -> Self {
        let v: Vec<Face> = ORDERED_FACES
            .iter()
            .flat_map(|&face| [face; N * N])
            .collect();
        Self(v.try_into().unwrap())
    }
}

impl<const N: usize> Default for FaceletModel<N>
where
    [Face; N * N * TOTAL_FACES]: ,
{
    fn default() -> Self {
        Self::new()
    }
}

// impl<const N: usize> PartialEq for FaceletModel<N> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.iter().eq(other.0.iter())
//     }
// }

impl<const N: usize> Index<usize> for FaceletModel<N>
where
    [Face; N * N * TOTAL_FACES]: ,
{
    type Output = Face;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for FaceletModel<N>
where
    [Face; N * N * TOTAL_FACES]: ,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Face::*;

    #[test]
    fn new_is_solved() {
        let solved_facelet = [
            U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, D, D,
            D, D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B,
        ];
        let FaceletModel(default) = FaceletModel::<3>::new();
        assert_eq!(default, solved_facelet);
    }
}
