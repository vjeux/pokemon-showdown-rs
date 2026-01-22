//! Telekinesis Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onTry(source, target, move) {
///     // Additional Gravity check for Z-move variant
///     if (this.field.getPseudoWeather('Gravity')) {
///         this.attrLastMove('[still]');
///         this.add('cant', source, 'move: Gravity', move);
///         return null;
///     }
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // if (this.field.getPseudoWeather('Gravity'))
    let has_gravity = battle
        .field
        .pseudo_weather
        .contains_key(&ID::from("gravity"));

    if has_gravity {
        // this.add('cant', source, 'move: Gravity', move);
        let source_slot = {
            let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => ID::from(""),
        };

        battle.add(
            "cant",
            &[
                crate::battle::Arg::from(source_slot),
                crate::battle::Arg::from("move: Gravity"),
                crate::battle::Arg::from(move_id.to_string()),
            ],
        );

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     if (['Diglett', 'Dugtrio', 'Palossand', 'Sandygast'].includes(target.baseSpecies.baseSpecies) ||
    ///         target.baseSpecies.name === 'Gengar-Mega') {
    ///         this.add('-immune', target);
    ///         return null;
    ///     }
    ///     if (target.volatiles['smackdown'] || target.volatiles['ingrain']) return false;
    ///     this.add('-start', target, 'Telekinesis');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let target = pokemon_pos;

        // if (['Diglett', 'Dugtrio', 'Palossand', 'Sandygast'].includes(target.baseSpecies.baseSpecies) ||
        //     target.baseSpecies.name === 'Gengar-Mega')
        let (is_immune, target_slot) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Check baseSpecies.baseSpecies for ground-type Pokemon
            let base_species_name = target_pokemon.get_base_species_name(&battle.dex);
            let is_ground_immune = matches!(
                base_species_name.as_deref(),
                Some("Diglett") | Some("Dugtrio") | Some("Palossand") | Some("Sandygast")
            );

            // Check baseSpecies.name for Gengar-Mega (the species name, not base species)
            let species_name = battle.dex.species().get(target_pokemon.species_id.as_str())
                .map(|s| s.name.clone());
            let is_gengar_mega = species_name.as_deref() == Some("Gengar-Mega");

            let is_immune = is_ground_immune || is_gengar_mega;

            (is_immune, target_pokemon.get_slot())
        };

        if is_immune {
            // this.add('-immune', target);
            battle.add(
                "-immune",
                &[crate::battle::Arg::from(target_slot)],
            );

            // return null;
            return EventResult::Null;
        }

        // if (target.volatiles['smackdown'] || target.volatiles['ingrain']) return false;
        let has_smackdown_or_ingrain = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.volatiles.contains_key(&ID::from("smackdown"))
                || target_pokemon.volatiles.contains_key(&ID::from("ingrain"))
        };

        if has_smackdown_or_ingrain {
            return EventResult::Boolean(false);
        }

        // this.add('-start', target, 'Telekinesis');
        battle.add(
            "-start",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Telekinesis"),
            ],
        );

        EventResult::Continue
    }

    /// onAccuracy(accuracy, target, source, move) {
    ///     if (move && !move.ohko) return true;
    /// }
    pub fn on_accuracy(
        _battle: &mut Battle,
        _accuracy: i32,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        // Get active_move from parameter
        let active_move_ref = match active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (move && !move.ohko) return true;
        // In JS, !move.ohko is true when ohko is falsy (null, undefined, false, etc.)
        // In Rust, ohko is Option<String>, so None means not an OHKO move
        if active_move_ref.ohko.is_none() {
            return EventResult::Boolean(true);
        }

        EventResult::Continue
    }

    /// onImmunity(type) {
    ///     if (type === 'Ground') return false;
    /// }
    pub fn on_immunity(_battle: &mut Battle, immunity_type: &str) -> EventResult {
        // if (type === 'Ground') return false;
        if immunity_type == "Ground" {
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onUpdate(pokemon) {
    ///     if (pokemon.baseSpecies.name === 'Gengar-Mega') {
    ///         delete pokemon.volatiles['telekinesis'];
    ///         this.add('-end', pokemon, 'Telekinesis', '[silent]');
    ///     }
    /// }
    pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.baseSpecies.name === 'Gengar-Mega')
        let (is_gengar_mega, pokemon_slot) = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Check baseSpecies.name (the species name, not base species)
            let species_name = battle.dex.species().get(pokemon_ref.species_id.as_str())
                .map(|s| s.name.clone());
            let is_gengar_mega = species_name.as_deref() == Some("Gengar-Mega");

            (is_gengar_mega, pokemon_ref.get_slot())
        };

        if is_gengar_mega {
            // delete pokemon.volatiles['telekinesis'];
            Pokemon::remove_volatile(battle, pokemon, &ID::from("telekinesis"));

            // this.add('-end', pokemon, 'Telekinesis', '[silent]');
            battle.add(
                "-end",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("Telekinesis"),
                    crate::battle::Arg::from("[silent]"),
                ],
            );
        }

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Telekinesis');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Telekinesis');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Telekinesis"),
            ],
        );

        EventResult::Continue
    }
}
