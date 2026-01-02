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
    pub fn get_moves(&self) -> Vec<String> {
        // JS: if (lockedMove) {
        // JS:     lockedMove = toID(lockedMove);
        // JS:     this.trapped = true;
        // Note: Missing lockedMove parameter handling

        // JS:     if (lockedMove === 'recharge') {
        // JS:         return [{ move: 'Recharge', id: 'recharge' as ID }];
        // JS:     }
        // Note: Missing Recharge special case

        // JS:     for (const moveSlot of this.moveSlots) {
        // JS:         if (moveSlot.id !== lockedMove) continue;
        // JS:         return [{ move: moveSlot.move, id: moveSlot.id }];
        // JS:     }
        // Note: Missing locked move slot search and early return

        // JS:     return [{ move: this.battle.dex.moves.get(lockedMove).name, id: lockedMove }];
        // Note: Missing fallback to dex lookup for locked move

        // JS: const moves = [];
        // JS: let hasValidMove = false;
        // Note: Missing hasValidMove tracking and empty array return

        // JS: for (const moveSlot of this.moveSlots) {
        // JS:     let moveName = moveSlot.move;
        self.move_slots
            .iter()
            .map(|slot| {
                // JS:     if (moveSlot.id === 'hiddenpower') {
                // JS:         moveName = `Hidden Power ${this.hpType}`;
                // JS:         if (this.battle.gen < 6) moveName += ` ${this.hpPower}`;
                // JS:     }
                // Note: Missing Hidden Power type/power formatting

                // JS:     else if (moveSlot.id === 'return' || moveSlot.id === 'frustration') {
                // JS:         const basePowerCallback = this.battle.dex.moves.get(moveSlot.id).basePowerCallback as (pokemon: Pokemon) => number;
                // JS:         moveName += ` ${basePowerCallback(this)}`;
                // JS:     }
                // Note: Missing Return/Frustration power calculation

                // JS:     let target = moveSlot.target;
                // JS:     switch (moveSlot.id) {
                // JS:     case 'curse':
                // JS:         if (!this.hasType('Ghost')) target = this.battle.dex.moves.get('curse').nonGhostTarget;
                // JS:         break;
                // JS:     case 'pollenpuff':
                // JS:         if (this.volatiles['healblock']) target = 'adjacentFoe';
                // JS:         break;
                // JS:     case 'terastarstorm':
                // JS:         if (this.species.name === 'Terapagos-Stellar') target = 'allAdjacentFoes';
                // JS:         break;
                // JS:     }
                // Note: Missing target overrides for Curse, Pollen Puff, Tera Star Storm

                // JS:     let disabled = moveSlot.disabled;
                // JS:     if (this.volatiles['dynamax']) {
                // JS:         const canCauseStruggle = ['Encore', 'Disable', 'Taunt', 'Assault Vest', 'Belch', 'Stuff Cheeks'];
                // JS:         disabled = this.maxMoveDisabled(moveSlot.id) || disabled && canCauseStruggle.includes(moveSlot.disabledSource!);
                // JS:     } else if (moveSlot.pp <= 0 && !this.volatiles['partialtrappinglock']) {
                // JS:         disabled = true;
                // JS:     }
                // Note: Missing disabled calculation (Dynamax, PP, partial trapping lock)

                // JS:     if (disabled === 'hidden') disabled = !restrictData;
                // Note: Missing restrictData parameter and hidden disabled handling

                // JS:     if (!disabled) hasValidMove = true;
                // Note: Missing hasValidMove tracking

                // JS:     moves.push({
                // JS:         move: moveName,
                // JS:         id: moveSlot.id,
                // JS:         pp: moveSlot.pp,
                // JS:         maxpp: moveSlot.maxpp,
                // JS:         target,
                // JS:         disabled,
                // JS:     });
                // Note: Currently returns just ID as string
                // Note: Should return object with move, id, pp, maxpp, target, disabled

                slot.id.as_str().to_string()
            })
            .collect()

        // JS: return hasValidMove ? moves : [];
        // Note: Should return empty array if no valid moves
    }
}
