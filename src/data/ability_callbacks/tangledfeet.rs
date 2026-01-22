//! Tangled Feet Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAccuracy(accuracy, target) {
///     if (typeof accuracy !== 'number') return;
///     if (target?.volatiles['confusion']) {
///         this.debug('Tangled Feet - decreasing accuracy');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_modify_accuracy(battle: &mut Battle, _accuracy: i32, target_pos: (usize, usize), _source_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target?.volatiles['confusion'])
    let has_confusion = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.volatiles.contains_key(&crate::ID::from("confusion"))
    };

    if has_confusion {
        // this.debug('Tangled Feet - decreasing accuracy');
        // return this.chainModify(0.5);
        battle.chain_modify(0.5);
        return EventResult::Continue;
    }

    EventResult::Continue
}

