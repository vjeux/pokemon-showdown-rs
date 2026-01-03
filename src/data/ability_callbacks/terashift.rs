//! Tera Shift Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
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
    use crate::Pokemon;

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
        unsafe {
            // SAFETY: We're creating two mutable references to battle.
            // This is safe because we're accessing different parts of the battle structure.
            let battle_ptr = battle as *mut Battle;
            let battle_ref1 = &mut *battle_ptr;
            let battle_ref2 = &mut *battle_ptr;

            // Get pokemon directly from sides array
            let side = &mut battle_ref1.sides[pokemon_pos.0];
            let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
            if let Some(pokemon_index) = active_slot {
                if pokemon_index < side.pokemon.len() {
                    let pokemon = &mut side.pokemon[pokemon_index];
                    pokemon.forme_change(battle_ref2, ID::from("terapagosterastal"), Some(ID::from("terashift")), true, "0", None);
                }
            }
        }
    }

    EventResult::Continue
}

