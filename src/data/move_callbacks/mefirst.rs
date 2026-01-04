//! Me First Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// ```ignore
/// onTryHit(target, pokemon) {
///     const action = this.queue.willMove(target);
///     if (!action) return false;
///     const move = this.dex.getActiveMove(action.move.id);
///     if (action.zmove || move.isZ || move.isMax) return false;
///     if (target.volatiles['mustrecharge']) return false;
///     if (move.category === 'Status' || move.flags['failmefirst']) return false;
///
///     pokemon.addVolatile('mefirst');
///     this.actions.useMove(move, pokemon, { target });
///     return null;
/// }
/// ```
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let pokemon = source_pos;
    let target = target_pos;

    // const action = this.queue.willMove(target);
    // if (!action) return false;
    let action = battle.queue.will_move(target.0, target.1);
    if action.is_none() {
        return EventResult::Boolean(false);
    }

    let action = action.unwrap();

    // const move = this.dex.getActiveMove(action.move.id);
    let move_id = action.move_id.clone();
    let move_data = match battle.dex.moves().get_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Boolean(false),
    };

    // if (action.zmove || move.isZ || move.isMax) return false;
    let zmove = action.zmove.is_some();
    if zmove || move_data.is_z_or_max_powered {
        return EventResult::Boolean(false);
    }

    // if (target.volatiles['mustrecharge']) return false;
    let has_mustrecharge = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        target_pokemon
            .volatiles
            .contains_key(&ID::from("mustrecharge"))
    };

    if has_mustrecharge {
        return EventResult::Boolean(false);
    }

    // if (move.category === 'Status' || move.flags['failmefirst']) return false;
    if move_data.category == "Status" || move_data.flags.contains_key("failmefirst") {
        return EventResult::Boolean(false);
    }

    // pokemon.addVolatile('mefirst');
    Pokemon::add_volatile(battle, pokemon, ID::from("mefirst"), None, None, None, None);

    // this.actions.useMove(move, pokemon, { target });
    crate::battle_actions::use_move(battle, &move_id, pokemon, Some(target), None, None, None);

    // return null;
    EventResult::Stop
}

pub mod condition {
    use super::*;

    /// onBasePower(basePower) {
    ///     return this.chainModify(1.5);
    /// }
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // return this.chainModify(1.5);
        EventResult::Number(battle.chain_modify_fraction(3, 2)) // 1.5 = 3/2
    }
}
