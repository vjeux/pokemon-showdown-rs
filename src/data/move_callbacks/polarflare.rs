//! Polar Flare Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, pokemon, move) {
///     if (pokemon.baseSpecies.baseSpecies === 'Ramnarok' && !pokemon.transformed) {
///         move.willChangeForme = true;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (pokemon.baseSpecies.baseSpecies === 'Ramnarok' && !pokemon.transformed) {
    let (is_ramnarok, transformed) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_pokemon.base_species == ID::from("ramnarok"), pokemon_pokemon.transformed)
    };

    if is_ramnarok && !transformed {
        // move.willChangeForme = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.will_change_forme = true;
        }
    }

    EventResult::Continue
}

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (move.willChangeForme) {
///         const forme = pokemon.species_id.id === 'ramnarokradiant' ? '' : '-Radiant';
///         pokemon.formeChange('Ramnarok' + forme, this.effect, false, '0', '[msg]');
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (move.willChangeForme) {
    let will_change_forme = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.will_change_forme
    };

    if !will_change_forme {
        return EventResult::Continue;
    }

    // const forme = pokemon.species_id.id === 'ramnarokradiant' ? '' : '-Radiant';
    let forme = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if pokemon_pokemon.species_id == ID::from("ramnarokradiant") {
            ""
        } else {
            "-Radiant"
        }
    };

    // pokemon.formeChange('Ramnarok' + forme, this.effect, false, '0', '[msg]');
    let target_forme = format!("Ramnarok{}", forme);
    // TODO: Implement forme_change method in Battle
    // battle.forme_change(...);

    EventResult::Continue
}

