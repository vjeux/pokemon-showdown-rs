//! Mirror Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	mirrorarmor: {
//! 		onTryBoost(boost, target, source, effect) {
//! 			// Don't bounce self stat changes, or boosts that have already bounced
//! 			if (!source || target === source || !boost || effect.name === 'Mirror Armor') return;
//! 			let b: BoostID;
//! 			for (b in boost) {
//! 				if (boost[b]! < 0) {
//! 					if (target.boosts[b] === -6) continue;
//! 					const negativeBoost: SparseBoostsTable = {};
//! 					negativeBoost[b] = boost[b];
//! 					delete boost[b];
//! 					if (source.hp) {
//! 						this.add('-ability', target, 'Mirror Armor');
//! 						this.boost(negativeBoost, source, target, null, true);
//! 					}
//! 				}
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Mirror Armor",
//! 		rating: 2,
//! 		num: 240,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
/// Bounces negative stat changes back to source
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, _has_secondaries: bool) -> AbilityHandlerResult {
    // Don't bounce self stat changes, or boosts that have already bounced
    // if (!source || target === source || !boost || effect.name === 'Mirror Armor') return;
    if source.is_none() {
        return AbilityHandlerResult::Undefined;
    }

    let src = source.unwrap();
    if (target.side_index, target.position) == (src.side_index, src.position) {
        return AbilityHandlerResult::Undefined;
    }

    if effect_id == "mirrorarmor" {
        return AbilityHandlerResult::Undefined;
    }

    let mut showed_ability = false;

    // Collect stats to bounce (need to clone keys to avoid borrow issues)
    let stats: Vec<String> = boost.keys().cloned().collect();

    for stat in stats {
        if let Some(&boost_value) = boost.get(&stat) {
            // if (boost[b]! < 0)
            if boost_value < 0 {
                // if (target.boosts[b] === -6) continue;
                let current_boost = match stat.as_str() {
                    "atk" => target.boosts.atk,
                    "def" => target.boosts.def,
                    "spa" => target.boosts.spa,
                    "spd" => target.boosts.spd,
                    "spe" => target.boosts.spe,
                    "accuracy" => target.boosts.accuracy,
                    "evasion" => target.boosts.evasion,
                    _ => 0,
                };

                if current_boost == -6 {
                    continue;
                }

                // Create negative boost and remove from original
                // delete boost[b];
                boost.remove(&stat);

                // if (source.hp)
                if src.hp > 0 {
                    if !showed_ability {
                        // this.add('-ability', target, 'Mirror Armor');
                        battle.add("-ability", &[Arg::Pokemon(target), Arg::Str("Mirror Armor")]);
                        showed_ability = true;
                    }
                    // this.boost(negativeBoost, source, target, null, true);
                    battle.boost(&[(&stat, boost_value)], (src.side_index, src.position), Some((target.side_index, target.position)), None);
                }
            }
        }
    }

    AbilityHandlerResult::Undefined
}

