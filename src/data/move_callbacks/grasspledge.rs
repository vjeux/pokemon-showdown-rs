//! Grass Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(target, source, move) {
///     if (['waterpledge', 'firepledge'].includes(move.sourceEffect)) {
///         this.add('-combine');
///         return 150;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (['waterpledge', 'firepledge'].includes(move.sourceEffect)) {
    let source_effect = battle.active_move.as_ref().and_then(|m| m.source_effect.clone());

    if let Some(ref effect_id) = source_effect {
        if effect_id == &ID::from("waterpledge") || effect_id == &ID::from("firepledge") {
            // this.add('-combine');
            battle.add("-combine", &[]);

            // return 150;
            return EventResult::Number(150);
        }
    }

    // return move.basePower;
    if let Some(ref active_move) = battle.active_move {
        return EventResult::Number(active_move.base_power);
    }

    EventResult::Continue
}

/// onPrepareHit(target, source, move) {
///     for (const action of this.queue.list as MoveAction[]) {
///         if (
///             !action.move || !action.pokemon?.isActive ||
///             action.pokemon.fainted || action.maxMove || action.zmove
///         ) {
///             continue;
///         }
///         if (action.pokemon.isAlly(source) && ['waterpledge', 'firepledge'].includes(action.move.id)) {
///             this.queue.prioritizeAction(action, move);
///             this.add('-waiting', source, action.pokemon);
///             return null;
///         }
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;

    // for (const action of this.queue.list as MoveAction[]) {
    // We need to iterate through the queue to find ally pledge moves
    let mut ally_pledge_action_index = None;

    for (i, action) in battle.queue.list.iter().enumerate() {
        // if (!action.move || !action.pokemon?.isActive || action.pokemon.fainted || action.maxMove || action.zmove) { continue; }
        if action.move_id.is_none() || action.pokemon.is_none() {
            continue;
        }

        let action_pokemon = action.pokemon.unwrap();

        // Check if pokemon is active
        let is_active = {
            if let Some(p) = battle.pokemon_at(action_pokemon.0, action_pokemon.1) {
                p.is_active
            } else {
                false
            }
        };

        if !is_active {
            continue;
        }

        // Check if pokemon is fainted
        let is_fainted = {
            if let Some(p) = battle.pokemon_at(action_pokemon.0, action_pokemon.1) {
                p.fainted
            } else {
                true
            }
        };

        if is_fainted {
            continue;
        }

        // Skip if maxMove or zmove
        if action.max_move || action.zmove {
            continue;
        }

        // if (action.pokemon.isAlly(source) && ['waterpledge', 'firepledge'].includes(action.move.id)) {
        let is_ally = action_pokemon.0 == source.0;

        if is_ally {
            if let Some(ref move_id) = action.move_id {
                if move_id == &ID::from("waterpledge") || move_id == &ID::from("firepledge") {
                    ally_pledge_action_index = Some(i);
                    break;
                }
            }
        }
    }

    if let Some(action_index) = ally_pledge_action_index {
        // this.queue.prioritizeAction(action, move);
        battle.queue.prioritize_action(action_index);

        // this.add('-waiting', source, action.pokemon);
        let source_arg = crate::battle::Arg::Pos(source.0, source.1);
        let ally_pos = battle.queue.list[action_index].pokemon.unwrap();
        let ally_arg = crate::battle::Arg::Pos(ally_pos.0, ally_pos.1);
        battle.add("-waiting", &[source_arg, ally_arg]);

        // return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.sourceEffect === 'waterpledge') {
///         move.type = 'Grass';
///         move.forceSTAB = true;
///         move.sideCondition = 'grasspledge';
///     }
///     if (move.sourceEffect === 'firepledge') {
///         move.type = 'Fire';
///         move.forceSTAB = true;
///         move.sideCondition = 'firepledge';
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (move.sourceEffect === 'waterpledge') {
    let source_effect = battle.active_move.as_ref().and_then(|m| m.source_effect.clone());

    if let Some(ref effect_id) = source_effect {
        if effect_id == &ID::from("waterpledge") {
            // move.type = 'Grass';
            // move.forceSTAB = true;
            // move.sideCondition = 'grasspledge';
            if let Some(ref current_move_id) = battle.active_move {
                if let Some(current_move) = battle.dex.get_move_by_id_mut(current_move_id) {
                    current_move.move_type = ID::from("grass");
                    current_move.force_stab = true;
                    current_move.side_condition = Some(ID::from("grasspledge"));
                }
            }
        }

        // if (move.sourceEffect === 'firepledge') {
        if effect_id == &ID::from("firepledge") {
            // move.type = 'Fire';
            // move.forceSTAB = true;
            // move.sideCondition = 'firepledge';
            if let Some(ref current_move_id) = battle.active_move {
                if let Some(current_move) = battle.dex.get_move_by_id_mut(current_move_id) {
                    current_move.move_type = ID::from("fire");
                    current_move.force_stab = true;
                    current_move.side_condition = Some(ID::from("firepledge"));
                }
            }
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'Grass Pledge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'Grass Pledge');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
            let side_arg = crate::battle::Arg::Side(side_index);
            battle.add("-sidestart", &[side_arg, "Grass Pledge".into()]);
                    }
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Grass Pledge');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'Grass Pledge');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
            let side_arg = crate::battle::Arg::Side(side_index);
            battle.add("-sideend", &[side_arg, "Grass Pledge".into()]);
                    }
        }

        EventResult::Continue
    }

    /// onModifySpe(spe, pokemon) {
    ///     return this.chainModify(0.25);
    /// }
    pub fn on_modify_spe(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // return this.chainModify(0.25);
        EventResult::Number(battle.chain_modify(0.25 as f32))
    }
}
