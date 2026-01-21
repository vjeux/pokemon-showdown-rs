//! Poison Puppeteer Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyAfterSetStatus(status, target, source, effect) {
///     if (source.baseSpecies.name !== "Pecharunt") return;
///     if (source !== this.effectState.target || target === source || effect.effectType !== 'Move') return;
///     if (status.id === 'psn' || status.id === 'tox') {
///         target.addVolatile('confusion');
///     }
/// }
pub fn on_any_after_set_status(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::ID;
    use crate::Pokemon;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let status_id = match status {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    // if (source.baseSpecies.name !== "Pecharunt") return;
    let source_base_species_name = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_data = match battle.dex.species().get(source.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        // JavaScript: species.baseSpecies defaults to species.name if not set
        species_data.base_species.clone().unwrap_or_else(|| species_data.name.clone())
    };

    if source_base_species_name != "Pecharunt" {
        return EventResult::Continue;
    }

    // if (source !== this.effectState.target || target === source || effect.effectType !== 'Move') return;
    let ability_holder_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if source_pos != ability_holder_pos || target_pos == source_pos {
        return EventResult::Continue;
    }

    // Check if effect is a Move - JavaScript: effect.effectType !== 'Move'
    // The effect is passed as a parameter (effect_id), not battle.effect
    // JavaScript: onAnyAfterSetStatus(status, target, source, effect)
    // We need to check the effect_id parameter and look up its type
    let effect_id_str = match effect_id {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // Check if the effect is a Move by looking it up in the dex
    let is_move_effect = battle.dex.moves().get(effect_id_str).is_some();

    if !is_move_effect {
        return EventResult::Continue;
    }

    // if (status.id === 'psn' || status.id === 'tox')
    if status_id == "psn" || status_id == "tox" {
        // target.addVolatile('confusion');
        Pokemon::add_volatile(battle, target_pos, ID::from("confusion"), Some(source_pos), None, None, None);
    }

    EventResult::Continue
}
