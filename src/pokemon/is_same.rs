use crate::*;

impl Pokemon {

    /// Check if this is the same Pokemon (by position and side)
    /// JavaScript pattern: if (target === pokemon) continue;
    pub fn is_same(&self, other: &Pokemon) -> bool {
        self.side_index == other.side_index && self.position == other.position
    }
}
