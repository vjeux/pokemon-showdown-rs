//! BattleActions::selfDrops - Apply self stat drops after a move
//!
//! 1:1 port of selfDrops from battle-actions.ts

// JS Source:
// 	selfDrops(
// 		targets: SpreadMoveTargets, source: Pokemon,
// 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
// 	) {
// 		for (const target of targets) {
// 			if (target === false) continue;
// 			if (moveData.self && !move.selfDropped) {
// 				if (!isSecondary && moveData.self.boosts) {
// 					const secondaryRoll = this.battle.random(100);
// 					if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
// 						this.moveHit(source, source, move, moveData.self, isSecondary, true);
// 					}
// 					if (!move.multihit) move.selfDropped = true;
// 				} else {
// 					this.moveHit(source, source, move, moveData.self, isSecondary, true);
// 				}
// 			}
// 		}
// 	}

use crate::*;
use crate::battle_actions::{SpreadMoveTargets, SpreadMoveTarget};

/// Apply self stat drops after a move
/// Equivalent to selfDrops() in battle-actions.ts
///
/// JavaScript signature:
/// selfDrops(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean)
pub fn self_drops(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    _move_id: &ID,
    is_secondary: bool,
) {
    // Get moveData.self from active_move
    let (has_self_data, has_boosts, self_chance, is_multihit, self_dropped) = {
        if let Some(ref active_move) = battle.active_move {
            let has_self = active_move.self_data.is_some();
            let has_boosts = active_move.self_data.as_ref()
                .and_then(|s| s.boosts.as_ref())
                .is_some();
            let chance = active_move.self_data.as_ref()
                .and_then(|s| s.chance);
            let multihit = active_move.multi_hit.is_some();
            let dropped = active_move.self_dropped;
            (has_self, has_boosts, chance, multihit, dropped)
        } else {
            return;
        }
    };

    // for (const target of targets) {
    for target in targets {
        // if (target === false) continue;
        if matches!(target, SpreadMoveTarget::Failed) {
            continue;
        }

        // if (moveData.self && !move.selfDropped) {
        if has_self_data && !self_dropped {
            // if (!isSecondary && moveData.self.boosts) {
            if !is_secondary && has_boosts {
                // const secondaryRoll = this.battle.random(100);
                let secondary_roll = battle.random(100);

                // if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
                let should_apply = match self_chance {
                    None => true, // chance is undefined, always apply
                    Some(chance) => secondary_roll < chance,
                };

                if should_apply {
                    // this.moveHit(source, source, move, moveData.self, isSecondary, true);
                    // TODO: Implement moveHit function
                    // This would apply the self boosts to the source Pokemon
                    // For now, we can apply boosts directly
                    if let Some(ref active_move) = battle.active_move.clone() {
                        if let Some(ref self_data) = active_move.self_data {
                            if let Some(ref boosts) = self_data.boosts {
                                // Convert BoostsTable to array of (stat_name, value) tuples
                                let mut boost_array = Vec::new();
                                if boosts.atk != 0 {
                                    boost_array.push(("atk", boosts.atk));
                                }
                                if boosts.def != 0 {
                                    boost_array.push(("def", boosts.def));
                                }
                                if boosts.spa != 0 {
                                    boost_array.push(("spa", boosts.spa));
                                }
                                if boosts.spd != 0 {
                                    boost_array.push(("spd", boosts.spd));
                                }
                                if boosts.spe != 0 {
                                    boost_array.push(("spe", boosts.spe));
                                }
                                if boosts.accuracy != 0 {
                                    boost_array.push(("accuracy", boosts.accuracy));
                                }
                                if boosts.evasion != 0 {
                                    boost_array.push(("evasion", boosts.evasion));
                                }

                                battle.boost(&boost_array, source_pos, Some(source_pos), None, false, true);
                            }
                        }
                    }
                }

                // if (!move.multihit) move.selfDropped = true;
                if !is_multihit {
                    if let Some(ref mut active_move) = battle.active_move {
                        active_move.self_dropped = true;
                    }
                }
            } else {
                // this.moveHit(source, source, move, moveData.self, isSecondary, true);
                // TODO: Implement moveHit function
                // This would apply the self effects to the source Pokemon
                // For self-drop moves without boosts check, just apply directly
                if let Some(ref active_move) = battle.active_move.clone() {
                    if let Some(ref self_data) = active_move.self_data {
                        if let Some(ref boosts) = self_data.boosts {
                            let mut boost_array = Vec::new();
                            if boosts.atk != 0 {
                                boost_array.push(("atk", boosts.atk));
                            }
                            if boosts.def != 0 {
                                boost_array.push(("def", boosts.def));
                            }
                            if boosts.spa != 0 {
                                boost_array.push(("spa", boosts.spa));
                            }
                            if boosts.spd != 0 {
                                boost_array.push(("spd", boosts.spd));
                            }
                            if boosts.spe != 0 {
                                boost_array.push(("spe", boosts.spe));
                            }
                            if boosts.accuracy != 0 {
                                boost_array.push(("accuracy", boosts.accuracy));
                            }
                            if boosts.evasion != 0 {
                                boost_array.push(("evasion", boosts.evasion));
                            }

                            battle.boost(&boost_array, source_pos, Some(source_pos), None, false, true);
                        }
                    }
                }
            }
        }
    }
}
