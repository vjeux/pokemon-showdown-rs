//! A Pokemon's move slot

use serde::{Deserialize, Serialize};

use crate::dex_data::ID;

/// A Pokemon's move slot
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: MoveSlot (sim/pokemon.ts)
/// 9 fields in JavaScript
pub struct MoveSlot {
    /// Move ID
    /// JavaScript: id: ID
    pub id: ID,
    /// Move name
    /// JavaScript: move: string
    /// TODO: Rust uses move_name to avoid keyword conflict with 'move'
    pub move_name: String,
    /// Current PP
    /// JavaScript: pp: number
    pub pp: u8,
    /// Maximum PP
    /// JavaScript: maxpp: number
    pub maxpp: u8,
    /// Move target type
    /// JavaScript: target?: string
    pub target: Option<String>,
    /// Whether move is disabled
    /// JavaScript: disabled: boolean | 'hidden'
    /// TODO: Rust uses bool, cannot represent 'hidden' string variant
    pub disabled: bool,
    /// What disabled the move
    /// JavaScript: disabledSource?: string
    pub disabled_source: Option<String>,
    /// Whether move was used this turn
    /// JavaScript: used: boolean
    pub used: bool,
    /// Virtual move flag
    /// JavaScript: virtual?: boolean
    /// TODO: Rust uses virtual_move to avoid keyword conflict with potential 'virtual'
    pub virtual_move: bool,
}

impl MoveSlot {
    pub fn new(id: ID, move_name: String, pp: u8, maxpp: u8) -> Self {
        Self {
            id,
            move_name,
            pp,
            maxpp,
            target: None,
            disabled: false,
            disabled_source: None,
            used: false,
            virtual_move: false,
        }
    }
}
