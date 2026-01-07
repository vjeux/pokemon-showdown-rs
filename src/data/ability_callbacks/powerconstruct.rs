//! Power Construct Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Zygarde' || pokemon.transformed || !pokemon.hp) return;
///     if (pokemon.species.id === 'zygardecomplete' || pokemon.hp > pokemon.maxhp / 2) return;
///     this.add('-activate', pokemon, 'ability: Power Construct');
///     pokemon.formeChange('Zygarde-Complete', this.effect, true);
///     pokemon.canMegaEvo = pokemon.canMegaEvo === false ? false : this.actions.canMegaEvo(pokemon);
///     pokemon.formeRegression = true;
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (pokemon.baseSpecies.baseSpecies !== 'Zygarde' || pokemon.transformed || !pokemon.hp) return;
    let (base_species_base_species, transformed, hp, maxhp, species_id): (Option<String>, bool, i32, i32, ID) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        (
            base_species_base_species,
            pokemon.transformed,
            pokemon.hp,
            pokemon.maxhp,
            pokemon.species_id.clone(),
        )
    };

    if base_species_base_species.as_deref() != Some("Zygarde") || transformed || hp == 0 {
        return EventResult::Continue;
    }

    // if (pokemon.species.id === 'zygardecomplete' || pokemon.hp > pokemon.maxhp / 2) return;
    if species_id.as_str() == "zygardecomplete" || hp > maxhp / 2 {
        return EventResult::Continue;
    }

    // this.add('-activate', pokemon, 'ability: Power Construct');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(pokemon_slot),
        Arg::Str("ability: Power Construct"),
    ]);

    // pokemon.formeChange('Zygarde-Complete', this.effect, true);
    unsafe {
        let battle_ptr = battle as *mut Battle;
        let battle_ref1 = &mut *battle_ptr;
        let battle_ref2 = &mut *battle_ptr;

        let side = &mut battle_ref1.sides[pokemon_pos.0];
        let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
        if let Some(pokemon_index) = active_slot {
            if pokemon_index < side.pokemon.len() {
                crate::pokemon::Pokemon::forme_change(
                    battle_ref2,
                    (pokemon_pos.0, pokemon_index),
                    ID::from("zygardecomplete"),
                    Some(ID::from("powerconstruct")),
                    true,
                    "0",
                    None
                );
            }
        }
    }

    // pokemon.canMegaEvo = pokemon.canMegaEvo === false ? false : this.actions.canMegaEvo(pokemon);
    // pokemon.formeRegression = true;
    {
        use crate::battle_actions::can_mega_evo;

        // First check if canMegaEvo is already false
        let should_update_can_mega_evo = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.can_mega_evo != Some("false".to_string())
        };

        // If not false, calculate new canMegaEvo value
        let new_can_mega_evo = if should_update_can_mega_evo {
            can_mega_evo(battle, pokemon_pos.0, pokemon_pos.1)
        } else {
            Some("false".to_string())
        };

        // Now update the pokemon
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon_mut.can_mega_evo = new_can_mega_evo;
        pokemon_mut.forme_regression = true;
    }

    EventResult::Continue
}

