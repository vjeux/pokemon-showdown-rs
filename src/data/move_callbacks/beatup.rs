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
    // Build list of party members that are not fainted and not statused
    // IMPORTANT: This iterates over the ENTIRE party (pokemon.side.pokemon), not just active Pokemon
    // CRITICAL: JavaScript swaps Pokemon positions in the array when switching, but Rust doesn't.
    // We need to iterate in position order (the order JS would see them) to match JavaScript behavior.
    let mut allies = Vec::new();

    let side_index = pokemon_pos.0;
    // pokemon_pos.1 is the PARTY INDEX of the user (not active slot index)
    let user_party_index = pokemon_pos.1;

    let party_size = if let Some(side) = battle.sides.get(side_index) {
        side.pokemon.len()
    } else {
        return EventResult::Continue;
    };

    // Create a list of (position, party_index) and sort by position to match JS iteration order
    let mut position_order: Vec<(usize, usize)> = Vec::new();
    for poke_idx in 0..party_size {
        if let Some(pokemon) = battle.pokemon_at(side_index, poke_idx) {
            position_order.push((pokemon.position, poke_idx));
        }
    }
    // Sort by position (the order JS would see them in pokemon.side.pokemon)
    position_order.sort_by_key(|(pos, _)| *pos);

    for (_pos, poke_idx) in position_order {
        let pos = (side_index, poke_idx);
        if let Some(pokemon) = battle.pokemon_at(pos.0, pos.1) {
            // ally === pokemon || !ally.fainted && !ally.status
            // Compare party indices - user is always included
            let is_user = poke_idx == user_party_index;
            let include = is_user || (!pokemon.fainted && pokemon.status == ID::from(""));
            if include {
                allies.push(pos);
            }
        }
    }

    // move.multihit = move.allies.length;
    let allies_count = allies.len() as i32;

    // Store allies directly on the active move (not in effect_state which is temporary)
    // Set multihit on the active move
    if let Some(ref mut active_move) = battle.active_move {
        active_move.borrow_mut().allies = Some(allies);
        active_move.borrow_mut().multi_hit = Some(crate::dex::Multihit::Fixed(allies_count));
    }

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
    // Get allies from active move (not effect_state)
    // move.allies!.shift() - pop the first ally from the list
    let ally_pos = if let Some(ref active_move) = battle.active_move {
        if let Some(ref mut allies) = active_move.borrow_mut().allies {
            if !allies.is_empty() {
                Some(allies.remove(0))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let ally_pos = match ally_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const setSpecies = this.dex.species.get(move.allies!.shift()!.set.species);
    // Note: JavaScript uses the SET species (original team species), not the current battle species.
    // This is important for form changes like Minior-Meteor which has different base stats.
    let set_species = if let Some(pokemon) = battle.pokemon_at(ally_pos.0, ally_pos.1) {
        pokemon.set.species.clone()
    } else {
        return EventResult::Continue;
    };

    let species_id = ID::from(set_species.as_str());
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
