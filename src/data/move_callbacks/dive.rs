//! Dive Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryMove(attacker, defender, move) {
///     if (attacker.removeVolatile(move.id)) {
///         return;
///     }
///     if (attacker.hasAbility('gulpmissile') && attacker.species.name === 'Cramorant' && !attacker.transformed) {
///         const forme = attacker.hp <= attacker.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
///         attacker.formeChange(forme, move);
///     }
///     this.add('-prepare', attacker, move.name);
///     if (!this.runEvent('ChargeMove', attacker, defender, move)) {
///         return;
///     }
///     attacker.addVolatile('twoturnmove', defender);
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let attacker = source_pos;
    let defender = target_pos;

    // Get move ID - for dive it should be "dive"
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    let removed = {
        let attacker_pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.remove_volatile(&move_id)
    };

    if removed {
        // return;
        return EventResult::Continue;
    }

    // if (attacker.hasAbility('gulpmissile') && attacker.species.name === 'Cramorant' && !attacker.transformed) {
    //     const forme = attacker.hp <= attacker.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
    //     attacker.formeChange(forme, move);
    // }
    let should_forme_change = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.has_ability(&ID::from("gulpmissile"), battle) &&
            attacker_pokemon.species_id.as_str() == "Cramorant" &&
            !attacker_pokemon.transformed
    };

    if should_forme_change {
        let forme = {
            let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            if attacker_pokemon.hp <= attacker_pokemon.maxhp / 2 {
                "cramorantgorging"
            } else {
                "cramorantgulping"
            }
        };

        let attacker_pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.forme_change(forme, Some(&move_id));
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_arg, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let move_data = battle.dex.get_move_by_id(&move_id);
        let move_name = move_data.map(|m| m.name.clone()).unwrap_or_else(|| move_id.to_string());

        (crate::battle::Arg::from(attacker_pokemon), move_name)
    };

    battle.add("-prepare", &[attacker_arg, move_name.into()]);

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(attacker), defender, None, None);
    if charge_result.unwrap_or(0) == 0 {
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    let attacker_pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    attacker_pokemon.add_volatile(ID::from("twoturnmove"));

    // return null;
    EventResult::Stop
}

pub mod condition {
    use super::*;

    /// onImmunity(type, pokemon) {
    ///     if (type === 'sandstorm' || type === 'hail') return false;
    /// }
    pub fn on_immunity(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // This callback needs additional parameters - the immunity type
        // The signature doesn't match the TypeScript version which takes (type, pokemon)
        // TODO: This needs to be called with the type parameter to work correctly
        // For now, we'll return Continue to not interfere
        EventResult::Continue
    }

    /// onInvulnerability(target, source, move) {
    ///     if (['surf', 'whirlpool'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // if (['surf', 'whirlpool'].includes(move.id)) {
        //     return;
        // }
        if move_id == "surf" || move_id == "whirlpool" {
            // return;
            return EventResult::Continue;
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.id === 'surf' || move.id === 'whirlpool') {
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // if (move.id === 'surf' || move.id === 'whirlpool') {
        //     return this.chainModify(2);
        // }
        if move_id == "surf" || move_id == "whirlpool" {
            // return this.chainModify(2);
            return EventResult::Number(battle.chain_modify(2.0 as f32));
        }

        EventResult::Continue
    }
}
