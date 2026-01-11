//! DexMoves - Moves lookup helper
//!
//! Equivalent to DexMoves class in dex-moves.ts

use crate::dex::{Dex, MoveData};
use crate::dex_data::ID;

/// Helper struct for moves lookups
/// JavaScript equivalent: DexMoves (sim/dex-moves.ts)
/// 1 field in JavaScript (dex)
pub struct DexMoves<'a> {
    /// Dex reference
    /// JavaScript: readonly dex: ModdedDex
    pub(crate) dex: &'a Dex,
}

impl<'a> DexMoves<'a> {
    /// Get move data by name or ID
    /// Equivalent to DexMoves.get() in dex-moves.ts
    pub fn get(&self, name: &str) -> Option<&'a MoveData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(move_data) = self.dex.moves.get(&id) {
            return Some(move_data);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.dex.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.dex.moves.get(&canonical_id);
        }
        None
    }

    /// Get move by ID
    /// Equivalent to DexMoves.getByID() in dex-moves.ts
    pub fn get_by_id(&self, id: &ID) -> Option<&'a MoveData> {
        // JS: if (id.startsWith('hiddenpower')) {
        //         id = /([a-z]*)([0-9]*)/.exec(id)![1] as ID;
        //     }
        // Hidden power variants (hiddenpowerdark, hiddenpowerbug, etc.) should use the base
        // hiddenpower move data so the onModifyType callback fires
        let id_str = id.as_str();
        if id_str.starts_with("hiddenpower") {
            return self.dex.moves.get(&ID::new("hiddenpower"));
        }
        self.dex.moves.get(id)
    }

    /// Get all moves data
    /// Equivalent to DexMoves.all() in dex-moves.ts
    pub fn all(&self) -> Vec<&'a MoveData> {
        self.dex.moves.values().collect()
    }
}
