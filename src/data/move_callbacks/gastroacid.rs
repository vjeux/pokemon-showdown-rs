//! Gastro Acid Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryHit(target) {
///     if (target.getAbility().flags['cantsuppress']) {
///         return false;
///     }
///     if (target.hasItem('Ability Shield')) {
///         this.add('-block', target, 'item: Ability Shield');
///         return null;
///     }
/// }
/// JavaScript signature: onTryHit(target, source, move) - TARGET FIRST
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // if (target.getAbility().flags['cantsuppress']) {
    let cant_suppress = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ability_id = &target_pokemon.ability;
        match battle.dex.abilities().get(ability_id.as_str()) {
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
        target_pokemon.has_item(battle, &["abilityshield"])
    };

    if has_ability_shield {
        // this.add('-block', target, 'item: Ability Shield');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };
        battle.add(
            "-block",
            &[target_ident.as_str().into(), "item: Ability Shield".into()],
        );

        // return null; - prevents the hit
        return EventResult::Null;
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
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.hasItem('Ability Shield')) return false;
        let has_ability_shield = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.has_item(battle, &["abilityshield"])
        };

        if has_ability_shield {
            return EventResult::Boolean(false);
        }

        // this.add('-endability', pokemon);
        let pokemon_ident = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };
        battle.add("-endability", &[pokemon_ident.as_str().into()]);

        // this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon, pokemon, 'gastroacid');
        let ability_id = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.ability.clone()
        };

        battle.single_event(
            "End",
            &crate::battle::Effect::ability(ability_id),
            None,
            Some(pokemon),
            Some(pokemon),
            Some(&crate::battle::Effect::move_(ID::from("gastroacid"))),
            None,
        );

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
            match battle.dex.abilities().get(ability_id.as_str()) {
                Some(ability_data) => ability_data.flags.contains_key("cantsuppress"),
                None => false,
            }
        };

        if cant_suppress {
            Pokemon::remove_volatile(battle, pokemon, &ID::from("gastroacid"));
        }

        EventResult::Continue
    }
}
