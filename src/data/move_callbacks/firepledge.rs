//! Fire Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(target, source, move) {
///     if (['grasspledge', 'waterpledge'].includes(move.sourceEffect)) {
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

    // Check if sourceEffect is 'grasspledge' or 'waterpledge'
    if let Some(ref source_effect) = active_move.source_effect {
        let source_str = source_effect.as_str();
        if source_str == "grasspledge" || source_str == "waterpledge" {
            // Note: JS has this.add('-combine') which we don't have infrastructure for yet
            // this.add('-combine');
            return EventResult::Number(150);
        }
    }

    // Get the move data for base power
    let move_data = match battle.dex.get_move_by_id(&active_move.id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    EventResult::Number(move_data.base_power)
}

/// onPrepareHit(target, source, move) {
///     for (const action of this.queue.list as MoveAction[]) {
///         if (
///             !action.move || !action.pokemon?.isActive ||
///             action.pokemon.fainted || action.maxMove || action.zmove
///         ) {
///             continue;
///         }
///         if (action.pokemon.isAlly(source) && ['grasspledge', 'waterpledge'].includes(action.move.id)) {
///             this.queue.prioritizeAction(action, move);
///             this.add('-waiting', source, action.pokemon);
///             return null;
///         }
///     }
/// }
pub fn on_prepare_hit(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.sourceEffect === 'waterpledge') {
///         move.type = 'Water';
///         move.forceSTAB = true;
///         move.self = { sideCondition: 'waterpledge' };
///     }
///     if (move.sourceEffect === 'grasspledge') {
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
    // Get the active move
    let active_move = match &mut battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.sourceEffect === 'waterpledge')
    if let Some(ref source_effect) = active_move.source_effect {
        if source_effect.as_str() == "waterpledge" {
            // move.type = 'Water';
            active_move.move_type = "Water".to_string();
            // move.forceSTAB = true;
            active_move.force_stab = true;
            // move.self = { sideCondition: 'waterpledge' };
            active_move.self_effect = Some(crate::battle_actions::SelfEffect {
                boosts: None,
                chance: None,
                side_condition: Some("waterpledge".to_string()),
            });
        } else if source_effect.as_str() == "grasspledge" {
            // move.type = 'Fire';
            active_move.move_type = "Fire".to_string();
            // move.forceSTAB = true;
            active_move.force_stab = true;
            // move.sideCondition = 'firepledge';
            active_move.side_condition = Some("firepledge".to_string());
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'Fire Pledge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'Fire Pledge');
        // Following the pattern from gmaxcannonade.rs - access side via current_effect_state
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sidestart", &[side_arg, "Fire Pledge".into()]);
            }
        }

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     if (!pokemon.hasType('Fire')) this.damage(pokemon.baseMaxhp / 8, pokemon);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // if (!pokemon.hasType('Fire'))
        let (has_fire, base_maxhp) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (pokemon.has_type("Fire"), pokemon.base_maxhp)
        };

        if !has_fire {
            // this.damage(pokemon.baseMaxhp / 8, pokemon);
            let damage_amount = base_maxhp / 8;
            use crate::dex_data::ID;
            battle.damage(damage_amount, Some(pokemon_pos), None, Some(&ID::from("firepledge")), false);
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Fire Pledge');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'Fire Pledge');
        // Following the pattern from gmaxcannonade.rs - access side via current_effect_state
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sideend", &[side_arg, "Fire Pledge".into()]);
            }
        }

        EventResult::Continue
    }
}
