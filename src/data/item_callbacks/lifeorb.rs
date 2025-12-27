//! Life Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	lifeorb: {
//! 		name: "Life Orb",
//! 		spritenum: 249,
//! 		fling: {
//! 			basePower: 30,
//! 		},
//! 		onModifyDamage(damage, source, target, move) {
//! 			return this.chainModify([5324, 4096]);
//! 		},
//! 		onAfterMoveSecondarySelf(source, target, move) {
//! 			if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag) {
//! 				this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
//! 			}
//! 		},
//! 		num: 270,
//! 		gen: 4,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move)
pub fn on_modify_damage(_battle: &Battle, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    // JS: return this.chainModify([5324, 4096]);
    EventResult::Number(5324) // 5324/4096 = ~1.3x
}

/// onAfterMoveSecondarySelf(source, target, move)
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // JS: if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag)
    // TODO: Need to check move category and forceSwitchFlag

    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // JS: this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
    let damage = source.maxhp / 10;
    battle.damage(damage as i32, Some(source_pos), Some(source_pos), Some(&ID::new("lifeorb")), false);
    EventResult::Continue
}
