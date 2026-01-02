//! Destiny Knot Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onAttract(target, source) {
///     this.debug(`attract intercepted: ${target} from ${source}`);
///     if (!source || source === target) return;
///     if (!source.volatiles['attract']) source.addVolatile('attract', target);
/// }
pub fn on_attract(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // this.debug(`attract intercepted: ${target} from ${source}`);
    // if (!source || source === target) return;
    // if (!source.volatiles['attract']) source.addVolatile('attract', target);

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source === target) return;
    if source == target {
        return EventResult::Continue;
    }

    battle.debug(&format!("attract intercepted: {:?} from {:?}", target, source));

    // if (!source.volatiles['attract']) source.addVolatile('attract', target);
    let has_attract = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_volatile(&ID::new("attract"))
    };

    if !has_attract {
        Pokemon::add_volatile(battle, source, ID::new("attract"), Some(target));
    }

    EventResult::Continue
}
