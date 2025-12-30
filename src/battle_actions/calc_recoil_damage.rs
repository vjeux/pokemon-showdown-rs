use crate::*;

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
        move_id: &str,
        recoil: Option<(i32, i32)>,
        pokemon_max_hp: i32,
    ) -> i32 {
        if move_id == "chloroblast" {
            return (pokemon_max_hp / 2).max(1);
        }
        if let Some((num, denom)) = recoil {
            let recoil_damage = (damage_dealt * num / denom).max(1);
            return recoil_damage;
        }
        0
    }
}
