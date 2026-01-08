//! Stance Change Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onModifyMove(move, attacker, defender) {
///     if (attacker.species.baseSpecies !== 'Aegislash' || attacker.transformed) return;
///     if (move.category === 'Status' && move.id !== 'kingsshield') return;
///     const targetForme = (move.id === 'kingsshield' ? 'Aegislash' : 'Aegislash-Blade');
///     if (attacker.species.name !== targetForme) attacker.formeChange(targetForme);
/// }
pub fn on_modify_move(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // Get move info from passed parameter
    let (move_category, move_id) = match &active_move {
        Some(m) => (m.category.clone(), m.id.clone()),
        None => return EventResult::Continue,
    };

    // Get attacker position from current event
    let attacker_pos = match &battle.event {
        Some(event) => match event.source {
            Some(pos) => pos,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // if (attacker.species.baseSpecies !== 'Aegislash' || attacker.transformed) return;
    let (base_species, transformed, species_name) = {
        let attacker = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_data = match battle.dex.species().get(attacker.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (
            species_data.base_species.clone().unwrap_or_default(),
            attacker.transformed,
            species_data.name.clone(),
        )
    };

    if base_species != "Aegislash" || transformed {
        return EventResult::Continue;
    }

    // if (move.category === 'Status' && move.id !== 'kingsshield') return;
    if move_category == "Status" && move_id.as_str() != "kingsshield" {
        return EventResult::Continue;
    }

    // const targetForme = (move.id === 'kingsshield' ? 'Aegislash' : 'Aegislash-Blade');
    let target_forme = if move_id.as_str() == "kingsshield" {
        "aegislash"
    } else {
        "aegislashblade"
    };

    // if (attacker.species.name !== targetForme) attacker.formeChange(targetForme);
    if species_name != target_forme && species_name.to_lowercase().replace("-", "") != target_forme {
        // attacker_pos is already (side_idx, pokemon_index), pass it directly
        crate::pokemon::Pokemon::forme_change(
            battle,
            attacker_pos,
            ID::from(target_forme),
            Some(Effect::ability("stancechange")),
            false,
            "0",
            None
        );
    }

    EventResult::Continue
}

