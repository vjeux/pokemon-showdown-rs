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
pub fn on_modify_damage(battle: &mut Battle, _damage: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // return this.chainModify([5324, 4096]);
    battle.chain_modify_fraction(5324, 4096);
    EventResult::Continue
}

/// onAfterMoveSecondarySelf(source, target, move) {
///     if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag) {
///         this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    battle.debug(&format!("[LIFE ORB] on_after_move_secondary_self called: source={:?}, target={:?}", source_pos, target_pos));

    // if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag)
    let target = match target_pos {
        Some(pos) => pos,
        None => {
            battle.debug("[LIFE ORB] No target, returning Continue");
            return EventResult::Continue;
        }
    };

    // source !== target
    if source_pos == target {
        battle.debug("[LIFE ORB] source == target, returning Continue");
        return EventResult::Continue;
    }

    // move && move.category !== 'Status'
    let is_status_move = {
        match &battle.active_move {
            Some(active_move) => active_move.category == "Status",
            None => {
                battle.debug("[LIFE ORB] No active_move, returning Continue");
                return EventResult::Continue;
            }
        }
    };

    if is_status_move {
        battle.debug("[LIFE ORB] Status move, returning Continue");
        return EventResult::Continue;
    }

    // !source.forceSwitchFlag
    let (base_maxhp, force_switch) = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => {
                battle.debug("[LIFE ORB] No pokemon at source_pos, returning Continue");
                return EventResult::Continue;
            }
        };
        (pokemon.base_maxhp, pokemon.force_switch_flag)
    };

    if force_switch {
        battle.debug("[LIFE ORB] force_switch is true, returning Continue");
        return EventResult::Continue;
    }

    battle.debug(&format!("[LIFE ORB] Applying recoil damage: {} HP", base_maxhp / 10));
    // this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
    battle.damage(base_maxhp / 10, Some(source_pos), Some(source_pos), None, false);

    EventResult::Continue
}
