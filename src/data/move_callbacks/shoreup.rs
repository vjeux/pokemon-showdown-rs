//! Shore Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     let factor = 0.5;
///     if (this.field.isWeather('sandstorm')) {
///         factor = 0.667;
///     }
///     const success = !!this.heal(this.modify(pokemon.maxhp, factor));
///     if (!success) {
///         this.add('-fail', pokemon, 'heal');
///         return this.NOT_FAIL;
///     }
///     return success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // onHit(pokemon) {
    //     let factor = 0.5;
    //     if (this.field.isWeather('sandstorm')) {
    //         factor = 0.667;
    //     }
    //     const success = !!this.heal(this.modify(pokemon.maxhp, factor));
    //     if (!success) {
    //         this.add('-fail', pokemon, 'heal');
    //         return this.NOT_FAIL;
    //     }
    //     return success;
    // }
    let pokemon = pokemon_pos;

    // let factor = 0.5;
    // if (this.field.isWeather('sandstorm')) {
    //     factor = 0.667;
    // }
    let is_sandstorm = battle.field.is_weather("sandstorm");
    let factor = if is_sandstorm { 0.667 } else { 0.5 };

    // const success = !!this.heal(this.modify(pokemon.maxhp, factor));
    let maxhp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.maxhp
    };

    let heal_amount = battle.modify_f(maxhp, factor);
    let heal_result = battle.heal(heal_amount, Some(pokemon), None, None);
    let success = heal_result != Some(0) && heal_result.is_some();

    // if (!success) {
    //     this.add('-fail', pokemon, 'heal');
    //     return this.NOT_FAIL;
    // }
    if !success {
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-fail", &[pokemon_arg.into(), "heal".into()]);

        return EventResult::Boolean(false); // this.NOT_FAIL
    }

    // return success;
    EventResult::Boolean(success)
}
