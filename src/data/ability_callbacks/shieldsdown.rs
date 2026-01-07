//! Shields Down Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed) return;
///     if (pokemon.hp > pokemon.maxhp / 2) {
///         if (pokemon.species.forme !== 'Meteor') {
///             pokemon.formeChange('Minior-Meteor');
///         }
///     } else {
///         if (pokemon.species.forme === 'Meteor') {
///             pokemon.formeChange(pokemon.set.species);
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed) return;
    let (base_species_base_species, transformed, hp, maxhp, forme, set_species): (Option<String>, bool, i32, i32, Option<String>, String) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (
            base_species_base_species,
            pokemon.transformed,
            pokemon.hp,
            pokemon.maxhp,
            species_data.forme.clone(),
            pokemon.set.species.clone(),
        )
    };

    if base_species_base_species.as_deref() != Some("Minior") || transformed {
        return EventResult::Continue;
    }

    // if (pokemon.hp > pokemon.maxhp / 2)
    if hp > maxhp / 2 {
        // if (pokemon.species.forme !== 'Meteor')
        if forme.as_deref() != Some("Meteor") {
            // pokemon.formeChange('Minior-Meteor');
            unsafe {
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        
                        crate::pokemon::Pokemon::forme_change(battle_ref2, (pokemon_pos.0, pokemon_index), ID::from("miniormeteor"), Some(ID::from("shieldsdown")), false, "0", None);
                    }
                }
            }
        }
    } else {
        // if (pokemon.species.forme === 'Meteor')
        if forme.as_deref() == Some("Meteor") {
            // pokemon.formeChange(pokemon.set.species);
            unsafe {
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        
                        crate::pokemon::Pokemon::forme_change(battle_ref2, (pokemon_pos.0, pokemon_index), ID::from(set_species.clone()), Some(ID::from("shieldsdown")), false, "0", None);
                    }
                }
            }
        }
    }

    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed || !pokemon.hp) return;
///     if (pokemon.hp > pokemon.maxhp / 2) {
///         if (pokemon.species.forme !== 'Meteor') {
///             pokemon.formeChange('Minior-Meteor');
///         }
///     } else {
///         if (pokemon.species.forme === 'Meteor') {
///             pokemon.formeChange(pokemon.set.species);
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed || !pokemon.hp) return;
    let (base_species_base_species, transformed, hp, maxhp, forme, set_species): (Option<String>, bool, i32, i32, Option<String>, String) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (
            base_species_base_species,
            pokemon.transformed,
            pokemon.hp,
            pokemon.maxhp,
            species_data.forme.clone(),
            pokemon.set.species.clone(),
        )
    };

    if base_species_base_species.as_deref() != Some("Minior") || transformed || hp == 0 {
        return EventResult::Continue;
    }

    // if (pokemon.hp > pokemon.maxhp / 2)
    if hp > maxhp / 2 {
        // if (pokemon.species.forme !== 'Meteor')
        if forme.as_deref() != Some("Meteor") {
            // pokemon.formeChange('Minior-Meteor');
            unsafe {
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        
                        crate::pokemon::Pokemon::forme_change(battle_ref2, (pokemon_pos.0, pokemon_index), ID::from("miniormeteor"), Some(ID::from("shieldsdown")), false, "0", None);
                    }
                }
            }
        }
    } else {
        // if (pokemon.species.forme === 'Meteor')
        if forme.as_deref() == Some("Meteor") {
            // pokemon.formeChange(pokemon.set.species);
            unsafe {
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        
                        crate::pokemon::Pokemon::forme_change(battle_ref2, (pokemon_pos.0, pokemon_index), ID::from(set_species.clone()), Some(ID::from("shieldsdown")), false, "0", None);
                    }
                }
            }
        }
    }

    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (target.species.id !== 'miniormeteor' || target.transformed) return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Shields Down');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, _status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (target.species.id !== 'miniormeteor' || target.transformed) return;
    let (species_id, transformed) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target.species_id.clone(), target.transformed)
    };

    if species_id.as_str() != "miniormeteor" || transformed {
        return EventResult::Continue;
    }

    // if ((effect as Move)?.status)
    let is_move_with_status = if let Some(eff_id) = effect_id {
        if let Some(move_data) = battle.dex.moves().get_by_id(&ID::from(eff_id)) {
            move_data.status.is_some()
        } else {
            false
        }
    } else {
        false
    };

    if is_move_with_status {
        // this.add('-immune', target, '[from] ability: Shields Down');
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Boolean(false),
            };
            target.get_slot()
        };

        battle.add("-immune", &[
            Arg::String(target_slot),
            Arg::Str("[from] ability: Shields Down"),
        ]);
    }

    // return false;
    EventResult::Boolean(false)
}

/// onTryAddVolatile(status, target) {
///     if (target.species.id !== 'miniormeteor' || target.transformed) return;
///     if (status.id !== 'yawn') return;
///     this.add('-immune', target, '[from] ability: Shields Down');
///     return null;
/// }
pub fn on_try_add_volatile(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (target.species.id !== 'miniormeteor' || target.transformed) return;
    let (species_id, transformed) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target.species_id.clone(), target.transformed)
    };

    if species_id.as_str() != "miniormeteor" || transformed {
        return EventResult::Continue;
    }

    // if (status.id !== 'yawn') return;
    if status_id != "yawn" {
        return EventResult::Continue;
    }

    // this.add('-immune', target, '[from] ability: Shields Down');
    let target_slot = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Null,
        };
        target.get_slot()
    };

    battle.add("-immune", &[
        Arg::String(target_slot),
        Arg::Str("[from] ability: Shields Down"),
    ]);

    // return null;
    EventResult::Null
}

