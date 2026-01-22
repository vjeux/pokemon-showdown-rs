//! Sticky Barb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;
use crate::Pokemon;

/// onResidual(pokemon) {
///     this.damage(pokemon.baseMaxhp / 8);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get pokemon's base_maxhp
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    // this.damage(pokemon.baseMaxhp / 8);
    battle.damage(hp_fraction(base_maxhp, 8), Some(pokemon_pos), None, None, false);

    EventResult::Continue
}

/// onHit(target, source, move) {
///     if (source && source !== target && !source.item && move && this.checkMoveMakesContact(move, source, target)) {
///         const barb = target.takeItem();
///         if (!barb) return; // Gen 4 Multitype
///         source.setItem(barb);
///         // no message for Sticky Barb changing hands
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (source && source !== target && !source.item && move && this.checkMoveMakesContact(move, source, target)) {
    //     const barb = target.takeItem();
    //     if (!barb) return;
    //     source.setItem(barb);
    // }

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source !== target
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    // Check if source has no item
    let source_has_item = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !source.item.is_empty()
    };

    if source_has_item {
        return EventResult::Continue;
    }

    // Check if move makes contact
    // Use check_move_makes_contact_with_active_move to check the active move's flags
    // (which may have been modified by onModifyMove, e.g., skydrop removing contact)
    if !battle.check_move_makes_contact_with_active_move(active_move, source_pos, target_pos, false) {
        return EventResult::Continue;
    }

    // Take item from target
    let barb = Pokemon::take_item(battle, target_pos, Some(source_pos));

    // if (!barb) return;
    let barb = match barb {
        Some(item) => item,
        None => return EventResult::Continue,
    };

    // Give item to source
    Pokemon::set_item(battle, source_pos, barb, None, None);

    EventResult::Continue
}
