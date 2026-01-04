//! Commander Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnySwitchIn() {
///     ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // Call onUpdate for the effectState.target (the ability holder)
    let target_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    on_update(battle, target_pos)
}

/// onStart(pokemon) {
///     ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Call onUpdate for this pokemon
    on_update(battle, pokemon_pos)
}

/// onUpdate(pokemon) {
///     if (this.gameType !== 'doubles') return;
///     // don't run between when a Pokemon switches in and the resulting onSwitchIn event
///     if (this.queue.peek()?.choice === 'runSwitch') return;
///
///     const ally = pokemon.allies()[0];
///     if (pokemon.switchFlag || ally?.switchFlag) return;
///     if (!ally || pokemon.baseSpecies.baseSpecies !== 'Tatsugiri' || ally.baseSpecies.baseSpecies !== 'Dondozo') {
///         // Handle any edge cases
///         if (pokemon.getVolatile('commanding')) pokemon.removeVolatile('commanding');
///         return;
///     }
///
///     if (!pokemon.getVolatile('commanding')) {
///         // If Dondozo already was commanded this fails
///         if (ally.getVolatile('commanded')) return;
///         // Cancel all actions this turn for pokemon if applicable
///         this.queue.cancelAction(pokemon);
///         // Add volatiles to both pokemon
///         this.add('-activate', pokemon, 'ability: Commander', `[of] ${ally}`);
///         pokemon.addVolatile('commanding');
///         ally.addVolatile('commanded', pokemon);
///         // Continued in conditions.ts in the volatiles
///     } else {
///         if (!ally.fainted) return;
///         pokemon.removeVolatile('commanding');
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::{GameType, ID};
    use crate::pokemon::Pokemon;

    // if (this.gameType !== 'doubles') return;
    let is_doubles = match battle.game_type {
        GameType::Doubles => true,
        _ => false,
    };

    if !is_doubles {
        return EventResult::Continue;
    }

    // if (this.queue.peek()?.choice === 'runSwitch') return;
    if let Some(action) = battle.queue.peek() {
        if let crate::battle_queue::Action::Pokemon(pokemon_action) = action {
            if pokemon_action.choice == crate::battle_queue::PokemonActionType::RunSwitch {
                return EventResult::Continue;
            }
        }
    }

    // const ally = pokemon.allies()[0];
    let allies = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.allies(battle, false)
    };

    let ally_pos = if let Some(&pos) = allies.first() {
        pos
    } else {
        // if (!ally || pokemon.baseSpecies.baseSpecies !== 'Tatsugiri' || ally.baseSpecies.baseSpecies !== 'Dondozo') {
        //     // Handle any edge cases
        //     if (pokemon.getVolatile('commanding')) pokemon.removeVolatile('commanding');
        //     return;
        // }
        let has_commanding = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_volatile(&ID::from("commanding"))
        };

        if has_commanding {
            Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("commanding"));
        }
        return EventResult::Continue;
    };

    // if (pokemon.switchFlag || ally?.switchFlag) return;
    let (pokemon_switch_flag, ally_switch_flag) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.switch_flag.is_some(), ally.switch_flag.is_some())
    };

    if pokemon_switch_flag || ally_switch_flag {
        return EventResult::Continue;
    }

    // if (!ally || pokemon.baseSpecies.baseSpecies !== 'Tatsugiri' || ally.baseSpecies.baseSpecies !== 'Dondozo')
    let (pokemon_base_species, ally_base_species) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.base_species.clone(), ally.base_species.clone())
    };

    if pokemon_base_species.as_str() != "tatsugiri" || ally_base_species.as_str() != "dondozo" {
        // Handle any edge cases
        // if (pokemon.getVolatile('commanding')) pokemon.removeVolatile('commanding');
        let has_commanding = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_volatile(&ID::from("commanding"))
        };

        if has_commanding {
            Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("commanding"));
        }
        return EventResult::Continue;
    }

    // if (!pokemon.getVolatile('commanding')) {
    let has_commanding = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_volatile(&ID::from("commanding"))
    };

    if !has_commanding {
        // If Dondozo already was commanded this fails
        // if (ally.getVolatile('commanded')) return;
        let has_commanded = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            ally.has_volatile(&ID::from("commanded"))
        };

        if has_commanded {
            return EventResult::Continue;
        }

        // Cancel all actions this turn for pokemon if applicable
        // this.queue.cancelAction(pokemon);
        battle.queue.cancel_action(pokemon_pos.0, pokemon_pos.1);

        // Add volatiles to both pokemon
        // this.add('-activate', pokemon, 'ability: Commander', `[of] ${ally}`);
        let (pokemon_slot, ally_slot) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (pokemon.get_slot(), ally.get_slot())
        };

        battle.add("-activate", &[
            pokemon_slot.into(),
            "ability: Commander".into(),
            format!("[of] {}", ally_slot).into(),
        ]);

        // pokemon.addVolatile('commanding');
        Pokemon::add_volatile(battle, pokemon_pos, ID::from("commanding"), None, None, None, None);

        // ally.addVolatile('commanded', pokemon);
        Pokemon::add_volatile(battle, ally_pos, ID::from("commanded"), Some(pokemon_pos), None, None, None);
    } else {
        // if (!ally.fainted) return;
        let ally_fainted = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            ally.fainted
        };

        if !ally_fainted {
            return EventResult::Continue;
        }

        // pokemon.removeVolatile('commanding');
        Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("commanding"));
    }

    EventResult::Continue
}

