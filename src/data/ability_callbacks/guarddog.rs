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
pub fn on_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
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
pub fn on_try_boost(_battle: &mut Battle, _boost: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

