//! Team Format

use serde::{Deserialize, Serialize};
use crate::pokemon::PokemonSet;

/// Team format - either packed string or array of Pokemon sets
/// JavaScript equivalent: PokemonSet[] | string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamFormat {
    /// Empty team (will be generated)
    Empty,
    /// Packed team string (needs unpacking with Teams::unpack)
    Packed(String),
    /// Array of Pokemon sets
    Sets(Vec<PokemonSet>),
}

impl Default for TeamFormat {
    fn default() -> Self {
        TeamFormat::Empty
    }
}
