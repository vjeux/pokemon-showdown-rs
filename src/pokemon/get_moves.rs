use crate::*;

impl Pokemon {

    // ==========================================
    // Additional methods from pokemon.ts
    // ==========================================

    /// Get moves as string list
    /// Equivalent to moves getter in pokemon.ts
    //
    // 	getMoves(lockedMove?: ID | null, restrictData?: boolean): {
    // 		move: string, id: ID, disabled?: string | boolean, disabledSource?: string,
    // 		target?: string, pp?: number, maxpp?: number,
    // 	}[] {
    // 		if (lockedMove) {
    // 			lockedMove = toID(lockedMove);
    // 			this.trapped = true;
    // 			if (lockedMove === 'recharge') {
    // 				return [{
    // 					move: 'Recharge',
    // 					id: 'recharge' as ID,
    // 				}];
    // 			}
    // 			for (const moveSlot of this.moveSlots) {
    // 				if (moveSlot.id !== lockedMove) continue;
    // 				return [{
    // 					move: moveSlot.move,
    // 					id: moveSlot.id,
    // 				}];
    // 			}
    // 			// does this happen?
    // 			return [{
    // 				move: this.battle.dex.moves.get(lockedMove).name,
    // 				id: lockedMove,
    // 			}];
    // 		}
    // 		const moves = [];
    // 		let hasValidMove = false;
    // 		for (const moveSlot of this.moveSlots) {
    // 			let moveName = moveSlot.move;
    // 			if (moveSlot.id === 'hiddenpower') {
    // 				moveName = `Hidden Power ${this.hpType}`;
    // 				if (this.battle.gen < 6) moveName += ` ${this.hpPower}`;
    // 			} else if (moveSlot.id === 'return' || moveSlot.id === 'frustration') {
    // 				const basePowerCallback = this.battle.dex.moves.get(moveSlot.id).basePowerCallback as (pokemon: Pokemon) => number;
    // 				moveName += ` ${basePowerCallback(this)}`;
    // 			}
    // 			let target = moveSlot.target;
    // 			switch (moveSlot.id) {
    // 			case 'curse':
    // 				if (!this.hasType('Ghost')) {
    // 					target = this.battle.dex.moves.get('curse').nonGhostTarget;
    // 				}
    // 				break;
    // 			case 'pollenpuff':
    // 				// Heal Block only prevents Pollen Puff from targeting an ally when the user has Heal Block
    // 				if (this.volatiles['healblock']) {
    // 					target = 'adjacentFoe';
    // 				}
    // 				break;
    // 			case 'terastarstorm':
    // 				if (this.species.name === 'Terapagos-Stellar') {
    // 					target = 'allAdjacentFoes';
    // 				}
    // 				break;
    // 			}
    // 			let disabled = moveSlot.disabled;
    // 			if (this.volatiles['dynamax']) {
    // 				// if each of a Pokemon's base moves are disabled by one of these effects, it will Struggle
    // 				const canCauseStruggle = ['Encore', 'Disable', 'Taunt', 'Assault Vest', 'Belch', 'Stuff Cheeks'];
    // 				disabled = this.maxMoveDisabled(moveSlot.id) || disabled && canCauseStruggle.includes(moveSlot.disabledSource!);
    // 			} else if (moveSlot.pp <= 0 && !this.volatiles['partialtrappinglock']) {
    // 				disabled = true;
    // 			}
    //
    // 			if (disabled === 'hidden') {
    // 				disabled = !restrictData;
    // 			}
    // 			if (!disabled) {
    // 				hasValidMove = true;
    // 			}
    //
    // 			moves.push({
    // 				move: moveName,
    // 				id: moveSlot.id,
    // 				pp: moveSlot.pp,
    // 				maxpp: moveSlot.maxpp,
    // 				target,
    // 				disabled,
    // 			});
    // 		}
    // 		return hasValidMove ? moves : [];
    // 	}
    //
    pub fn get_moves(&mut self, locked_move: Option<&ID>) -> Vec<serde_json::Value> {
        // JS: if (lockedMove) {
        // JS:     lockedMove = toID(lockedMove);
        // JS:     this.trapped = true;
        // ✅ NOW IMPLEMENTED (Session 24 Part 67): lockedMove parameter
        // ✅ NOW IMPLEMENTED (Session 24 Part 73): trapped = true side effect for 1:1 JavaScript equivalence
        if let Some(locked_move_id) = locked_move {
            // JS:     this.trapped = true;
            self.trapped = TrappedState::Visible;
            // JS:     if (lockedMove === 'recharge') {
            // JS:         return [{ move: 'Recharge', id: 'recharge' as ID }];
            // JS:     }
            // ✅ NOW IMPLEMENTED (Session 24 Part 67): Recharge special case
            if locked_move_id.as_str() == "recharge" {
                return vec![serde_json::json!({
                    "move": "Recharge",
                    "id": "recharge"
                })];
            }

            // JS:     for (const moveSlot of this.moveSlots) {
            // JS:         if (moveSlot.id !== lockedMove) continue;
            // JS:         return [{ move: moveSlot.move, id: moveSlot.id }];
            // JS:     }
            // ✅ NOW IMPLEMENTED (Session 24 Part 67): Locked move slot search and early return
            for slot in &self.move_slots {
                if slot.id == *locked_move_id {
                    return vec![serde_json::json!({
                        "move": slot.move_name.clone(),
                        "id": slot.id.as_str()
                    })];
                }
            }

            // JS:     return [{ move: this.battle.dex.moves.get(lockedMove).name, id: lockedMove }];
            // Note: Missing fallback to dex lookup for locked move (needs Battle/Dex reference)
            // For now, return the ID as the name if no move slot matches
            return vec![serde_json::json!({
                "move": locked_move_id.as_str(),
                "id": locked_move_id.as_str()
            })];
        }

        // JS: const moves = [];
        // JS: let hasValidMove = false;
        // ✅ NOW IMPLEMENTED (Session 24 Part 63): hasValidMove tracking
        let mut has_valid_move = false;

        // JS: for (const moveSlot of this.moveSlots) {
        // JS:     let moveName = moveSlot.move;
        let moves: Vec<serde_json::Value> = self.move_slots
            .iter()
            .map(|slot| {
                // JS:     if (moveSlot.id === 'hiddenpower') {
                // JS:         moveName = `Hidden Power ${this.hpType}`;
                // JS:         if (this.battle.gen < 6) moveName += ` ${this.hpPower}`;
                // JS:     }
                // ✅ NOW IMPLEMENTED (Session 24 Part 63): Hidden Power type/power formatting
                let move_name = if slot.id.as_str() == "hiddenpower" {
                    if !self.hp_type.is_empty() {
                        let name = format!("Hidden Power {}", self.hp_type);
                        // Note: Would need Battle reference for gen check to append power
                        // Assuming modern gen (6+) for now, don't append power
                        name
                    } else {
                        slot.move_name.clone()
                    }
                } else if slot.id.as_str() == "return" || slot.id.as_str() == "frustration" {
                    // JS:     else if (moveSlot.id === 'return' || moveSlot.id === 'frustration') {
                    // JS:         const basePowerCallback = this.battle.dex.moves.get(moveSlot.id).basePowerCallback as (pokemon: Pokemon) => number;
                    // JS:         moveName += ` ${basePowerCallback(this)}`;
                    // JS:     }
                    // Note: Missing Return/Frustration power calculation (needs Dex access)
                    slot.move_name.clone()
                } else {
                    slot.move_name.clone()
                };

                // JS:     let target = moveSlot.target;
                // JS:     switch (moveSlot.id) {
                // JS:     case 'curse': ...
                // JS:     case 'pollenpuff': ...
                // JS:     case 'terastarstorm': ...
                // JS:     }
                // Note: Missing target overrides for Curse, Pollen Puff, Tera Star Storm (needs volatiles/type checks)
                let target = slot.target.as_deref().unwrap_or("");

                // JS:     let disabled = moveSlot.disabled;
                // JS:     if (this.volatiles['dynamax']) { ... }
                // JS:     else if (moveSlot.pp <= 0 && !this.volatiles['partialtrappinglock']) {
                // JS:         disabled = true;
                // JS:     }
                // ✅ NOW IMPLEMENTED (Session 24 Part 63): Basic disabled calculation
                let disabled = if slot.pp == 0 && !self.has_volatile(&ID::new("partialtrappinglock")) {
                    true
                } else {
                    slot.disabled
                };
                // Note: Missing Dynamax disabled logic (needs maxMoveDisabled method)

                // JS:     if (disabled === 'hidden') disabled = !restrictData;
                // Note: Missing restrictData parameter and hidden disabled handling

                // JS:     if (!disabled) hasValidMove = true;
                // Can't mutate has_valid_move from inside map, will handle after

                // JS:     moves.push({ move: moveName, id: moveSlot.id, pp: moveSlot.pp, maxpp: moveSlot.maxpp, target, disabled });
                // ✅ NOW IMPLEMENTED (Session 24 Part 63): Return full move object structure
                serde_json::json!({
                    "move": move_name,
                    "id": slot.id.as_str(),
                    "pp": slot.pp,
                    "maxpp": slot.maxpp,
                    "target": target,
                    "disabled": disabled
                })
            })
            .collect();

        // JS: return hasValidMove ? moves : [];
        // ✅ NOW IMPLEMENTED (Session 24 Part 63): Check if any move is valid
        for move_obj in &moves {
            if let Some(disabled) = move_obj.get("disabled") {
                if !disabled.as_bool().unwrap_or(true) {
                    has_valid_move = true;
                    break;
                }
            }
        }

        if has_valid_move {
            moves
        } else {
            vec![]
        }
    }
}
