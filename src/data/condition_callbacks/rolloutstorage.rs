//! Rolloutstorage Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onBasePower
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBasePower(relayVar, source, target, move) {
///     let bp = Math.max(1, move.basePower);
///     bp *= 2 ** source.volatiles['rolloutstorage'].contactHitCount;
///     if (source.volatiles['defensecurl']) {
///         bp *= 2;
///     }
///     source.removeVolatile('rolloutstorage');
///     return bp;
/// }
/// ```
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // let bp = Math.max(1, move.basePower);
    let base_power = active_move_ref.base_power.max(1);

    // bp *= 2 ** source.volatiles['rolloutstorage'].contactHitCount;
    let contact_hit_count = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let rolloutstorage_id = ID::from("rolloutstorage");
        match pokemon.volatiles.get(&rolloutstorage_id) {
            Some(volatile) => volatile.contact_hit_count.unwrap_or(0),
            None => return EventResult::Continue,
        }
    };

    let mut bp = base_power * 2_i32.pow(contact_hit_count as u32);

    // if (source.volatiles['defensecurl'])
    let has_defensecurl = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let defensecurl_id = ID::from("defensecurl");
        pokemon.volatiles.contains_key(&defensecurl_id)
    };

    if has_defensecurl {
        // bp *= 2;
        bp *= 2;
    }

    // source.removeVolatile('rolloutstorage');
    let rolloutstorage_id = ID::from("rolloutstorage");
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &rolloutstorage_id);

    // return bp;
    EventResult::Number(bp)
}

