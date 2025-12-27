//! Guardian of Alola Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon, target) {
///     const hp75 = Math.floor(target.getUndynamaxedHP() * 3 / 4);
///     if (
///         target.volatiles['protect'] || target.volatiles['banefulbunker'] || target.volatiles['kingsshield'] ||
///         target.volatiles['spikyshield'] || target.side.getSideCondition('matblock')
///     ) {
///         this.add('-zbroken', target);
///         return this.clampIntRange(Math.ceil(hp75 / 4 - 0.5), 1);
///     }
///     return this.clampIntRange(hp75, 1);
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const hp75 = Math.floor(target.getUndynamaxedHP() * 3 / 4);
    let hp75 = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.get_undynamaxed_hp() * 3 / 4) as i32
    };

    // if (target.volatiles['protect'] || target.volatiles['banefulbunker'] || target.volatiles['kingsshield'] ||
    //     target.volatiles['spikyshield'] || target.side.getSideCondition('matblock'))
    let has_protection = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        target_pokemon.volatiles.contains_key(&ID::from("protect")) ||
        target_pokemon.volatiles.contains_key(&ID::from("banefulbunker")) ||
        target_pokemon.volatiles.contains_key(&ID::from("kingsshield")) ||
        target_pokemon.volatiles.contains_key(&ID::from("spikyshield"))
    };

    let has_matblock = {
        let side = &battle.sides[target.0];
        side.side_conditions.contains_key(&ID::from("matblock"))
    };

    if has_protection || has_matblock {
        // this.add('-zbroken', target);
        let target_arg = crate::battle::Arg::Pos(target.0, target.1);
        battle.add("-zbroken", &[target_arg]);

        // return this.clampIntRange(Math.ceil(hp75 / 4 - 0.5), 1);
        let damage = ((hp75 as f64 / 4.0 - 0.5).ceil() as i32).max(1);
        return EventResult::Int(damage);
    }

    // return this.clampIntRange(hp75, 1);
    let damage = hp75.max(1);
    EventResult::Int(damage)
}

