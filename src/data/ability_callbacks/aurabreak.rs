//! Aura Break Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.add('-ability', pokemon, 'Aura Break');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-ability",
        &[
            pokemon_ident.as_str().into(),
            "Aura Break".into(),
        ],
    );
    EventResult::Continue
}

/// onAnyTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     move.hasAuraBreak = true;
/// }
pub fn on_any_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target === source || move.category === 'Status') return;
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let is_status = if let Some(ref active_move) = battle.active_move {
        active_move.category == "Status"
    } else {
        false
    };

    if is_status {
        return EventResult::Continue;
    }

    // move.hasAuraBreak = true;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.has_aura_break = Some(true);
    }

    EventResult::Continue
}

