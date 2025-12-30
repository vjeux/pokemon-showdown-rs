use crate::*;
use crate::battle_actions::ZPowerResult;
use crate::dex_data::BoostsTable;

impl<'a> BattleActions<'a> {

    /// Run Z-Power effect for status Z-moves
    /// Equivalent to battle-actions.ts runZPower()
    //
    // 	runZPower(move: ActiveMove, pokemon: Pokemon) {
    // 		const zPower = this.dex.conditions.get('zpower');
    // 		if (move.category !== 'Status') {
    // 			this.battle.attrLastMove('[zeffect]');
    // 		} else if (move.zMove?.boost) {
    // 			this.battle.boost(move.zMove.boost, pokemon, pokemon, zPower);
    // 		} else if (move.zMove?.effect) {
    // 			switch (move.zMove.effect) {
    // 			case 'heal':
    // 				this.battle.heal(pokemon.maxhp, pokemon, pokemon, zPower);
    // 				break;
    // 			case 'healreplacement':
    // 				pokemon.side.addSlotCondition(pokemon, 'healreplacement', pokemon, move);
    // 				break;
    // 			case 'clearnegativeboost':
    // 				const boosts: SparseBoostsTable = {};
    // 				let i: BoostID;
    // 				for (i in pokemon.boosts) {
    // 					if (pokemon.boosts[i] < 0) {
    // 						boosts[i] = 0;
    // 					}
    // 				}
    // 				pokemon.setBoost(boosts);
    // 				this.battle.add('-clearnegativeboost', pokemon, '[zeffect]');
    // 				break;
    // 			case 'redirect':
    // 				pokemon.addVolatile('followme', pokemon, zPower);
    // 				break;
    // 			case 'crit2':
    // 				pokemon.addVolatile('focusenergy', pokemon, zPower);
    // 				break;
    // 			case 'curse':
    // 				if (pokemon.hasType('Ghost')) {
    // 					this.battle.heal(pokemon.maxhp, pokemon, pokemon, zPower);
    // 				} else {
    // 					this.battle.boost({ atk: 1 }, pokemon, pokemon, zPower);
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn run_z_power(
        move_category: &str,
        z_move_boost: Option<&BoostsTable>,
        z_move_effect: Option<&str>,
        pokemon_has_ghost_type: bool,
    ) -> ZPowerResult {
        if move_category != "Status" {
            // Damage Z-moves just add [zeffect] attribute
            return ZPowerResult::DamageMove;
        }

        // Status Z-moves can have boosts or effects
        if let Some(boosts) = z_move_boost {
            return ZPowerResult::Boost(*boosts);
        }

        if let Some(effect) = z_move_effect {
            match effect {
                "heal" => ZPowerResult::Heal,
                "healreplacement" => ZPowerResult::HealReplacement,
                "clearnegativeboost" => ZPowerResult::ClearNegativeBoost,
                "redirect" => ZPowerResult::Redirect,
                "crit2" => ZPowerResult::Crit2,
                "curse" => {
                    if pokemon_has_ghost_type {
                        ZPowerResult::Heal
                    } else {
                        ZPowerResult::Boost(BoostsTable {
                            atk: 1,
                            ..Default::default()
                        })
                    }
                }
                _ => ZPowerResult::None,
            }
        } else {
            ZPowerResult::None
        }
    }
}
