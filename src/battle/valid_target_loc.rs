use crate::*;

impl Battle {

    /// Check if a target location is valid
    /// Equivalent to battle.ts validTargetLoc()
    // TypeScript source:
    // /**
    // 	 * Returns whether a proposed target for a move is valid.
    // 	 */
    // 	validTargetLoc(targetLoc: number, source: Pokemon, targetType: string) {
    // 		if (targetLoc === 0) return true;
    // 		const numSlots = this.activePerHalf;
    // 		const sourceLoc = source.getLocOf(source);
    // 		if (Math.abs(targetLoc) > numSlots) return false;
    // 		const isSelf = (sourceLoc === targetLoc);
    // 		const isFoe = (this.gameType === 'freeforall' ? !isSelf : targetLoc > 0);
    // 		const acrossFromTargetLoc = -(numSlots + 1 - targetLoc);
    // 		const isAdjacent = (targetLoc > 0 ?
    // 			Math.abs(acrossFromTargetLoc - sourceLoc) <= 1 :
    // 			Math.abs(targetLoc - sourceLoc) === 1);
    //
    // 		if (this.gameType === 'freeforall' && targetType === 'adjacentAlly') {
    // 			// moves targeting one ally can instead target foes in Battle Royal
    // 			return isAdjacent;
    // 		}
    //
    // 		switch (targetType) {
    // 		case 'randomNormal':
    // 		case 'scripted':
    // 		case 'normal':
    // 			return isAdjacent;
    // 		case 'adjacentAlly':
    // 			return isAdjacent && !isFoe;
    // 		case 'adjacentAllyOrSelf':
    // 			return isAdjacent && !isFoe || isSelf;
    // 		case 'adjacentFoe':
    // 			return isAdjacent && isFoe;
    // 		case 'any':
    // 			return !isSelf;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn valid_target_loc(
        &self,
        target_loc: i32,
        source: (usize, usize),
        target_type: &str,
    ) -> bool {
        // JS: if (targetLoc === 0) return true;
        if target_loc == 0 {
            return true;
        }

        // JS: const numSlots = this.activePerHalf;
        let num_slots = self.active_per_half as i32;

        // JS: const sourceLoc = source.getLocOf(source);
        let source_loc = if let Some(source_pokemon) = self.sides.get(source.0).and_then(|s| s.pokemon.get(source.1)) {
            source_pokemon.get_loc_of(source.0, source.1, self.active_per_half) as i32
        } else {
            0
        };

        // JS: if (Math.abs(targetLoc) > numSlots) return false;
        if target_loc.abs() > num_slots {
            return false;
        }

        // JS: const isSelf = (sourceLoc === targetLoc);
        let is_self = source_loc == target_loc;

        // JS: const isFoe = (this.gameType === 'freeforall' ? !isSelf : targetLoc > 0);
        let is_foe = if self.game_type == GameType::FreeForAll {
            !is_self
        } else {
            target_loc > 0
        };

        // JS: const acrossFromTargetLoc = -(numSlots + 1 - targetLoc);
        let across_from_target_loc = -(num_slots + 1 - target_loc);

        // JS: const isAdjacent = (targetLoc > 0 ?
        //        Math.abs(acrossFromTargetLoc - sourceLoc) <= 1 :
        //        Math.abs(targetLoc - sourceLoc) === 1);
        let is_adjacent = if target_loc > 0 {
            (across_from_target_loc - source_loc).abs() <= 1
        } else {
            (target_loc - source_loc).abs() == 1
        };

        // JS: if (this.gameType === 'freeforall' && targetType === 'adjacentAlly') {
        //       return isAdjacent;
        //     }
        if self.game_type == GameType::FreeForAll && target_type == "adjacentAlly" {
            return is_adjacent;
        }

        // JS: switch (targetType)
        match target_type {
            "randomNormal" | "scripted" | "normal" => {
                // JS: return isAdjacent;
                is_adjacent
            }
            "adjacentAlly" => {
                // JS: return isAdjacent && !isFoe;
                is_adjacent && !is_foe
            }
            "adjacentAllyOrSelf" => {
                // JS: return isAdjacent && !isFoe || isSelf;
                (is_adjacent && !is_foe) || is_self
            }
            "adjacentFoe" => {
                // JS: return isAdjacent && isFoe;
                is_adjacent && is_foe
            }
            "any" => {
                // JS: return !isSelf;
                !is_self
            }
            _ => {
                // JS: return false;
                false
            }
        }
    }
}
