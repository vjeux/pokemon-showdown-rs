//! Protosynthesis Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;
use crate::Arg;
use crate::Pokemon;

/// onStart(pokemon) {
///     this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Trigger WeatherChange event to check if Protosynthesis should activate
    // Pass the ability ID as the effect
    let ability_id = ID::from("protosynthesis");
    battle.single_event("WeatherChange", &ability_id, Some(pokemon_pos), None, None);
    EventResult::Continue
}

/// onWeatherChange(pokemon) {
///     // Protosynthesis is not affected by Utility Umbrella
///     if (this.field.isWeather('sunnyday')) {
///         pokemon.addVolatile('protosynthesis');
///     } else if (!pokemon.volatiles['protosynthesis']?.fromBooster && !this.field.isWeather('sunnyday')) {
///         pokemon.removeVolatile('protosynthesis');
///     }
/// }
pub fn on_weather_change(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Check if it's sunny day
    let is_sunny = battle.field.is_weather("sunnyday");
    eprintln!("[PROTOSYNTHESIS] on_weather_change called, is_sunny={}", is_sunny);

    if is_sunny {
        // Add protosynthesis volatile
        let volatile_id = ID::from("protosynthesis");
        eprintln!("[PROTOSYNTHESIS] Adding volatile");
        Pokemon::add_volatile(battle, pokemon_pos, volatile_id, None, None, None);
    } else {
        // Check if pokemon has protosynthesis volatile and if it's not from Booster Energy
        let id = ID::from("protosynthesis");
        let has_volatile = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.has_volatile(&id)
        } else {
            false
        };
        eprintln!("[PROTOSYNTHESIS] Not sunny, has_volatile={}", has_volatile);

        if has_volatile {
            // TODO: Check if fromBooster is set (need to implement effect state)
            // For now, remove the volatile if it's not sunny
            Pokemon::remove_volatile(battle, pokemon_pos, &id);
        }
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     delete pokemon.volatiles['protosynthesis'];
///     this.add('-end', pokemon, 'Protosynthesis', '[silent]');
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Remove protosynthesis volatile
    let id = crate::dex_data::ID::from("protosynthesis");
    Pokemon::remove_volatile(battle, pokemon_pos, &id);

    // Add silent end message
    let slot = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        pokemon.get_slot()
    } else {
        return EventResult::Continue;
    };

    battle.add("-end", &[
        Arg::String(slot),
        Arg::String("Protosynthesis".to_string()),
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
    ///         this.add('-activate', pokemon, 'ability: Protosynthesis', '[fromitem]');
    ///     } else {
    ///         this.add('-activate', pokemon, 'ability: Protosynthesis');
    ///     }
    ///     this.effectState.bestStat = pokemon.getBestStat(false, true);
    ///     this.add('-start', pokemon, 'protosynthesis' + this.effectState.bestStat);
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get pokemon slot for messages
        let slot = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.get_slot()
        } else {
            return EventResult::Continue;
        };

        // TODO: Check if effect is Booster Energy and set fromBooster
        // For now, just add the activate message
        battle.add("-activate", &[
            Arg::String(slot.clone()),
            Arg::String("ability: Protosynthesis".to_string()),
        ]);

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

        // TODO: Store bestStat in effectState (need to implement effect state storage)
        // For now, we'll rely on getBestStat being called again in the modify callbacks

        // Add start message with best stat
        let stat_name = match best_stat {
            StatID::Atk => "Atk",
            StatID::Def => "Def",
            StatID::SpA => "SpA",
            StatID::SpD => "SpD",
            StatID::Spe => "Spe",
            _ => "Atk", // Fallback
        };

        battle.add("-start", &[
            Arg::String(slot),
            Arg::String(format!("protosynthesis{}", stat_name)),
        ]);

        EventResult::Continue
    }

    /// onModifyAtk(atk, pokemon) {
    ///     if (this.effectState.bestStat !== 'atk' || pokemon.ignoringAbility()) return;
    ///     this.debug('Protosynthesis atk boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_atk(battle: &mut Battle, atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get best stat by computing it inline to avoid borrow issues
        let stats = [
            crate::dex_data::StatID::Atk,
            crate::dex_data::StatID::Def,
            crate::dex_data::StatID::SpA,
            crate::dex_data::StatID::SpD,
            crate::dex_data::StatID::Spe,
        ];
        let mut best_stat = crate::dex_data::StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = battle.get_pokemon_stat(attacker_pos, stat, false, true);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        if best_stat != StatID::Atk {
            return EventResult::Continue;
        }

        // TODO: Check if pokemon is ignoring ability

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(atk, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifyDef(def, pokemon) {
    ///     if (this.effectState.bestStat !== 'def' || pokemon.ignoringAbility()) return;
    ///     this.debug('Protosynthesis def boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_def(battle: &mut Battle, def: i32, defender_pos: (usize, usize), _attacker_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get best stat by computing it inline to avoid borrow issues
        let stats = [
            crate::dex_data::StatID::Atk,
            crate::dex_data::StatID::Def,
            crate::dex_data::StatID::SpA,
            crate::dex_data::StatID::SpD,
            crate::dex_data::StatID::Spe,
        ];
        let mut best_stat = crate::dex_data::StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = battle.get_pokemon_stat(defender_pos, stat, false, true);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        if best_stat != StatID::Def {
            return EventResult::Continue;
        }

        // TODO: Check if pokemon is ignoring ability

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(def, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifySpA(spa, pokemon) {
    ///     if (this.effectState.bestStat !== 'spa' || pokemon.ignoringAbility()) return;
    ///     this.debug('Protosynthesis spa boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_sp_a(battle: &mut Battle, spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get best stat by computing it inline to avoid borrow issues
        let stats = [
            crate::dex_data::StatID::Atk,
            crate::dex_data::StatID::Def,
            crate::dex_data::StatID::SpA,
            crate::dex_data::StatID::SpD,
            crate::dex_data::StatID::Spe,
        ];
        let mut best_stat = crate::dex_data::StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = battle.get_pokemon_stat(attacker_pos, stat, false, true);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        if best_stat != StatID::SpA {
            return EventResult::Continue;
        }

        // TODO: Check if pokemon is ignoring ability

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(spa, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifySpD(spd, pokemon) {
    ///     if (this.effectState.bestStat !== 'spd' || pokemon.ignoringAbility()) return;
    ///     this.debug('Protosynthesis spd boost');
    ///     return this.chainModify([5325, 4096]);
    /// }
    pub fn on_modify_sp_d(battle: &mut Battle, spd: i32, defender_pos: (usize, usize), _attacker_pos: (usize, usize), _move_id: &str) -> EventResult {
        // Get best stat by computing it inline to avoid borrow issues
        let stats = [
            crate::dex_data::StatID::Atk,
            crate::dex_data::StatID::Def,
            crate::dex_data::StatID::SpA,
            crate::dex_data::StatID::SpD,
            crate::dex_data::StatID::Spe,
        ];
        let mut best_stat = crate::dex_data::StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = battle.get_pokemon_stat(defender_pos, stat, false, true);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        if best_stat != StatID::SpD {
            return EventResult::Continue;
        }

        // TODO: Check if pokemon is ignoring ability

        // Apply 1.3x boost (5325/4096)
        let modified = battle.modify(spd, 5325, 4096);
        EventResult::Number(modified)
    }

    /// onModifySpe(spe, pokemon) {
    ///     if (this.effectState.bestStat !== 'spe' || pokemon.ignoringAbility()) return;
    ///     this.debug('Protosynthesis spe boost');
    ///     return this.chainModify(1.5);
    /// }
    pub fn on_modify_spe(battle: &mut Battle, spe: i32, pokemon_pos: (usize, usize)) -> EventResult {
        // Get best stat by computing it inline to avoid borrow issues
        let stats = [
            crate::dex_data::StatID::Atk,
            crate::dex_data::StatID::Def,
            crate::dex_data::StatID::SpA,
            crate::dex_data::StatID::SpD,
            crate::dex_data::StatID::Spe,
        ];
        let mut best_stat = crate::dex_data::StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = battle.get_pokemon_stat(pokemon_pos, stat, false, true);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        if best_stat != StatID::Spe {
            return EventResult::Continue;
        }

        // TODO: Check if pokemon is ignoring ability

        // Apply 1.5x boost (3/2 ratio)
        let modified = battle.modify(spe, 3, 2);
        EventResult::Number(modified)
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Protosynthesis');
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
            Arg::String("Protosynthesis".to_string()),
        ]);

        EventResult::Continue
    }
}
