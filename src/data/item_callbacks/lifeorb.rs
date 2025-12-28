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
    EventResult::Number(battle.chain_modify_fraction(5324, 4096))
}

/// onAfterMoveSecondarySelf(source, target, move) {
///     if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag) {
///         this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag)
    let (source_base_maxhp, should_damage) = {
        // Check if source !== target
        if let Some(target) = target_pos {
            if source_pos == target {
                return EventResult::Continue;
            }
        }

        // Get active move
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // move.category !== 'Status'
        if active_move.category == "Status" {
            return EventResult::Continue;
        }

        // Get source pokemon
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // !source.forceSwitchFlag
        if source.force_switch_flag {
            return EventResult::Continue;
        }

        (source.base_maxhp, true)
    };

    if should_damage {
        // this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
        let lifeorb_id = ID::from("lifeorb");
        battle.damage(source_base_maxhp / 10, Some(source_pos), Some(source_pos), Some(&lifeorb_id), false);
    }

    EventResult::Continue
}
