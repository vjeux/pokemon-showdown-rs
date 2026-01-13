//! Grass Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// basePowerCallback(target, source, move) {
///     if (['waterpledge', 'firepledge'].includes(move.sourceEffect)) {
///         this.add('-combine');
///         return 150;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (['waterpledge', 'firepledge'].includes(move.sourceEffect)) {
    let source_effect = battle
        .active_move
        .as_ref()
        .and_then(|m| m.source_effect.clone());

    if let Some(ref effect) = source_effect {
        if effect.as_str() == "waterpledge" || effect.as_str() == "firepledge" {
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
pub fn on_prepare_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // JS: onPrepareHit(target, source, move)
    // First param is target, second is the actual source (move user)
    let source = source_pos.unwrap_or(_target_pos);

    // for (const action of this.queue.list as MoveAction[]) {
    // We need to iterate through the queue to find ally pledge moves
    let mut ally_pledge_action_index = None;
    let mut ally_pokemon_pos = None;

    for (i, action) in battle.queue.list.iter().enumerate() {
        // Only process Move actions
        let move_action = match action {
            crate::battle_queue::Action::Move(ma) => ma,
            _ => continue,
        };

        // Skip if maxMove or zmove
        if move_action.max_move.is_some() || move_action.zmove.is_some() {
            continue;
        }

        // Get action pokemon position
        let action_pokemon = (move_action.side_index, move_action.pokemon_index);

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

        // if (action.pokemon.isAlly(source) && ['waterpledge', 'firepledge'].includes(action.move.id)) {
        let is_ally = action_pokemon.0 == source.0;

        if is_ally
            && (move_action.move_id == ID::from("waterpledge")
                || move_action.move_id == ID::from("firepledge"))
        {
            ally_pledge_action_index = Some(i);
            ally_pokemon_pos = Some(action_pokemon);
            break;
        }
    }

    if let Some(_action_index) = ally_pledge_action_index {
        // this.queue.prioritizeAction(action, move);
        let ally_pos = ally_pokemon_pos.unwrap();
        battle.queue.prioritize_action(ally_pos.0, ally_pos.1);

        // this.add('-waiting', source, action.pokemon);
        let source_ident = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        let ally_pos = ally_pokemon_pos.unwrap();
        let ally_ident = {
            let pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add(
            "-waiting",
            &[source_ident.as_str().into(), ally_ident.as_str().into()],
        );

        // return null;
        return EventResult::Null;
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
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (move.sourceEffect === 'waterpledge') {
    let source_effect = battle
        .active_move
        .as_ref()
        .and_then(|m| m.source_effect.clone());

    if let Some(ref effect) = source_effect {
        if effect.as_str() == "waterpledge" {
            // move.type = 'Grass';
            // move.forceSTAB = true;
            // move.sideCondition = 'grasspledge';
            if let Some(ref mut current_move) = battle.active_move {
                current_move.move_type = "Grass".to_string();
                current_move.force_stab = true;
                current_move.side_condition = Some("grasspledge".to_string());
            }
        }

        // if (move.sourceEffect === 'firepledge') {
        if effect.as_str() == "firepledge" {
            // move.type = 'Fire';
            // move.forceSTAB = true;
            // move.sideCondition = 'firepledge';
            if let Some(ref mut current_move) = battle.active_move {
                current_move.move_type = "Fire".to_string();
                current_move.force_stab = true;
                current_move.side_condition = Some("firepledge".to_string());
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
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Grass Pledge".into()]);
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Grass Pledge');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'Grass Pledge');
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "Grass Pledge".into()]);
        }

        EventResult::Continue
    }

    /// onModifySpe(spe, pokemon) {
    ///     return this.chainModify(0.25);
    /// }
    pub fn on_modify_spe(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // return this.chainModify(0.25);
        { battle.chain_modify(0.25_f32); EventResult::Continue }
    }
}
