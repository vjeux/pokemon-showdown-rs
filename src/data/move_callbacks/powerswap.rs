//! Power Swap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// onHit(target, source) {
///     const targetBoosts: SparseBoostsTable = {};
///     const sourceBoosts: SparseBoostsTable = {};
///
///     const atkSpa: BoostID[] = ['atk', 'spa'];
///     for (const stat of atkSpa) {
///         targetBoosts[stat] = target.boosts[stat];
///         sourceBoosts[stat] = source.boosts[stat];
///     }
///
///     source.setBoost(targetBoosts);
///     target.setBoost(sourceBoosts);
///
///     this.add('-swapboost', source, target, 'atk, spa', '[from] move: Power Swap');
/// }
/// ```
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use std::collections::HashMap;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetBoosts: SparseBoostsTable = {};
    // const sourceBoosts: SparseBoostsTable = {};
    //
    // const atkSpa: BoostID[] = ['atk', 'spa'];
    // for (const stat of atkSpa) {
    //     targetBoosts[stat] = target.boosts[stat];
    //     sourceBoosts[stat] = source.boosts[stat];
    // }
    let (target_boosts, source_boosts) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let mut target_boosts = HashMap::new();
        let mut source_boosts = HashMap::new();

        target_boosts.insert("atk".to_string(), target_pokemon.boosts.atk);
        target_boosts.insert("spa".to_string(), target_pokemon.boosts.spa);

        source_boosts.insert("atk".to_string(), source_pokemon.boosts.atk);
        source_boosts.insert("spa".to_string(), source_pokemon.boosts.spa);

        (target_boosts, source_boosts)
    };

    // source.setBoost(targetBoosts);
    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        for (stat, value) in &target_boosts {
            let boost_id = match stat.as_str() {
                "atk" => crate::dex_data::BoostID::Atk,
                "def" => crate::dex_data::BoostID::Def,
                "spa" => crate::dex_data::BoostID::SpA,
                "spd" => crate::dex_data::BoostID::SpD,
                "spe" => crate::dex_data::BoostID::Spe,
                "accuracy" => crate::dex_data::BoostID::Accuracy,
                "evasion" => crate::dex_data::BoostID::Evasion,
                _ => continue,
            };
            source_pokemon.set_boost(boost_id, *value);
        }
    }

    // target.setBoost(sourceBoosts);
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        for (stat, value) in &source_boosts {
            let boost_id = match stat.as_str() {
                "atk" => crate::dex_data::BoostID::Atk,
                "def" => crate::dex_data::BoostID::Def,
                "spa" => crate::dex_data::BoostID::SpA,
                "spd" => crate::dex_data::BoostID::SpD,
                "spe" => crate::dex_data::BoostID::Spe,
                "accuracy" => crate::dex_data::BoostID::Accuracy,
                "evasion" => crate::dex_data::BoostID::Evasion,
                _ => continue,
            };
            target_pokemon.set_boost(boost_id, *value);
        }
    }

    // this.add('-swapboost', source, target, 'atk, spa', '[from] move: Power Swap');
    let (source_arg, target_arg) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.get_slot(), target_pokemon.get_slot())
    };

    battle.add(
        "-swapboost",
        &[
            source_arg.into(),
            target_arg.into(),
            "atk, spa".into(),
            "[from] move: Power Swap".into(),
        ],
    );

    EventResult::Continue
}
