//! Tera Shift Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Terapagos') return;
///     if (pokemon.species.forme !== 'Terastal') {
///         this.add('-activate', pokemon, 'ability: Tera Shift');
///         pokemon.formeChange('Terapagos-Terastal', this.effect, true);
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;
    

    // if (pokemon.baseSpecies.baseSpecies !== 'Terapagos') return;
    let (base_species_base_species, forme) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (base_species_base_species, species_data.forme.clone())
    };

    if base_species_base_species.as_deref() != Some("Terapagos") {
        return EventResult::Continue;
    }

    // if (pokemon.species.forme !== 'Terastal')
    if forme.as_deref() != Some("Terastal") {
        // this.add('-activate', pokemon, 'ability: Tera Shift');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(pokemon_slot),
            Arg::Str("ability: Tera Shift"),
        ]);

        // pokemon.formeChange('Terapagos-Terastal', this.effect, true);
        // pokemon_pos is already (side_idx, pokemon_index), pass it directly
        crate::pokemon::Pokemon::forme_change(
            battle,
            pokemon_pos,
            ID::from("terapagosterastal"),
            Some(Effect::ability("terashift")),
            true,
            "0",
            None
        );
    }

    EventResult::Continue
}

