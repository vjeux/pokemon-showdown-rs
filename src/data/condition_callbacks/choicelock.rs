//! Choicelock Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(pokemon) {
///     if (!this.activeMove) throw new Error("Battle.activeMove is null");
///     if (!this.activeMove.id || this.activeMove.hasBounced || this.activeMove.sourceEffect === 'snatch') return false;
///     this.effectState.move = this.activeMove.id;
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // if (!this.activeMove) throw new Error("Battle.activeMove is null");
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => {
            debug_elog!("[ERROR] Battle.activeMove is null in choicelock onStart");
            return EventResult::Continue;
        }
    };

    // if (!this.activeMove.id || this.activeMove.hasBounced || this.activeMove.sourceEffect === 'snatch') return false;
    if active_move.id.as_str().is_empty() || active_move.has_bounced ||
       active_move.source_effect.as_ref().map(|se| se.as_str() == "snatch").unwrap_or(false) {
        return EventResult::Boolean(false);
    }

    // this.effectState.move = this.activeMove.id;
    // JavaScript: this.effectState.move = ...
    let move_id = active_move.id.to_string();

    battle.with_effect_state(|state| {
        state.move_id = Some(move_id);
    });

    EventResult::Continue
}

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeMove(pokemon, target, move) {
///     if (!pokemon.getItem().isChoice) {
///         pokemon.removeVolatile('choicelock');
///         return;
///     }
///     if (
///         !pokemon.ignoringItem() && !pokemon.volatiles['dynamax'] &&
///         move.id !== this.effectState.move && move.id !== 'struggle'
///     ) {
///         // Fails unless the Choice item is being ignored, and no PP is lost
///         this.addMove('move', pokemon, move.name);
///         this.attrLastMove('[still]');
///         this.debug("Disabled by Choice item lock");
///         this.add('-fail', pokemon);
///         return false;
///     }
/// }
/// ```
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (!pokemon.getItem().isChoice)
    let is_choice = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.items().get_by_id(pokemon.get_item())
            .map(|item| item.is_choice)
            .unwrap_or(false)
    };

    if !is_choice {
        // pokemon.removeVolatile('choicelock');
        let choicelock_id = ID::from("choicelock");
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &choicelock_id);
        return EventResult::Continue;
    }

    // Get move.id and effectState.move
    let (move_id, locked_move_id) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let active_move_id = active_move_ref.id.to_string();

        let choicelock_id = ID::from("choicelock");
        let locked_move = pokemon.volatiles.get(&choicelock_id)
            .and_then(|v| v.move_id.clone())
            .unwrap_or_default();

        (active_move_id, locked_move)
    };

    // Check conditions
    let ignoring_item = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ignoring_item(battle, false)
    };

    let has_dynamax = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let dynamax_id = ID::from("dynamax");
        pokemon.volatiles.contains_key(&dynamax_id)
    };

    // if (!pokemon.ignoringItem() && !pokemon.volatiles['dynamax'] && move.id !== this.effectState.move && move.id !== 'struggle')
    if !ignoring_item && !has_dynamax && move_id != locked_move_id && move_id != "struggle" {
        // this.addMove('move', pokemon, move.name);
        let move_name = active_move_ref.name.clone();
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add_move(&["move", &pokemon_ident, &move_name]);

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.debug("Disabled by Choice item lock");
        // In Rust, we just use eprintln for debug

        // this.add('-fail', pokemon);
        battle.add("-fail", &[Arg::String(pokemon_ident)]);

        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onDisableMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onDisableMove(pokemon) {
///     if (!pokemon.getItem().isChoice || !pokemon.hasMove(this.effectState.move)) {
///         pokemon.removeVolatile('choicelock');
///         return;
///     }
///     if (pokemon.ignoringItem() || pokemon.volatiles['dynamax']) {
///         return;
///     }
///     for (const moveSlot of pokemon.moveSlots) {
///         if (moveSlot.id !== this.effectState.move) {
///             pokemon.disableMove(moveSlot.id, false, this.effectState.sourceEffect);
///         }
///     }
/// }
/// ```
pub fn on_disable_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Get is_choice, has_move, and locked_move_id
    let (is_choice, has_locked_move, locked_move_id) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_choice = battle.dex.items().get_by_id(pokemon.get_item())
            .map(|item| item.is_choice)
            .unwrap_or(false);

        let choicelock_id = ID::from("choicelock");
        let locked_move = pokemon.volatiles.get(&choicelock_id)
            .and_then(|v| v.move_id.clone())
            .unwrap_or_default();

        // Create ActiveMove from ID to call has_move
        let has_move = battle.dex.get_active_move(locked_move.as_str())
            .map(|am| pokemon.has_move(&am))
            .unwrap_or(false);

        (is_choice, has_move, locked_move)
    };

    // if (!pokemon.getItem().isChoice || !pokemon.hasMove(this.effectState.move))
    if !is_choice || !has_locked_move {
        // pokemon.removeVolatile('choicelock');
        let choicelock_id = ID::from("choicelock");
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &choicelock_id);
        return EventResult::Continue;
    }

    // if (pokemon.ignoringItem() || pokemon.volatiles['dynamax'])
    let ignoring_item = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ignoring_item(battle, false)
    };

    let has_dynamax = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let dynamax_id = ID::from("dynamax");
        pokemon.volatiles.contains_key(&dynamax_id)
    };

    if ignoring_item || has_dynamax {
        return EventResult::Continue;
    }

    // for (const moveSlot of pokemon.moveSlots)
    // Get move slots to iterate
    let move_slots = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon.move_slots.iter().map(|ms| ms.id.clone()).collect::<Vec<_>>()
    };

    for move_id in move_slots {
        // if (moveSlot.id !== this.effectState.move)
        if move_id.as_str() != locked_move_id {
            // pokemon.disableMove(moveSlot.id, false, this.effectState.sourceEffect);
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            // Note: sourceEffect from volatile data is not set, so pass None
            pokemon.disable_move(move_id.as_str(), false, None);
        }
    }

    EventResult::Continue
}

