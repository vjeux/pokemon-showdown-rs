//! Record of a Pokemon that attacked this Pokemon

use serde::{Deserialize, Serialize};

use crate::dex_data::ID;

/// Record of a Pokemon that attacked this Pokemon
/// Equivalent to Attacker interface in pokemon.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Attacker (sim/pokemon.ts)
/// 6 fields in JavaScript
pub struct Attacker {
    /// Source Pokemon (side_idx, poke_idx)
    /// JavaScript: source: Pokemon
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub source: (usize, usize),
    /// Damage dealt
    /// JavaScript: damage: number
    pub damage: i32,
    /// Whether this attack happened this turn
    /// JavaScript: thisTurn: boolean
    pub this_turn: bool,
    /// Move ID used
    /// JavaScript: move?: ID
    pub move_id: Option<ID>,
    /// Source slot
    /// JavaScript: slot: PokemonSlot
    pub slot: (usize, usize),
    /// Damage value (can be number, boolean, or undefined)
    /// JavaScript: damageValue?: number | boolean | undefined
    /// TODO: Rust uses Option<i32>, cannot represent boolean variant
    pub damage_value: Option<i32>,
}
