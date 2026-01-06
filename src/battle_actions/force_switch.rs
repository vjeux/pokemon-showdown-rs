//! BattleActions::forceSwitch - Handle forced switching from moves
//!
//! 1:1 port of forceSwitch from battle-actions.ts

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::{SpreadMoveDamage, DamageResult, SpreadMoveTargets, SpreadMoveTarget, ActiveMove};

/// Handle forced switching from moves like Dragon Tail, Roar
/// Equivalent to battle-actions.ts forceSwitch()
///
/// forceSwitch(
///     damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove
/// ) {
///     for (const [i, target] of targets.entries()) {
///         if (target && target.hp > 0 && source.hp > 0 && this.battle.canSwitch(target.side)) {
///             const hitResult = this.battle.runEvent('DragOut', target, source, move);
///             if (hitResult) {
///                 target.forceSwitchFlag = true;
///             } else if (hitResult === false && move.category === 'Status') {
///                 this.battle.add('-fail', source);
///                 this.battle.attrLastMove('[still]');
///                 damage[i] = false;
///             }
///         }
///     }
///     return damage;
/// }
pub fn force_switch(
    battle: &mut Battle,
    mut damage: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    active_move: &ActiveMove,
) -> SpreadMoveDamage {
    eprintln!("[FORCE_SWITCH] Entry: move={}, source={:?}, targets.len()={}", active_move.id, source_pos, targets.len());
    // for (const [i, target] of targets.entries()) {
    for (i, target) in targets.iter().enumerate() {
        // if (target && target.hp > 0 && source.hp > 0 && this.battle.canSwitch(target.side)) {
        match target {
            SpreadMoveTarget::Target(target_pos_tuple) => {
                let target_pos = *target_pos_tuple; // Dereference to get (usize, usize)
                // Check conditions in two phases to avoid borrow issues
                let (target_hp, source_hp) = {
                    let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    (target_pokemon.hp, source_pokemon.hp)
                };

                if target_hp > 0 && source_hp > 0 {
                    // Check if can switch (returns count of possible switches)
                    let can_switch = battle.can_switch(target_pos.0) > 0;

                    if can_switch {
                        //     const hitResult = this.battle.runEvent('DragOut', target, source, move);
                        let hit_result = battle.run_event("DragOut", Some(crate::event::EventTarget::Pokemon(target_pos)), Some(source_pos), Some(&active_move.id), EventResult::Continue, false, false);

                        //     if (hitResult) {
                        //         target.forceSwitchFlag = true;
                        //     } else if (hitResult === false && move.category === 'Status') {
                        //         this.battle.add('-fail', source);
                        //         this.battle.attrLastMove('[still]');
                        //         damage[i] = false;
                        //     }
                        eprintln!("[FORCE_SWITCH] hit_result={:?}", hit_result);
                        if !matches!(hit_result, EventResult::Null | EventResult::Continue) && !matches!(hit_result, EventResult::Number(0)) {
                            // hitResult is truthy (not None, not 0)
                            eprintln!("[FORCE_SWITCH] Setting force_switch_flag=true for target {:?}", target_pos);
                            if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                                target_pokemon.force_switch_flag = true;
                                eprintln!("[FORCE_SWITCH] Successfully set force_switch_flag");
                            }
                        } else if matches!(hit_result, EventResult::Number(0)) && active_move.category == "Status" {
                            // hitResult is false (0)
                            // Get source pokemon for add message
                            if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                                // Create a temporary owned string for the identifier
                                let source_ident = format!(
                                    "p{}a: {}",
                                    source_pos.0 + 1,
                                    source_pokemon.set.species
                                );
                                battle.add("-fail", &[
                                    crate::battle::Arg::String(source_ident),
                                ]);
                                battle.attr_last_move(&["[still]"]);
                                if i < damage.len() {
                                    damage[i] = DamageResult::Failed;
                                }
                            }
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    // return damage;
    damage
}
