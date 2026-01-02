//! Starf Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
    //     pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
    //     pokemon.eatItem();
    // }

    let should_eat = {
        if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            let quarter_hp = pokemon.maxhp / 4;
            let half_hp = pokemon.maxhp / 2;

            if pokemon.hp <= quarter_hp {
                true
            } else if pokemon.hp <= half_hp {
                let has_gluttony = pokemon.has_ability(battle, &["gluttony"]);
                let gluttony_active = pokemon.ability_state.data.get("gluttony")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                has_gluttony && gluttony_active
            } else {
                false
            }
        } else {
            false
        }
    };

    if should_eat {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.eat_item(false);
        }
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     const stats: BoostID[] = [];
///     let stat: BoostID;
///     for (stat in pokemon.boosts) {
///         if (stat !== 'accuracy' && stat !== 'evasion' && pokemon.boosts[stat] < 6) {
///             stats.push(stat);
///         }
///     }
///     if (stats.length) {
///         const randomStat = this.sample(stats);
///         const boost: SparseBoostsTable = {};
///         boost[randomStat] = 2;
///         this.boost(boost);
///     }
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // const stats: BoostID[] = [];
    // for (stat in pokemon.boosts) {
    //     if (stat !== 'accuracy' && stat !== 'evasion' && pokemon.boosts[stat] < 6) {
    //         stats.push(stat);
    //     }
    // }
    // if (stats.length) {
    //     const randomStat = this.sample(stats);
    //     const boost: SparseBoostsTable = {};
    //     boost[randomStat] = 2;
    //     this.boost(boost);
    // }

    let boostable_stats = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let mut stats = Vec::new();
        if pokemon.boosts.atk < 6 {
            stats.push("atk");
        }
        if pokemon.boosts.def < 6 {
            stats.push("def");
        }
        if pokemon.boosts.spa < 6 {
            stats.push("spa");
        }
        if pokemon.boosts.spd < 6 {
            stats.push("spd");
        }
        if pokemon.boosts.spe < 6 {
            stats.push("spe");
        }
        stats
    };

    if !boostable_stats.is_empty() {
        if let Some(&random_stat) = battle.sample(&boostable_stats) {
            battle.boost(&[(random_stat, 2)], pokemon_pos, None, None, false, false);
        }
    }

    EventResult::Continue
}
