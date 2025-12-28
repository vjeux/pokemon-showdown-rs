//! Life Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move) {
///     return this.chainModify([5324, 4096]);
/// }
pub fn on_modify_damage(battle: &mut Battle, damage: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // return this.chainModify([5324, 4096]);
    battle.chain_modify_fraction(5324, 4096);
    EventResult::Continue
}

/// onAfterMoveSecondarySelf(source, target, move) {
///     if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag) {
///         this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag)
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source !== target
    if source_pos == target {
        return EventResult::Continue;
    }

    // move && move.category !== 'Status'
    let is_status_move = {
        match &battle.active_move {
            Some(active_move) => active_move.category == "Status",
            None => return EventResult::Continue,
        }
    };

    if is_status_move {
        return EventResult::Continue;
    }

    // !source.forceSwitchFlag
    let (base_maxhp, force_switch) = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.base_maxhp, pokemon.force_switch_flag)
    };

    if force_switch {
        return EventResult::Continue;
    }

    // this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
    battle.damage(base_maxhp / 10, Some(source_pos), Some(source_pos), None, false);

    EventResult::Continue
}
