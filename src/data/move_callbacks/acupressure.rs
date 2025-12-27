//! Acupressure Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::BoostID;

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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Collect stats that can be boosted (boost < 6)
    let boostable_stats = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let mut stats = Vec::new();

        if target.boosts.atk < 6 {
            stats.push(BoostID::Atk);
        }
        if target.boosts.def < 6 {
            stats.push(BoostID::Def);
        }
        if target.boosts.spa < 6 {
            stats.push(BoostID::SpA);
        }
        if target.boosts.spd < 6 {
            stats.push(BoostID::SpD);
        }
        if target.boosts.spe < 6 {
            stats.push(BoostID::Spe);
        }
        if target.boosts.accuracy < 6 {
            stats.push(BoostID::Accuracy);
        }
        if target.boosts.evasion < 6 {
            stats.push(BoostID::Evasion);
        }

        stats
    };

    if !boostable_stats.is_empty() {
        let random_stat = match battle.sample(&boostable_stats) {
            Some(stat) => stat,
            None => return EventResult::Continue,
        };

        // Convert BoostID to string for battle.boost()
        let stat_name = match random_stat {
            BoostID::Atk => "atk",
            BoostID::Def => "def",
            BoostID::SpA => "spa",
            BoostID::SpD => "spd",
            BoostID::Spe => "spe",
            BoostID::Accuracy => "accuracy",
            BoostID::Evasion => "evasion",
        };

        battle.boost(&[(stat_name, 2)], target_pos, Some(pokemon_pos), None);
        EventResult::Continue
    } else {
        EventResult::Bool(false)
    }
}

