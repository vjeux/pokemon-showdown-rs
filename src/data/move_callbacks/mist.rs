//! Mist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onTryBoost(boost, target, source, effect) {
    ///     if (effect.effectType === 'Move' && effect.infiltrates && !target.isAlly(source)) return;
    ///     if (source && target !== source) {
    ///         let showMsg = false;
    ///         let i: BoostID;
    ///         for (i in boost) {
    ///             if (boost[i]! < 0) {
    ///                 delete boost[i];
    ///                 showMsg = true;
    ///             }
    ///         }
    ///         if (showMsg && !(effect as ActiveMove).secondaries) {
    ///             this.add('-activate', target, 'move: Mist');
    ///         }
    ///     }
    /// }
    pub fn on_try_boost(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // if (effect.effectType === 'Move' && effect.infiltrates && !target.isAlly(source)) return;
        if let (Some(source), Some(target)) = (source_pos, target_pos) {
            // Check if the effect is a Move with infiltrates property
            let has_infiltrates = battle.active_move.as_ref()
                .map(|m| m.infiltrates)
                .unwrap_or(false);

            if has_infiltrates {
                // Check if target is NOT an ally of source
                let is_ally = battle.is_ally(target, source);
                if !is_ally {
                    // Infiltrating move bypasses Mist protection
                    return EventResult::Continue;
                }
            }
        }

        // if (source && target !== source) {
        if let (Some(source), Some(target)) = (source_pos, target_pos) {
            if target == source {
                return EventResult::Continue;
            }

            // let showMsg = false;
            // for (i in boost) {
            //     if (boost[i]! < 0) {
            //         delete boost[i];
            //         showMsg = true;
            //     }
            // }
            let mut show_msg = false;
            if let Some(ref mut event) = battle.current_event {
                if let Some(EventResult::Boost(ref mut boosts)) = event.relay_var {
                    if boosts.atk < 0 { boosts.atk = 0; show_msg = true; }
                    if boosts.def < 0 { boosts.def = 0; show_msg = true; }
                    if boosts.spa < 0 { boosts.spa = 0; show_msg = true; }
                    if boosts.spd < 0 { boosts.spd = 0; show_msg = true; }
                    if boosts.spe < 0 { boosts.spe = 0; show_msg = true; }
                    if boosts.accuracy < 0 { boosts.accuracy = 0; show_msg = true; }
                    if boosts.evasion < 0 { boosts.evasion = 0; show_msg = true; }
                }
            }

            // if (showMsg && !(effect as ActiveMove).secondaries) {
            //     this.add('-activate', target, 'move: Mist');
            // }
            if show_msg {
                let has_secondaries = battle.active_move.as_ref()
                    .map(|m| !m.secondaries.is_empty())
                    .unwrap_or(false);

                if !has_secondaries {
                    let target_arg = {
                        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        target_pokemon.get_slot()
                    };

                    battle.add("-activate", &[target_arg.into(), "move: Mist".into()]);
                }
            }
        }

        EventResult::Continue
    }

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'Mist');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'Mist');
        // Following the pattern from gmaxcannonade.rs - access side via with_effect_state_ref
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Mist".into()]);
        }

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'Mist');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'Mist');
        // Following the pattern from gmaxcannonade.rs - access side via with_effect_state_ref
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "Mist".into()]);
        }

        EventResult::Continue
    }
}
