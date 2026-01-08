//! Guard Dog Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDragOut(pokemon) {
///     this.add('-activate', pokemon, 'ability: Guard Dog');
///     return null;
/// }
pub fn on_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize), __source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Null,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-activate",
        &[
            pokemon_ident.as_str().into(),
            "ability: Guard Dog".into(),
        ],
    );
    EventResult::Null
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.boost({ atk: 1 }, target, target, null, false, true);
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>,
) -> EventResult {
    // Check if effect is Intimidate
    let is_intimidate = battle.event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|effect| effect.id.as_str() == "intimidate")
        .unwrap_or(false);

    if !is_intimidate {
        return EventResult::Continue;
    }

    // Check if we have a boost table
    let boost = match boost {
        Some(b) => b,
        None => return EventResult::Continue,
    };

    // if (boost.atk) {
    if boost.atk != 0 {
        // delete boost.atk;
        boost.atk = 0;

        // this.boost({ atk: 1 }, target, target, null, false, true);
        battle.boost(
            &[("atk", 1)],
            target_pos,
            Some(target_pos),
            None,
            false,
            true,
        );
    }

    EventResult::Continue
}

