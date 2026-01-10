//! Trapped state for a Pokemon

use serde::{Deserialize, Serialize};

/// Trapped state for a Pokemon
/// JavaScript equivalent: false | true | 'hidden' (sim/pokemon.ts)
/// JavaScript uses: false | true | 'hidden'
/// Rust equivalent:
/// - None: Not trapped (false)
/// - Some(false): Trapped and visible (true)
/// - Some(true): Trapped and hidden ('hidden')
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrappedState {
    /// Not trapped
    None,
    /// Trapped but visible to opponent
    Visible,
    /// Trapped and hidden from opponent (Shadow Tag, etc.)
    Hidden,
}

impl Default for TrappedState {
    fn default() -> Self {
        TrappedState::None
    }
}

impl TrappedState {
    /// Check if the Pokemon is trapped (either visible or hidden)
    pub fn is_trapped(&self) -> bool {
        matches!(self, TrappedState::Visible | TrappedState::Hidden)
    }

    /// Check if trapped and hidden
    pub fn is_hidden(&self) -> bool {
        matches!(self, TrappedState::Hidden)
    }
}
