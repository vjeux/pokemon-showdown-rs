//! Gastro Acid Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (target.getAbility().flags['cantsuppress']) {
///         return false;
///     }
///     if (target.hasItem('Ability Shield')) {
///         this.add('-block', target, 'item: Ability Shield');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // if (target.getAbility().flags['cantsuppress']) {
    let cant_suppress = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ability_id = &target_pokemon.ability;
        match battle.dex.get_ability(ability_id.as_str()) {
            Some(ability_data) => ability_data.flags.contains_key("cantsuppress"),
            None => false,
        }
    };

    // return false;
    if cant_suppress {
        return EventResult::Boolean(false);
    }

    // if (target.hasItem('Ability Shield')) {
    let has_ability_shield = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_item(&ID::from("abilityshield"))
    };

    if has_ability_shield {
        // this.add('-block', target, 'item: Ability Shield');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };
        battle.add("-block", &[target_arg, "item: Ability Shield".into()]);

        // return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (pokemon.hasItem('Ability Shield')) return false;
    ///     this.add('-endability', pokemon);
    ///     this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon, pokemon, 'gastroacid');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.hasItem('Ability Shield')) return false;
        let has_ability_shield = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.has_item(&ID::from("abilityshield"))
        };

        if has_ability_shield {
            return EventResult::Boolean(false);
        }

        // this.add('-endability', pokemon);
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };
        battle.add("-endability", &[pokemon_arg]);

        // this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon, pokemon, 'gastroacid');
        let ability_id = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.ability.clone()
        };

        battle.single_event("End", &ability_id.to_string(), Some(pokemon), Some(pokemon), Some(&ID::from("gastroacid")));

        EventResult::Continue
    }

    /// onCopy(pokemon) {
    ///     if (pokemon.getAbility().flags['cantsuppress']) pokemon.removeVolatile('gastroacid');
    /// }
    pub fn on_copy(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.getAbility().flags['cantsuppress']) pokemon.removeVolatile('gastroacid');
        let cant_suppress = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let ability_id = &pokemon_pokemon.ability;
            match battle.dex.get_ability(ability_id.as_str()) {
                Some(ability_data) => ability_data.flags.contains_key("cantsuppress"),
                None => false,
            }
        };

        if cant_suppress {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.remove_volatile(&ID::from("gastroacid"));
        }

        EventResult::Continue
    }
}
