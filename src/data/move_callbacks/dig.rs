//! Dig Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onTryMove(attacker, defender, move) {
///     if (attacker.removeVolatile(move.id)) {
///         return;
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

    // Get move ID - for dig it should be "dig"
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
    let charge_result = battle.run_event("ChargeMove", Some(attacker), defender, None, None);
    if charge_result.unwrap_or(0) == 0 {
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, None);

    // return null;
    EventResult::Stop
}

pub mod condition {
    use super::*;

    /// onImmunity(type, pokemon) {
    ///     if (type === 'sandstorm' || type === 'hail') return false;
    /// }
    pub fn on_immunity(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // This callback needs additional parameters - the immunity type
        // The signature doesn't match the TypeScript version which takes (type, pokemon)
        // TODO: This needs to be called with the type parameter to work correctly
        // For now, we'll return Continue to not interfere
        EventResult::Continue
    }

    /// onInvulnerability(target, source, move) {
    ///     if (['earthquake', 'magnitude'].includes(move.id)) {
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
        // if (['earthquake', 'magnitude'].includes(move.id)) {
        //     return;
        // }
        if move_id == "earthquake" || move_id == "magnitude" {
            // return;
            return EventResult::Continue;
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.id === 'earthquake' || move.id === 'magnitude') {
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
        // if (move.id === 'earthquake' || move.id === 'magnitude') {
        //     return this.chainModify(2);
        // }
        if move_id == "earthquake" || move_id == "magnitude" {
            // return this.chainModify(2);
            return EventResult::Number(battle.chain_modify(2.0_f32));
        }

        EventResult::Continue
    }
}
