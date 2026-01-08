//! Illusion Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBeforeSwitchIn(pokemon) {
///     pokemon.illusion = null;
///     // yes, you can Illusion an active pokemon but only if it's to your right
///     for (let i = pokemon.side.pokemon.length - 1; i > pokemon.position; i--) {
///         const possibleTarget = pokemon.side.pokemon[i];
///         if (!possibleTarget.fainted) {
///             // If Ogerpon is in the last slot while the Illusion Pokemon is Terastallized
///             // Illusion will not disguise as anything
///             if (!pokemon.terastallized || !['Ogerpon', 'Terapagos'].includes(possibleTarget.species.baseSpecies)) {
///                 pokemon.illusion = possibleTarget;
///             }
///             break;
///         }
///     }
/// }
pub fn on_before_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let (side_index, pokemon_index) = pokemon_pos;

    // pokemon.illusion = null;
    {
        let pokemon = match battle.pokemon_at_mut(side_index, pokemon_index) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.illusion = None;
    }

    // Get pokemon position and terastallized status
    let (position, is_terastallized) = {
        let pokemon = match battle.pokemon_at(side_index, pokemon_index) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.position, pokemon.terastallized.is_some())
    };

    // for (let i = pokemon.side.pokemon.length - 1; i > pokemon.position; i--)
    // Iterate from right to left through pokemon on the side
    let side_pokemon_count = battle.sides[side_index].pokemon.len();

    for i in (position + 1..side_pokemon_count).rev() {
        // const possibleTarget = pokemon.side.pokemon[i];
        // if (!possibleTarget.fainted)
        let (target_fainted, target_base_species) = {
            let target = match battle.pokemon_at(side_index, i) {
                Some(p) => p,
                None => continue,
            };
            (target.fainted, target.base_species.clone())
        };

        if !target_fainted {
            // if (!pokemon.terastallized || !['Ogerpon', 'Terapagos'].includes(possibleTarget.species.baseSpecies))
            let should_disguise = if is_terastallized {
                target_base_species.as_str() != "Ogerpon" && target_base_species.as_str() != "Terapagos"
            } else {
                true
            };

            if should_disguise {
                // pokemon.illusion = possibleTarget;
                let pokemon = match battle.pokemon_at_mut(side_index, pokemon_index) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.illusion = Some(i);
            }
            break;
        }
    }

    EventResult::Continue
}

/// onDamagingHit(damage, target, source, move) {
///     if (target.illusion) {
///         this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, source, move);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.illusion)
    let has_illusion = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.illusion.is_some()
    };

    if has_illusion {
        // this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, source, move);
        let illusion_id = ID::from("illusion");
        battle.single_event("End", &crate::battle::Effect::ability(illusion_id), None, Some(target), source_pos, None, None);
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (pokemon.illusion) {
///         this.debug('illusion cleared');
///         pokemon.illusion = null;
///         const details = pokemon.getUpdatedDetails();
///         this.add('replace', pokemon, details);
///         this.add('-end', pokemon, 'Illusion');
///         if (this.ruleTable.has('illusionlevelmod')) {
///             this.hint("Illusion Level Mod is active, so this Pokémon's true level was hidden.", true);
///         }
///     }
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    

    // if (pokemon.illusion)
    let has_illusion = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.illusion.is_some()
    };

    if has_illusion {
        // this.debug('illusion cleared');
        eprintln!("DEBUG: illusion cleared");

        // pokemon.illusion = null;
        // const details = pokemon.getUpdatedDetails();
        let details = {
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.illusion = None;
            pokemon.get_updated_details()
        };

        // this.add('replace', pokemon, details);
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("replace", &[pokemon_slot.clone().into(), details.into()]);

        // this.add('-end', pokemon, 'Illusion');
        battle.add("-end", &[pokemon_slot.into(), "Illusion".into()]);

        // if (this.ruleTable.has('illusionlevelmod'))
        if let Some(ref rule_table) = battle.rule_table {
            if rule_table.has("illusionlevelmod") {
                // this.hint("Illusion Level Mod is active, so this Pokémon's true level was hidden.", true);
                battle.hint("Illusion Level Mod is active, so this Pokémon's true level was hidden.", true, None);
            }
        }
    }

    EventResult::Continue
}

/// onFaint(pokemon) {
///     pokemon.illusion = null;
/// }
pub fn on_faint(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // pokemon.illusion = null;
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon.illusion = None;

    EventResult::Continue
}

