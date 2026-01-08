//! Heal Pulse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     let success = false;
///     if (source.hasAbility('megalauncher')) {
///         success = !!this.heal(this.modify(target.baseMaxhp, 0.75));
///     } else {
///         success = !!this.heal(Math.ceil(target.baseMaxhp * 0.5));
///     }
///     if (success && !target.isAlly(source)) {
///         target.staleness = 'external';
///     }
///     if (!success) {
///         this.add('-fail', target, 'heal');
///         return this.NOT_FAIL;
///     }
///     return success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target (the Pokemon being healed)
    source_pos: Option<(usize, usize)>,  // Second param is source (the Pokemon using the move)
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success = false;
    // if (source.hasAbility('megalauncher')) {
    let has_megalauncher = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_ability(battle, &["megalauncher"])
    };

    let success = if has_megalauncher {
        // success = !!this.heal(this.modify(target.baseMaxhp, 0.75));
        let heal_amount = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            battle.modify_f(target_pokemon.base_maxhp, 0.75)
        };
        battle
            .heal(heal_amount, Some(target), Some(target), None)
            .unwrap_or(0)
            > 0
    } else {
        // success = !!this.heal(Math.ceil(target.baseMaxhp * 0.5));
        let heal_amount = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.base_maxhp as f64 * 0.5).ceil() as i32
        };
        battle
            .heal(heal_amount, Some(target), Some(target), None)
            .unwrap_or(0)
            > 0
    };

    // if (success && !target.isAlly(source)) {
    let is_ally = target.0 == source.0;

    if success && !is_ally {
        // target.staleness = 'external';
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.staleness = Some("external".to_string());
    }

    // if (!success) {
    if !success {
        // this.add('-fail', target, 'heal');
        let target_ident = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add("-fail", &[target_ident.as_str().into(), "heal".into()]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // return success;
    EventResult::Boolean(success)
}
