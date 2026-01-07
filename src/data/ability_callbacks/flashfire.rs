//! Flash Fire Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Fire') {
///         move.accuracy = true;
///         if (!target.addVolatile('flashfire')) {
///             this.add('-immune', target, '[from] ability: Flash Fire');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), _move_id: &str) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // if (target !== source && move.type === 'Fire')
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let move_type = if let Some(ref active_move) = battle.active_move {
        active_move.move_type.clone()
    } else {
        return EventResult::Continue;
    };

    if move_type != "Fire" {
        return EventResult::Continue;
    }

    // move.accuracy = true;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.accuracy = crate::dex::Accuracy::AlwaysHits;
    }

    // if (!target.addVolatile('flashfire'))
    let added = Pokemon::add_volatile(battle, target_pos, ID::from("flashfire"), Some(source_pos), None, None,
            None);

    if !added {
        // this.add('-immune', target, '[from] ability: Flash Fire');
        let target_id = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };

        battle.add("-immune", &[
            Arg::String(target_id),
            Arg::Str("[from] ability: Flash Fire"),
        ]);
    }

    // return null;
    EventResult::Null
}

/// onEnd(pokemon) {
///     pokemon.removeVolatile('flashfire');
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // pokemon.removeVolatile('flashfire');
    Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("flashfire"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'ability: Flash Fire');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
        use crate::battle::Arg;

        // this.add('-start', target, 'ability: Flash Fire');
        let pokemon_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-start", &[
            Arg::String(pokemon_id),
            Arg::Str("ability: Flash Fire"),
        ]);

        EventResult::Continue
    }

    /// onModifyAtk(atk, attacker, defender, move) {
    ///     if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
    ///         this.debug('Flash Fire boost');
    ///         return this.chainModify(1.5);
    ///     }
    /// }
    pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.type === 'Fire' && attacker.hasAbility('flashfire'))
        let move_type = if let Some(ref active_move) = battle.active_move {
            active_move.move_type.clone()
        } else {
            return EventResult::Continue;
        };

        if move_type != "Fire" {
            return EventResult::Continue;
        }

        let has_flash_fire = {
            let attacker = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            attacker.ability == ID::from("flashfire") || attacker.base_ability == ID::from("flashfire")
        };

        if !has_flash_fire {
            return EventResult::Continue;
        }

        // this.debug('Flash Fire boost');
        // return this.chainModify(1.5);
        { battle.chain_modify(1.5); EventResult::Continue }
    }

    /// onModifySpA(atk, attacker, defender, move) {
    ///     if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
    ///         this.debug('Flash Fire boost');
    ///         return this.chainModify(1.5);
    ///     }
    /// }
    pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.type === 'Fire' && attacker.hasAbility('flashfire'))
        let move_type = if let Some(ref active_move) = battle.active_move {
            active_move.move_type.clone()
        } else {
            return EventResult::Continue;
        };

        if move_type != "Fire" {
            return EventResult::Continue;
        }

        let has_flash_fire = {
            let attacker = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            attacker.ability == ID::from("flashfire") || attacker.base_ability == ID::from("flashfire")
        };

        if !has_flash_fire {
            return EventResult::Continue;
        }

        // this.debug('Flash Fire boost');
        // return this.chainModify(1.5);
        { battle.chain_modify(1.5); EventResult::Continue }
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'ability: Flash Fire', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::battle::Arg;

        // this.add('-end', target, 'ability: Flash Fire', '[silent]');
        let pokemon_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-end", &[
            Arg::String(pokemon_id),
            Arg::Str("ability: Flash Fire"),
            Arg::Str("[silent]"),
        ]);

        EventResult::Continue
    }
}

