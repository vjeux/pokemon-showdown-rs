//! Water Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// basePowerCallback(target, source, move) {
///     if (['firepledge', 'grasspledge'].includes(move.sourceEffect)) {
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
    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if sourceEffect is 'firepledge' or 'grasspledge'
    let active_move_ref = active_move.borrow();
    if let Some(ref source_effect) = active_move_ref.source_effect {
        let source_str = source_effect.as_str();
        if source_str == "firepledge" || source_str == "grasspledge" {
            // Note: JS has this.add('-combine') which we don't have infrastructure for yet
            // this.add('-combine');
            return EventResult::Number(150);
        }
    }

    // Get the move data for base power
    let move_data = match battle.dex.moves().get_by_id(&active_move_ref.id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    EventResult::Number(move_data.base_power)
}

/// onPrepareHit(target, source, move) {
///     for (const action of this.queue) {
///         if (action.choice !== 'move') continue;
///         const otherMove = action.move;
///         const otherMoveUser = action.pokemon;
///         if (
///             !otherMove || !action.pokemon || !otherMoveUser.isActive ||
///             otherMoveUser.fainted || action.maxMove || action.zmove
///         ) {
///             continue;
///         }
///         if (otherMoveUser.isAlly(source) && ['firepledge', 'grasspledge'].includes(otherMove.id)) {
///             this.queue.prioritizeAction(action, move);
///             this.add('-waiting', source, otherMoveUser);
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

    // Find ally pledge move in queue
    // Store the indices to avoid borrow checker issues
    let ally_pledge_action: Option<(usize, usize)> = {
        let mut result = None;
        // for (const action of this.queue)
        for action in &battle.queue.list {
            // if (action.choice !== 'move') continue;
            // Only process Move actions
            let move_action = match action {
                crate::battle_queue::Action::Move(ma) => ma,
                _ => continue,
            };

            // if (!otherMove || !action.pokemon || !otherMoveUser.isActive || otherMoveUser.fainted || action.maxMove || action.zmove)
            let (pokemon_is_active, pokemon_fainted) = {
                let pokemon = match battle.pokemon_at(move_action.side_index, move_action.pokemon_index) {
                    Some(p) => p,
                    None => continue, // No pokemon, skip
                };
                (pokemon.is_active, pokemon.fainted)
            };

            // Check all the skip conditions
            if !pokemon_is_active || pokemon_fainted || move_action.max_move.is_some() || move_action.zmove.is_some() {
                continue;
            }

            // if (otherMoveUser.isAlly(source) && ['firepledge', 'grasspledge'].includes(otherMove.id))
            let is_ally = Pokemon::is_ally(battle, (move_action.side_index, move_action.pokemon_index), source.0);

            let move_id_str = move_action.move_id.as_str();
            if is_ally && (move_id_str == "firepledge" || move_id_str == "grasspledge") {
                // Found ally pledge move, store indices and break
                result = Some((move_action.side_index, move_action.pokemon_index));
                break;
            }
        }
        result
    };

    // If we found an ally pledge action, prioritize it and add waiting message
    if let Some((ally_side_index, ally_pokemon_index)) = ally_pledge_action {
        // this.queue.prioritizeAction(action, move);
        battle.queue.prioritize_action(ally_side_index, ally_pokemon_index);

        // this.add('-waiting', source, otherMoveUser);
        let (source_slot, action_pokemon_slot) = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let action_pokemon = match battle.pokemon_at(ally_side_index, ally_pokemon_index) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (source_pokemon.get_slot(), action_pokemon.get_slot())
        };

        battle.add(
            "-waiting",
            &[
                crate::battle::Arg::from(source_slot),
                crate::battle::Arg::from(action_pokemon_slot),
            ],
        );

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.sourceEffect === 'grasspledge') {
///         move.type = 'Grass';
///         move.forceSTAB = true;
///         move.sideCondition = 'grasspledge';
///     }
///     if (move.sourceEffect === 'firepledge') {
///         move.type = 'Water';
///         move.forceSTAB = true;
///         move.self = { sideCondition: 'waterpledge' };
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.sourceEffect === 'grasspledge')
    let source_effect = active_move.borrow().source_effect.clone();
    if let Some(ref effect) = source_effect {
        if effect.as_str() == "grasspledge" {
            // move.type = 'Grass';
            active_move.borrow_mut().move_type = "Grass".to_string();
            // move.forceSTAB = true;
            active_move.borrow_mut().force_stab = true;
            // move.sideCondition = 'grasspledge';
            active_move.borrow_mut().side_condition = Some("grasspledge".to_string());
        } else if effect.as_str() == "firepledge" {
            // move.type = 'Water';
            active_move.borrow_mut().move_type = "Water".to_string();
            // move.forceSTAB = true;
            active_move.borrow_mut().force_stab = true;
            // move.self = { sideCondition: 'waterpledge' };
            active_move.borrow_mut().self_effect = Some(crate::dex::MoveSecondary {
                side_condition: Some("waterpledge".to_string()),
                ..Default::default()
            });
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'Water Pledge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'Water Pledge');
        // Following the pattern from gmaxcannonade.rs - access side via with_effect_state_ref
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Water Pledge".into()]);
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Water Pledge');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'Water Pledge');
        // Following the pattern from gmaxcannonade.rs - access side via with_effect_state_ref
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "Water Pledge".into()]);
        }

        EventResult::Continue
    }

    /// onModifyMove(move, pokemon) {
    ///     if (move.secondaries && move.id !== 'secretpower') {
    ///         this.debug('doubling secondary chance');
    ///         for (const secondary of move.secondaries) {
    ///             if (pokemon.hasAbility('serenegrace') && secondary.volatileStatus === 'flinch') continue;
    ///             if (secondary.chance) secondary.chance *= 2;
    ///         }
    ///         if (move.self?.chance) move.self.chance *= 2;
    ///     }
    /// }
    pub fn on_modify_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // Check if pokemon has Serene Grace ability (before getting mutable reference)
        let has_serene_grace = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_ability(battle, &["serenegrace"])
        };

        // Get the active move - first check if we need to double secondaries
        let should_double_secondaries = {
            if let Some(ref active_move) = battle.active_move {
                let am = active_move.borrow();
                !am.secondaries.is_empty() && am.id.as_str() != "secretpower"
            } else {
                false
            }
        };

        // if (move.secondaries && move.id !== 'secretpower')
        if should_double_secondaries {
            // this.debug('doubling secondary chance');
            battle.debug("doubling secondary chance");

            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };

            // for (const secondary of move.secondaries)
            let mut active_move_mut = active_move.borrow_mut();
            for secondary in &mut active_move_mut.secondaries {
                // if (pokemon.hasAbility('serenegrace') && secondary.volatileStatus === 'flinch') continue;
                if has_serene_grace
                    && secondary.volatile_status.as_deref() == Some("flinch")
                {
                    continue;
                }

                // if (secondary.chance) secondary.chance *= 2;
                if let Some(ref mut chance) = secondary.chance {
                    *chance *= 2;
                }
            }

            // if (move.self?.chance) move.self.chance *= 2;
            if let Some(ref mut self_effect) = active_move_mut.self_effect {
                if let Some(ref mut chance) = self_effect.chance {
                    *chance *= 2;
                }
            }
        }

        EventResult::Continue
    }
}
