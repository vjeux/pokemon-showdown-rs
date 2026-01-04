//! Dive Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

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
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
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

    let has_volatile = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.has_volatile(&move_id)
    };

    if has_volatile {
        Pokemon::remove_volatile(battle, attacker, &move_id);
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
        attacker_pokemon.has_ability(battle, &["gulpmissile"])
            && attacker_pokemon.species_id.as_str() == "Cramorant"
            && !attacker_pokemon.transformed
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

        // attacker.formeChange(forme, move);
        // Use position-based forme_change
        use crate::dex_data::ID;
        crate::pokemon::Pokemon::forme_change(
            battle,
            attacker,
            ID::from(forme),
            Some(move_id.clone()),
            false,
            "",
            None,
        );
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_ident, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        (attacker_pokemon.get_slot(), move_name)
    };

    battle.add(
        "-prepare",
        &[attacker_ident.as_str().into(), move_name.into()],
    );

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(attacker), defender, None, EventResult::Continue, false, false);
    if match charge_result { EventResult::Number(n) => n, _ => 0 } == 0 {
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, None, None, None);

    // return null;
    EventResult::Stop
}

pub mod condition {
    use super::*;

    /// onImmunity(type, pokemon) {
    ///     if (type === 'sandstorm' || type === 'hail') return false;
    /// }
    pub fn on_immunity(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // Get the immunity type from the event's relay_var_type
        let immunity_type = match &battle.current_event {
            Some(event) => event.relay_var_type.clone(),
            None => return EventResult::Continue,
        };

        // if (type === 'sandstorm' || type === 'hail') return false;
        if let Some(type_str) = immunity_type {
            if type_str == "sandstorm" || type_str == "hail" {
                // return false; - grant immunity to sandstorm and hail while underwater
                return EventResult::Boolean(false);
            }
        }

        EventResult::Continue
    }

    /// onInvulnerability(target, source, move) {
    ///     if (['surf', 'whirlpool'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_invulnerability(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
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
    pub fn on_source_modify_damage(
        battle: &mut Battle,
        _damage: i32,
        _source_pos: Option<(usize, usize)>,
        _target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // if (move.id === 'surf' || move.id === 'whirlpool') {
        //     return this.chainModify(2);
        // }
        if move_id == "surf" || move_id == "whirlpool" {
            // return this.chainModify(2);
            return EventResult::Number(battle.chain_modify(2.0_f32));
        }

        EventResult::Continue
    }
}
