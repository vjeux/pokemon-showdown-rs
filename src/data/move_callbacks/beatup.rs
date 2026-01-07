//! Beat Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     move.allies = pokemon.side.pokemon.filter(ally => ally === pokemon || !ally.fainted && !ally.status);
///     move.multihit = move.allies.length;
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // move.allies = pokemon.side.pokemon.filter(ally => ally === pokemon || !ally.fainted && !ally.status);
    // Build list of allies that are not fainted and not statused
    let mut allies = Vec::new();
    let allies_and_self = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.allies_and_self(battle, false)
    };
    for pos in allies_and_self {
        if let Some(pokemon) = battle.pokemon_at(pos.0, pos.1) {
            // ally === pokemon || !ally.fainted && !ally.status
            if pos == pokemon_pos || (!pokemon.fainted && pokemon.status == ID::from("")) {
                allies.push(pos);
            }
        }
    }

    // move.multihit = move.allies.length;
    let multihit = allies.len() as i32;

    // Store allies and multihit in current effect state
    battle.with_effect_state(|state| {
        state.allies = Some(allies);
        // Note: multihit is not stored in EffectState, it's derived from allies.len()
    });

    EventResult::Continue
}

/// basePowerCallback(pokemon, target, move) {
///     const setSpecies = this.dex.species.get(move.allies!.shift()!.set.species);
///     const bp = 5 + Math.floor(setSpecies.baseStats.atk / 10);
///     this.debug(`BP for ${setSpecies.name} hit: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get allies from current effect state
    let ally_pos = battle.with_effect_state(|state| {
        if let Some(allies) = state.allies.as_mut() {
            // move.allies!.shift()
            if !allies.is_empty() {
                Some(allies.remove(0))
            } else {
                None
            }
        } else {
            None
        }
    }).flatten();

    let ally_pos = match ally_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const setSpecies = this.dex.species.get(move.allies!.shift()!.set.species);
    let species_id = if let Some(pokemon) = battle.pokemon_at(ally_pos.0, ally_pos.1) {
        pokemon.species_id.clone()
    } else {
        return EventResult::Continue;
    };

    let species = match battle.dex.species().get_by_id(&species_id) {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    // const bp = 5 + Math.floor(setSpecies.baseStats.atk / 10);
    let bp = 5 + (species.base_stats.atk / 10);

    // this.debug(`BP for ${setSpecies.name} hit: ${bp}`);
    battle.debug(&format!("BP for {} hit: {}", species.name, bp));

    // return bp;
    EventResult::Number(bp)
}
