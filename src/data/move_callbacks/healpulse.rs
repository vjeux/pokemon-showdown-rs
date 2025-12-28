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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success = false;
    let success;

    // if (source.hasAbility('megalauncher')) {
    let has_megalauncher = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_ability(&ID::from("megalauncher"))
    };

    if has_megalauncher {
        // success = !!this.heal(this.modify(target.baseMaxhp, 0.75));
        let heal_amount = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            battle.modify_value(target_pokemon.base_maxhp as f64, 0.75) as i32
        };
        success = battle.heal(heal_amount, target, Some(target), None) > 0;
    } else {
        // success = !!this.heal(Math.ceil(target.baseMaxhp * 0.5));
        let heal_amount = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            ((target_pokemon.base_maxhp as f64 * 0.5).ceil() as i32)
        };
        success = battle.heal(heal_amount, target, Some(target), None) > 0;
    }

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
        let target_arg = crate::battle::Arg::Pos(target.0, target.1);
        battle.add("-fail", &[target_arg, "heal".into()]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // return success;
    EventResult::Boolean(success)
}

