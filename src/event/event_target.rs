//! Event Target

/// Target for runEvent - mirrors JavaScript's overloaded target parameter
/// JavaScript: target?: Pokemon | Pokemon[] | Side | Battle | null
#[derive(Debug, Clone)]
pub enum EventTarget {
    /// Single Pokemon target
    Pokemon((usize, usize)),
    /// Multiple Pokemon targets
    Pokemons(Vec<(usize, usize)>),
    /// Side target
    Side(usize),
    /// Battle target (whole battle)
    Battle,
}

impl EventTarget {
    /// Convert from Option<(usize, usize)> for backwards compatibility
    pub fn from_pokemon(pos: Option<(usize, usize)>) -> Option<Self> {
        pos.map(EventTarget::Pokemon)
    }

    /// Convert from side index
    pub fn from_side(side_idx: usize) -> Self {
        EventTarget::Side(side_idx)
    }

    /// Get the pokemon position if this is a Pokemon target
    pub fn as_pokemon(&self) -> Option<(usize, usize)> {
        match self {
            EventTarget::Pokemon(pos) => Some(*pos),
            _ => None,
        }
    }

    /// Get the side index if this is a Side target
    pub fn as_side(&self) -> Option<usize> {
        match self {
            EventTarget::Side(idx) => Some(*idx),
            _ => None,
        }
    }

    /// Get multiple pokemon positions if this is a Pokemons target
    pub fn as_pokemons(&self) -> Option<&Vec<(usize, usize)>> {
        match self {
            EventTarget::Pokemons(positions) => Some(positions),
            _ => None,
        }
    }

    /// Check if this is a Battle target
    pub fn is_battle(&self) -> bool {
        matches!(self, EventTarget::Battle)
    }
}
