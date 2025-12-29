//! Max Geyser Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				if (!source.volatiles['dynamax']) return;
/// 				this.field.setWeather('raindance');
/// 			},
///
/// 		}
/// ```
pub fn on_hit(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!source.volatiles['dynamax']) return;
    let has_dynamax = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.has_volatile(&ID::from("dynamax"))
    };

    if !has_dynamax {
        return EventResult::Continue;
    }

    // this.field.setWeather('raindance');
    battle.field.set_weather(ID::from("raindance"), None);

    EventResult::Continue
}


