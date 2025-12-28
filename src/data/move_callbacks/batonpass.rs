//! Baton Pass Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onHit(target) {
///     if (!this.canSwitch(target.side) || target.volatiles['commanded']) {
///         this.attrLastMove('[still]');
///         this.add('-fail', target);
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target (which is the user of Baton Pass)
    let target = target_pos.unwrap_or(pokemon_pos);

    // Get target pokemon
    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // if (!this.canSwitch(target.side) || target.volatiles['commanded']) {
    let can_switch_count = battle.can_switch(target.0);
    let has_commanded = target_pokemon.volatiles.contains_key(&ID::from("commanded"));

    if can_switch_count == 0 || has_commanded {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // Get target identifier for battle.add
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            format!("p{}{}", target.0 + 1, target_pokemon.ident)
        };

        // this.add('-fail', target);
        battle.add("-fail", &[target_ident.as_str().into()]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    EventResult::Continue
}

