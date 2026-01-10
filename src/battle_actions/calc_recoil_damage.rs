use crate::*;
use crate::battle_actions::ActiveMove;

impl<'a> BattleActions<'a> {

    /// Calculate recoil damage
    /// Equivalent to calcRecoilDamage in battle-actions.ts
    // TypeScript source:
    //
    //
    // 	calcRecoilDamage(damageDealt: number, move: Move, pokemon: Pokemon): number {
    // 		if (move.id === 'chloroblast') return Math.round(pokemon.maxhp / 2);
    // 		return this.battle.clampIntRange(Math.round(damageDealt * move.recoil![0] / move.recoil![1]), 1);
    // 	}
    //
    pub fn calc_recoil_damage(
        damage_dealt: i32,
        active_move: &ActiveMove,
        pokemon_max_hp: i32,
    ) -> i32 {
        // JS: if (move.id === 'chloroblast') return Math.round(pokemon.maxhp / 2);
        if active_move.id.as_str() == "chloroblast" {
            return (pokemon_max_hp as f64 / 2.0).round() as i32;
        }
        // JS: return this.battle.clampIntRange(Math.round(damageDealt * move.recoil![0] / move.recoil![1]), 1);
        if let Some((num, denom)) = active_move.recoil {
            // Use floating point math and round to match JavaScript's Math.round()
            let recoil_damage = ((damage_dealt as f64 * num as f64 / denom as f64).round() as i32).max(1);
            return recoil_damage;
        }
        0
    }
}
