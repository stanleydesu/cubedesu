use crate::{Face, Face::*};

const ORDERED_FACES: [Face; 6] = [U, R, F, D, L, B];
const STICKERS_PER_FACE: usize = 9;
const TOTAL_STICKERS: usize = ORDERED_FACES.len() * STICKERS_PER_FACE;

#[derive(Debug)]
pub struct FaceletModel([Face; TOTAL_STICKERS]);

impl FaceletModel {
    pub fn default_facelet() -> Self {
        let mut stickers = [Face::U; TOTAL_STICKERS];
        let v: Vec<Face> = ORDERED_FACES
            .iter()
            .flat_map(|&face| [face; STICKERS_PER_FACE])
            .collect();
        stickers.copy_from_slice(&v.as_slice());
        Self(stickers)
    }
}

impl PartialEq for FaceletModel {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_solved() {
        let solved_facelet = [
            U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, D, D,
            D, D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B,
        ];
        let FaceletModel(default) = FaceletModel::default_facelet();
        assert_eq!(default, solved_facelet);
    }
}
