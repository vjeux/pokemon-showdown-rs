//! Purify Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.cureStatus()) {
///         this.add('-fail', source);
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     this.heal(Math.ceil(source.maxhp * 0.5), source);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!target.cureStatus()) {
    let cured = battle.cure_status(target, None, None);

    if !cured {
        // this.add('-fail', source);
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(source_pokemon)
        };

        battle.add("-fail", &[source_arg]);

        // this.attrLastMove('[still]');
        battle.attr_last_move("[still]");

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // this.heal(Math.ceil(source.maxhp * 0.5), source);
    let heal_amount = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.max_hp + 1) / 2 // Math.ceil equivalent for integer division
    };

    battle.heal(heal_amount, source, None, None);

    EventResult::Continue
}

