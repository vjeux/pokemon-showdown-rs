//! Unnerve Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.effectState.unnerved) return;
///     this.add('-ability', pokemon, 'Unnerve');
///     this.effectState.unnerved = true;
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;

    // if (this.effectState.unnerved) return;
    let already_unnerved = battle.with_effect_state_ref(|state| {
        state.unnerved
    }).flatten().unwrap_or(false);
    if already_unnerved {
        return EventResult::Continue;
    }

    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let side_id = format!("p{}", pokemon.side_index + 1);
        if pokemon.is_active {
            let pos_letter = (b'a' + pokemon.position as u8) as char;
            format!("{}{}: {}", side_id, pos_letter, pokemon.name)
        } else {
            format!("{}: {}", side_id, pokemon.name)
        }
    };

    // this.add('-ability', pokemon, 'Unnerve');
    battle.add("-ability", &[
        Arg::String(pokemon_id),
        Arg::Str("Unnerve"),
    ]);

    // this.effectState.unnerved = true;
    battle.with_effect_state(|state| {
        state.unnerved = Some(true);
    });

    EventResult::Continue
}

/// onEnd() {
///     this.effectState.unnerved = false;
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // this.effectState.unnerved = false;
    battle.with_effect_state(|state| {
        state.unnerved = Some(false);
    });
    EventResult::Continue
}

/// onFoeTryEatItem() {
///     return !this.effectState.unnerved;
/// }
pub fn on_foe_try_eat_item(battle: &mut Battle) -> EventResult {
    // return !this.effectState.unnerved;
    let unnerved = battle.with_effect_state_ref(|state| {
        state.unnerved
    }).flatten().unwrap_or(false);
    EventResult::Boolean(!unnerved)
}

