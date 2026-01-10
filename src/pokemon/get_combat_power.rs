// JS Source:
// 	getCombatPower() {
// 		let statSum = 0;
// 		let awakeningSum = 0;
// 		for (const stat in this.stats) {
// 			statSum += this.calculateStat(stat, this.boosts[stat as BoostName]);
// 			awakeningSum += this.calculateStat(
// 				stat, this.boosts[stat as BoostName]) + this.set.evs[stat];
// 		}
// 		const combatPower = Math.floor(Math.floor(statSum * this.level * 6 / 100) +
// 			(Math.floor(awakeningSum) * Math.floor((this.level * 4) / 100 + 2)));
// 		return this.battle.clampIntRange(combatPower, 0, 10000);
// 	}


use crate::*;
use crate::dex_data::{StatID, BoostID};

impl Battle {

    /// Get combat power (for Pokemon Go style formats)
    /// Equivalent to getCombatPower in pokemon.ts
    /// ✅ NOW IMPLEMENTED: 1-to-1 with JavaScript (was using wrong formula)
    pub fn get_combat_power(&mut self, pokemon_pos: (usize, usize)) -> i32 {
        // Get pokemon data we need first
        let (level, boosts, evs) = {
            let pokemon = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return 0,
            };
            (pokemon.level, pokemon.boosts.clone(), pokemon.set.evs.clone())
        };

        // JS: let statSum = 0;
        // JS: let awakeningSum = 0;
        let mut stat_sum = 0;
        let mut awakening_sum = 0;

        // JS: for (const stat in this.stats) {
        // JS:     statSum += this.calculateStat(stat, this.boosts[stat as BoostName]);
        // JS:     awakeningSum += this.calculateStat(stat, this.boosts[stat as BoostName]) + this.set.evs[stat];
        // JS: }
        for stat_id in StatID::all() {
            // HP doesn't have a boost, so use 0 for HP
            let boost = match stat_id {
                StatID::HP => 0,
                StatID::Atk => boosts.get(BoostID::Atk),
                StatID::Def => boosts.get(BoostID::Def),
                StatID::SpA => boosts.get(BoostID::SpA),
                StatID::SpD => boosts.get(BoostID::SpD),
                StatID::Spe => boosts.get(BoostID::Spe),
            };

            // Calculate stat with boost
            let calculated = self.calculate_stat(pokemon_pos, *stat_id, boost, 1.0, None);

            stat_sum += calculated;

            // For awakening sum, add EV value
            // JS: this.set.evs[stat]
            let ev_value = evs.get(*stat_id);
            awakening_sum += calculated + ev_value;
        }

        // JS: const combatPower = Math.floor(Math.floor(statSum * this.level * 6 / 100) +
        // JS:     (Math.floor(awakeningSum) * Math.floor((this.level * 4) / 100 + 2)));
        let level = level as i32;
        let combat_power =
            ((stat_sum * level * 6) / 100) +
            (awakening_sum * ((level * 4) / 100 + 2));

        // JS: return this.battle.clampIntRange(combatPower, 0, 10000);
        self.clamp_int_range(combat_power, Some(0), Some(10000))
    }
}
