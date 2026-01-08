//! Cursed Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (source.volatiles['disable']) return;
///     if (!move.isMax && !move.flags['futuremove'] && move.id !== 'struggle') {
///         if (this.randomChance(3, 10)) {
///             source.addVolatile('disable', this.effectState.target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::Pokemon;

    let active_move = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };
    let move_id = active_move.id.as_str();

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.volatiles['disable']) return;
    let source_has_disable = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.volatiles.contains_key(&crate::dex_data::ID::from("disable"))
    };

    if source_has_disable {
        return EventResult::Continue;
    }

    // if (!move.isMax && !move.flags['futuremove'] && move.id !== 'struggle')
    let should_activate = !active_move.is_max
        && !active_move.flags.future_move
        && move_id != "struggle";

    if should_activate {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // source.addVolatile('disable', this.effectState.target);
            Pokemon::add_volatile(battle, source_pos, crate::dex_data::ID::from("disable"), Some(target_pos), None, None,
            None);
        }
    }

    EventResult::Continue
}

