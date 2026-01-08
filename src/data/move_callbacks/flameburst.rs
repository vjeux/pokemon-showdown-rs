//! Flame Burst Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     for (const ally of target.adjacentAllies()) {
///         this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // for (const ally of target.adjacentAllies()) {
    let adjacent_allies = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.adjacent_allies(battle)
    };

    let effect = Effect::condition(ID::from("flameburst"));

    for ally_pos in adjacent_allies {
        // this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
        let base_max_hp = {
            let ally_pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally_pokemon.base_maxhp
        };

        battle.damage(
            base_max_hp / 16,
            Some(ally_pos),
            Some(source),
            Some(&effect),
            false,
        );
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     for (const ally of target.adjacentAllies()) {
///         this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
///     }
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // for (const ally of target.adjacentAllies()) {
    let adjacent_allies = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.adjacent_allies(battle)
    };

    let effect = Effect::condition(ID::from("flameburst"));

    for ally_pos in adjacent_allies {
        // this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
        let base_max_hp = {
            let ally_pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally_pokemon.base_maxhp
        };

        battle.damage(
            base_max_hp / 16,
            Some(ally_pos),
            Some(source),
            Some(&effect),
            false,
        );
    }

    EventResult::Continue
}
