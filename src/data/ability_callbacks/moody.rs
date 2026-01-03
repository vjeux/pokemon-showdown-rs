//! Moody Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     let stats: BoostID[] = [];
///     const boost: SparseBoostsTable = {};
///     let statPlus: BoostID;
///     for (statPlus in pokemon.boosts) {
///         if (statPlus === 'accuracy' || statPlus === 'evasion') continue;
///         if (pokemon.boosts[statPlus] < 6) {
///             stats.push(statPlus);
///         }
///     }
///     let randomStat: BoostID | undefined = stats.length ? this.sample(stats) : undefined;
///     if (randomStat) boost[randomStat] = 2;
///
///     stats = [];
///     let statMinus: BoostID;
///     for (statMinus in pokemon.boosts) {
///         if (statMinus === 'accuracy' || statMinus === 'evasion') continue;
///         if (pokemon.boosts[statMinus] > -6 && statMinus !== randomStat) {
///             stats.push(statMinus);
///         }
///     }
///     randomStat = stats.length ? this.sample(stats) : undefined;
///     if (randomStat) boost[randomStat] = -1;
///
///     this.boost(boost, pokemon, pokemon);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::BoostID;

    // let stats: BoostID[] = [];
    // for (statPlus in pokemon.boosts) {
    //     if (statPlus === 'accuracy' || statPlus === 'evasion') continue;
    //     if (pokemon.boosts[statPlus] < 6) {
    //         stats.push(statPlus);
    //     }
    // }
    let boostable_stats: Vec<BoostID> = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        BoostID::stats_only()
            .iter()
            .filter(|&&stat| pokemon.boosts.get(stat) < 6)
            .copied()
            .collect()
    };

    // let randomStat: BoostID | undefined = stats.length ? this.sample(stats) : undefined;
    // if (randomStat) boost[randomStat] = 2;
    let stat_plus: Option<BoostID> = if !boostable_stats.is_empty() {
        battle.sample(&boostable_stats).map(|&stat| stat)
    } else {
        None
    };

    // stats = [];
    // for (statMinus in pokemon.boosts) {
    //     if (statMinus === 'accuracy' || statMinus === 'evasion') continue;
    //     if (pokemon.boosts[statMinus] > -6 && statMinus !== randomStat) {
    //         stats.push(statMinus);
    //     }
    // }
    let lowerable_stats: Vec<BoostID> = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        BoostID::stats_only()
            .iter()
            .filter(|&&stat| {
                pokemon.boosts.get(stat) > -6 && Some(stat) != stat_plus
            })
            .copied()
            .collect()
    };

    // randomStat = stats.length ? this.sample(stats) : undefined;
    // if (randomStat) boost[randomStat] = -1;
    let stat_minus: Option<BoostID> = if !lowerable_stats.is_empty() {
        battle.sample(&lowerable_stats).map(|&stat| stat)
    } else {
        None
    };

    // this.boost(boost, pokemon, pokemon);
    // Build the boost table
    let mut boost_list: Vec<(&str, i8)> = Vec::new();
    if let Some(stat) = stat_plus {
        let stat_str = match stat {
            BoostID::Atk => "atk",
            BoostID::Def => "def",
            BoostID::SpA => "spa",
            BoostID::SpD => "spd",
            BoostID::Spe => "spe",
            _ => return EventResult::Continue,
        };
        boost_list.push((stat_str, 2));
    }
    if let Some(stat) = stat_minus {
        let stat_str = match stat {
            BoostID::Atk => "atk",
            BoostID::Def => "def",
            BoostID::SpA => "spa",
            BoostID::SpD => "spd",
            BoostID::Spe => "spe",
            _ => return EventResult::Continue,
        };
        boost_list.push((stat_str, -1));
    }

    if !boost_list.is_empty() {
        battle.boost(&boost_list, pokemon_pos, Some(pokemon_pos), None, false, false);
    }

    EventResult::Continue
}

