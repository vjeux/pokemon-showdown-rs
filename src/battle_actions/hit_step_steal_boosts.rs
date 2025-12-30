use crate::*;
use crate::dex_data::BoostsTable;

impl<'a> BattleActions<'a> {

    /// Check if move steals boosts
    /// Equivalent to hitStepStealBoosts in battle-actions.ts
    // 	hitStepStealBoosts(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const target = targets[0]; // hardcoded
    // 		if (move.stealsBoosts) {
    // 			const boosts: SparseBoostsTable = {};
    // 			let stolen = false;
    // 			let statName: BoostID;
    // 			for (statName in target.boosts) {
    // 				const stage = target.boosts[statName];
    // 				if (stage > 0) {
    // 					boosts[statName] = stage;
    // 					stolen = true;
    // 				}
    // 			}
    // 			if (stolen) {
    // 				this.battle.attrLastMove('[still]');
    // 				this.battle.add('-clearpositiveboost', target, pokemon, 'move: ' + move.name);
    // 				this.battle.boost(boosts, pokemon, pokemon);
    //
    // 				let statName2: BoostID;
    // 				for (statName2 in boosts) {
    // 					boosts[statName2] = 0;
    // 				}
    // 				target.setBoost(boosts);
    // 				if (move.id === "spectralthief") {
    // 					this.battle.addMove('-anim', pokemon, "Spectral Thief", target);
    // 				}
    // 			}
    // 		}
    // 		return undefined;
    // 	}
    //
    pub fn hit_step_steal_boosts(
        move_steals_boosts: bool,
        target_boosts: &BoostsTable,
    ) -> Option<BoostsTable> {
        if !move_steals_boosts {
            return None;
        }

        // Copy only positive boosts
        let mut stolen = BoostsTable::default();
        if target_boosts.atk > 0 {
            stolen.atk = target_boosts.atk;
        }
        if target_boosts.def > 0 {
            stolen.def = target_boosts.def;
        }
        if target_boosts.spa > 0 {
            stolen.spa = target_boosts.spa;
        }
        if target_boosts.spd > 0 {
            stolen.spd = target_boosts.spd;
        }
        if target_boosts.spe > 0 {
            stolen.spe = target_boosts.spe;
        }
        if target_boosts.accuracy > 0 {
            stolen.accuracy = target_boosts.accuracy;
        }
        if target_boosts.evasion > 0 {
            stolen.evasion = target_boosts.evasion;
        }

        Some(stolen)
    }
}
