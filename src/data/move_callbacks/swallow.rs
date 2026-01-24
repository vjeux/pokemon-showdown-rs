//! Swallow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onTry(source, target, move) {
///     if (move.sourceEffect === 'snatch') return;
///     return !!source.volatiles['stockpile'];
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (move.sourceEffect === 'snatch') return;
    // Check if this move was called by Snatch
    let source_effect = battle.with_effect_state_ref(|state| state.source_effect.clone()).flatten();
    if let Some(effect) = source_effect {
        if effect.as_str() == "snatch" {
            return EventResult::Continue;
        }
    }

    // return !!source.volatiles['stockpile'];
    let has_stockpile = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_volatile(&ID::from("stockpile"))
    };

    if has_stockpile {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}

/// onHit(pokemon) {
///     const layers = pokemon.volatiles['stockpile']?.layers || 1;
///     const healAmount = [0.25, 0.5, 1];
///     const success = !!this.heal(this.modify(pokemon.maxhp, healAmount[layers - 1]));
///     if (!success) this.add('-fail', pokemon, 'heal');
///     pokemon.removeVolatile('stockpile');
///     return success || this.NOT_FAIL;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // const layers = pokemon.volatiles['stockpile']?.layers || 1;
    let layers = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(stockpile_volatile) = pokemon_pokemon.volatiles.get(&ID::from("stockpile")) {
            stockpile_volatile.borrow().layers.unwrap_or(1)
        } else {
            1
        }
    };

    // const healAmount = [0.25, 0.5, 1];
    // const success = !!this.heal(this.modify(pokemon.maxhp, healAmount[layers - 1]));
    let heal_amounts = [(1, 4), (1, 2), (1, 1)]; // [0.25, 0.5, 1.0] as fractions
    let (numerator, denominator) = heal_amounts[(layers - 1).max(0).min(2) as usize];

    let heal_amount = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.modify(pokemon_pokemon.maxhp, numerator, denominator)
    };

    let success = battle.heal(heal_amount, Some(pokemon), None, None);

    // if (!success) this.add('-fail', pokemon, 'heal');
    if success.is_none() {
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-fail", &[pokemon_arg.into(), "heal".into()]);
    }

    // pokemon.removeVolatile('stockpile');
    {
        Pokemon::remove_volatile(battle, pokemon, &ID::from("stockpile"));
    }

    // return success || this.NOT_FAIL;
    if success.is_some() {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}
