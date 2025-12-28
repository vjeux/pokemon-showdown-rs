//! Heart Swap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const targetBoosts: SparseBoostsTable = {};
///     const sourceBoosts: SparseBoostsTable = {};
/// 
///     let i: BoostID;
///     for (i in target.boosts) {
///         targetBoosts[i] = target.boosts[i];
///         sourceBoosts[i] = source.boosts[i];
///     }
/// 
///     target.setBoost(sourceBoosts);
///     source.setBoost(targetBoosts);
/// 
///     this.add('-swapboost', source, target, '[from] move: Heart Swap');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetBoosts: SparseBoostsTable = {};
    // const sourceBoosts: SparseBoostsTable = {};
    // let i: BoostID;
    // for (i in target.boosts) {
    //     targetBoosts[i] = target.boosts[i];
    //     sourceBoosts[i] = source.boosts[i];
    // }

    // Get all boost keys from both pokemon
    let target_boosts = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts.clone()
    };

    let source_boosts = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.boosts.clone()
    };

    // target.setBoost(sourceBoosts);
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts = source_boosts;
    }

    // source.setBoost(targetBoosts);
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.boosts = target_boosts;
    }

    // this.add('-swapboost', source, target, '[from] move: Heart Swap');
    let source_ident = {
        let pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    let target_ident = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add("-swapboost", &[source_ident.as_str().into(), target_ident.as_str().into(), "[from] move: Heart Swap".into()]);

    EventResult::Continue
}

