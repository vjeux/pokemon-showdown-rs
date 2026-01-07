//! Quark Drive Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Arg;
use crate::Pokemon;

/// onStart(pokemon) {
///     this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Trigger TerrainChange event to check if Quark Drive should activate
    // Pass the ability ID as the effect
    let ability_id = ID::from("quarkdrive");
    battle.single_event("TerrainChange", &Effect::ability(ability_id), None, Some(pokemon_pos), None, None, None);
    EventResult::Continue
}

/// onTerrainChange(pokemon) {
///     if (this.field.isTerrain('electricterrain')) {
///         pokemon.addVolatile('quarkdrive');
///     } else if (!pokemon.volatiles['quarkdrive']?.fromBooster) {
///         pokemon.removeVolatile('quarkdrive');
///     }
/// }
pub fn on_terrain_change(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Check if it's electric terrain
    let is_electric_terrain = battle.is_terrain("electricterrain");

    if is_electric_terrain {
        // Add quarkdrive volatile
        let volatile_id = ID::from("quarkdrive");
        Pokemon::add_volatile(battle, pokemon_pos, volatile_id, None, None, None, None);
    } else {
        // Check if pokemon has quarkdrive volatile and if it's not from Booster Energy
        let id = ID::from("quarkdrive");
        let has_volatile = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.has_volatile(&id)
        } else {
            false
        };

        if has_volatile {
            // Check if fromBooster is set
            let from_booster = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                pokemon.volatiles.get(&id)
                    .and_then(|v| v.from_booster)
                    .unwrap_or(false)
            } else {
                false
            };

            // Only remove volatile if it's not from Booster Energy
            if !from_booster {
                Pokemon::remove_volatile(battle, pokemon_pos, &id);
            }
        }
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     delete pokemon.volatiles['quarkdrive'];
///     this.add('-end', pokemon, 'Quark Drive', '[silent]');
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Remove quarkdrive volatile
    let id = ID::from("quarkdrive");
    Pokemon::remove_volatile(battle, pokemon_pos, &id);

    // Add silent end message
    let slot = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        pokemon.get_slot()
    } else {
        return EventResult::Continue;
    };

    battle.add("-end", &[
        Arg::String(slot),
        Arg::String("Quark Drive".to_string()),
        Arg::String("[silent]".to_string()),
    ]);

    EventResult::Continue
}

pub mod condition {
    use super::*;
    use crate::dex_data::StatID;

