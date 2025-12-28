//! Guard Swap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target, source) {
///     const targetBoosts: SparseBoostsTable = {};
///     const sourceBoosts: SparseBoostsTable = {};
/// 
///     const defSpd: BoostID[] = ['def', 'spd'];
///     for (const stat of defSpd) {
///         targetBoosts[stat] = target.boosts[stat];
///         sourceBoosts[stat] = source.boosts[stat];
///     }
/// 
///     source.setBoost(targetBoosts);
///     target.setBoost(sourceBoosts);
/// 
///     this.add('-swapboost', source, target, 'def, spd', '[from] move: Guard Swap');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetBoosts: SparseBoostsTable = {};
    // const sourceBoosts: SparseBoostsTable = {};
    // const defSpd: BoostID[] = ['def', 'spd'];
    // for (const stat of defSpd) {
    //     targetBoosts[stat] = target.boosts[stat];
    //     sourceBoosts[stat] = source.boosts[stat];
    // }

    let target_def_boost = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts.def
    };

    let target_spd_boost = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts.spd
    };

    let source_def_boost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.boosts.def
    };

    let source_spd_boost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.boosts.spd
    };

    // source.setBoost(targetBoosts);
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.boosts.insert("def".to_string(), target_def_boost);
        source_pokemon.boosts.insert("spd".to_string(), target_spd_boost);
    }

    // target.setBoost(sourceBoosts);
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts.insert("def".to_string(), source_def_boost);
        target_pokemon.boosts.insert("spd".to_string(), source_spd_boost);
    }

    // this.add('-swapboost', source, target, 'def, spd', '[from] move: Guard Swap');
    let source_arg = {
        let pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(pokemon)
    };
    let target_arg = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(pokemon)
    };
    battle.add("-swapboost", &[source_arg, target_arg, "def, spd".into(), "[from] move: Guard Swap".into()]);

    EventResult::Continue
}

