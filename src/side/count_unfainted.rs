use crate::side::*;

impl Side {

    /// Count unfainted Pokemon
    pub fn count_unfainted(&self) -> usize {
        self.pokemon.iter().filter(|p| !p.is_fainted()).count()
    }
}