    /// onStart(pokemon, source, effect) {
    ///     if (effect?.name === 'Booster Energy') {
    ///         this.effectState.fromBooster = true;
    ///         this.add('-activate', pokemon, 'ability: Quark Drive', '[fromitem]');
    ///     } else {
    ///         this.add('-activate', pokemon, 'ability: Quark Drive');
    ///     }
    ///     this.effectState.bestStat = pokemon.getBestStat(false, true);
    ///     this.add('-start', pokemon, 'quarkdrive' + this.effectState.bestStat);
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
        // Get pokemon slot for messages
        let slot = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.get_slot()
        } else {
            return EventResult::Continue;
        };

        // Check if effect is Booster Energy
        let is_booster_energy = battle.event.as_ref()
            .and_then(|e| e.effect.as_ref())
            .map(|eff| eff.id.as_str() == "boosterenergy")
            .unwrap_or(false);

        if is_booster_energy {
            // Set fromBooster flag in volatile's data using with_effect_state
            // JavaScript: this.effectState.fromBooster = true
            battle.with_effect_state(|state| {
                state.from_booster = Some(true);
            });

            battle.add("-activate", &[
                Arg::String(slot.clone()),
                Arg::String("ability: Quark Drive".to_string()),
                Arg::String("[fromitem]".to_string()),
            ]);
        } else {
            battle.add("-activate", &[
                Arg::String(slot.clone()),
                Arg::String("ability: Quark Drive".to_string()),
            ]);
        }

        // Get best stat by computing it inline to avoid borrow issues
        let stats = [
            StatID::Atk,
            StatID::Def,
            StatID::SpA,
            StatID::SpD,
            StatID::Spe,
        ];
        let mut best_stat = StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = battle.get_pokemon_stat(pokemon_pos, stat, false, true);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        // Store bestStat in volatile's effectState data using with_effect_state
        // JavaScript: this.effectState.bestStat = ...
        let stat_name = match best_stat {
            StatID::Atk => "atk",
            StatID::Def => "def",
            StatID::SpA => "spa",
            StatID::SpD => "spd",
            StatID::Spe => "spe",
            _ => "atk", // Fallback
        };

        battle.with_effect_state(|state| {
            state.best_stat = Some(stat_name.to_string());
        });

        // Add start message with best stat
        let display_stat = match best_stat {
            StatID::Atk => "Atk",
            StatID::Def => "Def",
            StatID::SpA => "SpA",
            StatID::SpD => "SpD",
            StatID::Spe => "Spe",
            _ => "Atk", // Fallback
        };

        battle.add("-start", &[
            Arg::String(slot),
            Arg::String(format!("quarkdrive{}", display_stat)),
        ]);

        EventResult::Continue
    }

    /// onModifyAtk(atk, pokemon) {
    ///     if (this.effectState.bestStat !== 'atk' || pokemon.ignoringAbility()) return;
    ///     this.debug('Quark Drive atk boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_atk(battle: &mut Battle, atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get bestStat from volatile's data using with_effect_state_ref
        // JavaScript: this.effectState.bestStat
        let best_stat = battle.with_effect_state_ref(|state| {
            state.best_stat.clone()
        }).flatten().unwrap_or_default();

        if best_stat != "atk" {
            return EventResult::Continue;
        }

        // Check if pokemon is ignoring ability
        let ignoring = if let Some(pokemon) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            pokemon.ignoring_ability(battle)
        } else {
            return EventResult::Continue;
        };

        if ignoring {
            return EventResult::Continue;
        }

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(atk, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifyDef(def, pokemon) {
    ///     if (this.effectState.bestStat !== 'def' || pokemon.ignoringAbility()) return;
    ///     this.debug('Quark Drive def boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_def(battle: &mut Battle, def: i32, defender_pos: (usize, usize), _attacker_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get bestStat from volatile's data using with_effect_state_ref
        // JavaScript: this.effectState.bestStat
        let best_stat = battle.with_effect_state_ref(|state| {
            state.best_stat.clone()
        }).flatten().unwrap_or_default();

        if best_stat != "def" {
            return EventResult::Continue;
        }

        // Check if pokemon is ignoring ability
        let ignoring = if let Some(pokemon) = battle.pokemon_at(defender_pos.0, defender_pos.1) {
            pokemon.ignoring_ability(battle)
        } else {
            return EventResult::Continue;
        };

        if ignoring {
            return EventResult::Continue;
        }

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(def, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifySpA(spa, pokemon) {
    ///     if (this.effectState.bestStat !== 'spa' || pokemon.ignoringAbility()) return;
    ///     this.debug('Quark Drive spa boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_sp_a(battle: &mut Battle, spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get bestStat from volatile's data using with_effect_state_ref
        // JavaScript: this.effectState.bestStat
        let best_stat = battle.with_effect_state_ref(|state| {
            state.best_stat.clone()
        }).flatten().unwrap_or_default();

        if best_stat != "spa" {
            return EventResult::Continue;
        }

        // Check if pokemon is ignoring ability
        let ignoring = if let Some(pokemon) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            pokemon.ignoring_ability(battle)
        } else {
            return EventResult::Continue;
        };

        if ignoring {
            return EventResult::Continue;
        }

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(spa, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifySpD(spd, pokemon) {
    ///     if (this.effectState.bestStat !== 'spd' || pokemon.ignoringAbility()) return;
    ///     this.debug('Quark Drive spd boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_sp_d(battle: &mut Battle, spd: i32, defender_pos: (usize, usize), _attacker_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get bestStat from volatile's data using with_effect_state_ref
        // JavaScript: this.effectState.bestStat
        let best_stat = battle.with_effect_state_ref(|state| {
            state.best_stat.clone()
        }).flatten().unwrap_or_default();

        if best_stat != "spd" {
            return EventResult::Continue;
        }

        // Check if pokemon is ignoring ability
        let ignoring = if let Some(pokemon) = battle.pokemon_at(defender_pos.0, defender_pos.1) {
            pokemon.ignoring_ability(battle)
        } else {
            return EventResult::Continue;
        };

        if ignoring {
            return EventResult::Continue;
        }

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(spd, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifySpe(spe, pokemon) {
    ///     if (this.effectState.bestStat !== 'spe' || pokemon.ignoringAbility()) return;
    ///     this.debug('Quark Drive spe boost');
    ///     return this.chainModify(1.5);
    /// }
    pub fn on_modify_spe(battle: &mut Battle, spe: i32, pokemon_pos: (usize, usize)) -> EventResult {
        // Get bestStat from volatile's data using with_effect_state_ref
        // JavaScript: this.effectState.bestStat
        let best_stat = battle.with_effect_state_ref(|state| {
            state.best_stat.clone()
        }).flatten().unwrap_or_default();

        if best_stat != "spe" {
            return EventResult::Continue;
        }

        // Check if pokemon is ignoring ability
        let ignoring = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.ignoring_ability(battle)
        } else {
            return EventResult::Continue;
        };

        if ignoring {
            return EventResult::Continue;
        }

        // Apply 1.5x boost (3/2 ratio)
        let modified = battle.modify(spe, 3, 2);
        EventResult::Number(modified)
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Quark Drive');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get pokemon slot for message
        let slot = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.get_slot()
        } else {
            return EventResult::Continue;
        };

        battle.add("-end", &[
            Arg::String(slot),
            Arg::String("Quark Drive".to_string()),
        ]);

        EventResult::Continue
    }
}
