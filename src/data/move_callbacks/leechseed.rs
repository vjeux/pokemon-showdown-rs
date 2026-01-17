//! Leech Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return !target.hasType('Grass');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return !target.hasType('Grass');
    let has_grass_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, "grass")
    };

    EventResult::Boolean(!has_grass_type)
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'move: Leech Seed');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let target = pokemon_pos;

        // this.add('-start', target, 'move: Leech Seed');
        let target_arg = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add("-start", &[target_arg.into(), "move: Leech Seed".into()]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     const target = this.getAtSlot(pokemon.volatiles['leechseed'].sourceSlot);
    ///     if (!target || target.fainted || target.hp <= 0) {
    ///         this.debug('Nothing to leech into');
    ///         return;
    ///     }
    ///     const damage = this.damage(pokemon.baseMaxhp / 8, pokemon, target);
    ///     if (damage) {
    ///         this.heal(damage, target, pokemon);
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // const target = this.getAtSlot(pokemon.volatiles['leechseed'].sourceSlot);
        // sourceSlot stores the slot index (0, 1, etc.) and source stores (side_idx, party_idx)
        // We need to get the pokemon currently in that active slot, not the original party pokemon
        let (source_side_idx, source_slot_idx) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let leechseed_volatile = match pokemon_pokemon.volatiles.get(&ID::from("leechseed")) {
                Some(v) => v,
                None => return EventResult::Continue,
            };
            // Get the side from source and the slot index from source_slot
            let source_side = leechseed_volatile.source.map(|(side, _)| side);
            let slot = leechseed_volatile.source_slot;
            match (source_side, slot) {
                (Some(side), Some(slot)) => (side, slot),
                _ => return EventResult::Continue,
            }
        };

        // Look up the pokemon currently in that active slot (like JavaScript's getAtSlot)
        let target_pos = {
            let side = match battle.sides.get(source_side_idx) {
                Some(s) => s,
                None => return EventResult::Continue,
            };
            // Get the party index of the pokemon currently in this slot
            let party_idx = match side.active.get(source_slot_idx) {
                Some(Some(idx)) => *idx,
                _ => return EventResult::Continue,
            };
            (source_side_idx, party_idx)
        };

        let target_is_alive = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            !target_pokemon.fainted && target_pokemon.hp > 0
        };

        if !target_is_alive {
            // this.debug('Nothing to leech into');
            return EventResult::Continue;
        }

        // const damage = this.damage(pokemon.baseMaxhp / 8, pokemon, target);
        let damage_amount = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            hp_fraction(pokemon_pokemon.base_maxhp, 8)
        };

        let damage = battle.damage(damage_amount, Some(pokemon), Some(target_pos), None, false);

        // if (damage) {
        //     this.heal(damage, target, pokemon);
        // }
        if let Some(damage_amount) = damage {
            if damage_amount > 0 {
                battle.heal(damage_amount, Some(target_pos), Some(pokemon), None);
            }
        }

        EventResult::Continue
    }
}
