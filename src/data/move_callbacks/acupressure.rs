//! Acupressure Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::BoostID;
use crate::event::EventResult;

/// onHit(target) {
///     const stats: BoostID[] = [];
///     let stat: BoostID;
///     for (stat in target.boosts) {
///         if (target.boosts[stat] < 6) {
///             stats.push(stat);
///         }
///     }
///     if (stats.length) {
///         const randomStat = this.sample(stats);
///         const boost: SparseBoostsTable = {};
///         boost[randomStat] = 2;
///         this.boost(boost);
///     } else {
///         return false;
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target pokemon
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // const stats: BoostID[] = [];
    let mut stats: Vec<BoostID> = Vec::new();

    // for (stat in target.boosts) {
    //     if (target.boosts[stat] < 6) {
    //         stats.push(stat);
    //     }
    // }
    for boost_id in BoostID::all() {
        if target_pokemon.boosts.get(*boost_id) < 6 {
            stats.push(*boost_id);
        }
    }

    // if (stats.length) {
    if !stats.is_empty() {
        // const randomStat = this.sample(stats);
        let random_stat = match battle.sample(&stats) {
            Some(stat) => stat,
            None => return EventResult::Continue,
        };

        // const boost: SparseBoostsTable = {};
        // boost[randomStat] = 2;
        // this.boost(boost);
        let boost_name = match random_stat {
            BoostID::Atk => "atk",
            BoostID::Def => "def",
            BoostID::SpA => "spa",
            BoostID::SpD => "spd",
            BoostID::Spe => "spe",
            BoostID::Accuracy => "accuracy",
            BoostID::Evasion => "evasion",
        };
        battle.boost(&[(boost_name, 2)], target, Some(pokemon_pos), None, false, false);

        EventResult::Continue
    } else {
        // return false;
        EventResult::Boolean(false)
    }
}
