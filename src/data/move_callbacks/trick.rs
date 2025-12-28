//! Trick Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return !target.hasAbility('stickyhold');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return !target.hasAbility('stickyhold');
    let has_stickyhold = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_ability(&["stickyhold"])
    };

    // Return the negation - if has stickyhold, return false (immune), otherwise true (not immune)
    EventResult::Boolean(!has_stickyhold)
}

/// onHit(target, source, move) {
///     const yourItem = target.takeItem(source);
///     const myItem = source.takeItem();
///     if (target.item || source.item || (!yourItem && !myItem)) {
///         if (yourItem) target.item = yourItem.id;
///         if (myItem) source.item = myItem.id;
///         return false;
///     }
///     if (
///         (myItem && !this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem)) ||
///         (yourItem && !this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem))
///     ) {
///         if (yourItem) target.item = yourItem.id;
///         if (myItem) source.item = myItem.id;
///         return false;
///     }
///     this.add('-activate', source, 'move: Trick', `[of] ${target}`);
///     if (myItem) {
///         target.setItem(myItem);
///         this.add('-item', target, myItem, '[from] move: Trick');
///     } else {
///         this.add('-enditem', target, yourItem, '[silent]', '[from] move: Trick');
///     }
///     if (yourItem) {
///         source.setItem(yourItem);
///         this.add('-item', source, yourItem, '[from] move: Trick');
///     } else {
///         this.add('-enditem', source, myItem, '[silent]', '[from] move: Trick');
///     }
/// }
pub fn on_hit(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
