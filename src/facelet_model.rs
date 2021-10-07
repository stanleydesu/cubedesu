use std::ops::{Index, IndexMut};

use crate::{Face, Face::*, ORDERED_FACES, STICKERS_PER_FACE, TOTAL_STICKERS};

#[derive(Debug)]
pub struct FaceletModel(pub [Face; TOTAL_STICKERS]);

impl FaceletModel {
    pub fn new() -> Self {
        let mut stickers = [U; TOTAL_STICKERS];
        let v: Vec<Face> = ORDERED_FACES
            .iter()
            .flat_map(|&face| [face; STICKERS_PER_FACE])
            .collect();
        stickers.copy_from_slice(v.as_slice());
        Self(stickers)
    }
}

impl PartialEq for FaceletModel {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl Index<usize> for FaceletModel {
    type Output = Face;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for FaceletModel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_solved() {
        let solved_facelet = [
            U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, D, D,
            D, D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B,
        ];
        let FaceletModel(default) = FaceletModel::new();
        assert_eq!(default, solved_facelet);
    }
}
