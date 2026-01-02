//! Fly Move
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
    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    {
        let attacker = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if attacker.volatiles.contains_key(&move_id) {
            attacker.remove_volatile(&move_id);
            // return;
            return EventResult::Continue;
        }
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_ident, move_name) = {
        let attacker = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        (attacker.get_slot(), move_name)
    };

    battle.add(
        "-prepare",
        &[attacker_ident.as_str().into(), move_name.into()],
    );

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    if battle
        .run_event("ChargeMove", Some(source_pos), target_pos, None, None)
        .unwrap_or(0)
        == 0
    {
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, source_pos, ID::from("twoturnmove"), target_pos, None, None);

    // return null;
    EventResult::Stop
}

pub mod condition {
    use super::*;

    /// onInvulnerability(target, source, move) {
    ///     if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
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
        // if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
        let valid_moves = [
            "gust",
            "twister",
            "skyuppercut",
            "thunder",
            "hurricane",
            "smackdown",
            "thousandarrows",
        ];

        if valid_moves.contains(&move_id) {
            // return;
            return EventResult::Continue;
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.id === 'gust' || move.id === 'twister') {
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
        // if (move.id === 'gust' || move.id === 'twister') {
        if move_id == "gust" || move_id == "twister" {
            // return this.chainModify(2);
            return EventResult::Number(battle.chain_modify(2.0));
        }

        EventResult::Continue
    }
}
