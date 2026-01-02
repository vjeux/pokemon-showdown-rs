// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get foes
    /// Equivalent to pokemon.ts foes()
    ///
    // 	foes(all?: boolean): Pokemon[] {
    // 		return this.side.foes(all);
    // 	}
    //
    // side.foes(all?: boolean) implementation:
    // 	foes(all?: boolean) {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.map(side => side.active[0])
    // 				.filter(pokemon => pokemon && pokemon.side !== this && (all || !!pokemon.hp));
    // 		}
    // 		return this.foe.allies(all);
    // 	}
    //
    // Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
    // so we take Battle as a parameter instead of accessing this.battle
    pub fn foes(&self, battle: &Battle, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        // JS: if (this.battle.gameType === 'freeforall') {
        if battle.game_type == GameType::FreeForAll {
            // Return all active pokemon from other sides
            for (foe_side_idx, foe_side) in battle.sides.iter().enumerate() {
                if foe_side_idx != self.side_index {
                    if let Some(Some(foe_idx)) = foe_side.active.first() {
                        if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                            if include_fainted || foe.hp > 0 {
                                result.push((foe_side_idx, *foe_idx));
                            }
                        }
                    }
                }
            }
        } else {
            // JS: return this.foe.allies(all);
            // Get foe side index
            let foe_side_idx = if self.side_index == 0 { 1 } else { 0 };
            // Return allies from foe side - get any pokemon from that side to call the method
            if let Some(foe_side) = battle.sides.get(foe_side_idx) {
                if let Some(&foe_idx) = foe_side.active.iter().flatten().next() {
                    if let Some(foe_pokemon) = foe_side.pokemon.get(foe_idx) {
                        result = foe_pokemon.allies_and_self(battle, include_fainted);
                    }
                }
            }
        }

        result
    }
}
