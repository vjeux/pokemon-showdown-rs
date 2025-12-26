//! Costar Ability - Copies ally's stat boosts on switch-in

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SWITCH_IN_PRIORITY: i32 = -2;

/// onStart(pokemon)
/// Copies ally's stat boosts and critical hit volatiles
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // const ally = pokemon.allies()[0];
    // if (!ally) return;
    let side = &battle.sides[pokemon.side_index];
    let mut ally_data: Option<(String, i8, i8, i8, i8, i8, i8, i8)> = None;

    for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
        if ally.position == pokemon.position {
            continue;
        }
        // Found an ally - store its name and boosts
        ally_data = Some((
            ally.name.clone(),
            ally.boosts.atk,
            ally.boosts.def,
            ally.boosts.spa,
            ally.boosts.spd,
            ally.boosts.spe,
            ally.boosts.accuracy,
            ally.boosts.evasion,
        ));
        break;
    }

    let (ally_name, atk, def, spa, spd, spe, accuracy, evasion) = match ally_data {
        Some(data) => data,
        None => return AbilityHandlerResult::Undefined,
    };

    // for (i in ally.boosts) { pokemon.boosts[i] = ally.boosts[i]; }
    let holder_ref = (pokemon.side_index, pokemon.position);
    let holder = &mut battle.sides[holder_ref.0].pokemon[holder_ref.1];
    holder.boosts.atk = atk;
    holder.boosts.def = def;
    holder.boosts.spa = spa;
    holder.boosts.spd = spd;
    holder.boosts.spe = spe;
    holder.boosts.accuracy = accuracy;
    holder.boosts.evasion = evasion;

    // const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
    // for (const volatile of volatilesToCopy) pokemon.removeVolatile(volatile);
    let volatiles_to_copy = ["dragoncheer", "focusenergy", "gmaxchistrike", "laserfocus"];
    for volatile in volatiles_to_copy.iter() {
        battle.sides[holder_ref.0].pokemon[holder_ref.1].remove_volatile(&ID::new(volatile));
    }

    // Collect ally volatile data first
    let ally_ref = (pokemon.side_index, {
        let side = &battle.sides[pokemon.side_index];
        side.pokemon.iter().find(|p| p.is_active && !p.fainted && p.position != pokemon.position)
            .map(|a| a.position)
            .unwrap_or(0)
    });

    let mut ally_volatiles: Vec<(&str, Option<u32>, Option<bool>)> = Vec::new();
    if let Some(ally) = battle.sides.get(ally_ref.0).and_then(|s| s.pokemon.get(ally_ref.1)) {
        if ally.is_active && !ally.fainted && ally.position != pokemon.position {
            for volatile_id in volatiles_to_copy.iter() {
                if ally.has_volatile(&ID::new(volatile_id)) {
                    let layers = if *volatile_id == "gmaxchistrike" {
                        ally.volatiles.get(&ID::new(volatile_id))
                            .and_then(|v| v.layers)
                    } else {
                        None
                    };
                    let has_dragon = if *volatile_id == "dragoncheer" {
                        ally.volatiles.get(&ID::new(volatile_id))
                            .and_then(|v| v.data.get("hasDragonType"))
                            .and_then(|val| val.as_bool())
                    } else {
                        None
                    };
                    ally_volatiles.push((*volatile_id, layers, has_dragon));
                }
            }
        }
    }

    // for (const volatile of volatilesToCopy) {
    //     if (ally.volatiles[volatile]) {
    //         pokemon.addVolatile(volatile);
    //         if (volatile === 'gmaxchistrike') pokemon.volatiles[volatile].layers = ally.volatiles[volatile].layers;
    //         if (volatile === 'dragoncheer') pokemon.volatiles[volatile].hasDragonType = ally.volatiles[volatile].hasDragonType;
    //     }
    // }
    for (volatile_id, layers_opt, has_dragon_opt) in ally_volatiles {
        battle.sides[holder_ref.0].pokemon[holder_ref.1].add_volatile(ID::new(volatile_id));

        if let Some(layers) = layers_opt {
            if let Some(volatile) = battle.sides[holder_ref.0].pokemon[holder_ref.1]
                .volatiles.get_mut(&ID::new(volatile_id)) {
                volatile.layers = Some(layers);
            }
        }

        if let Some(has_dragon) = has_dragon_opt {
            if let Some(volatile) = battle.sides[holder_ref.0].pokemon[holder_ref.1]
                .volatiles.get_mut(&ID::new(volatile_id)) {
                volatile.data.insert("hasDragonType".to_string(), serde_json::json!(has_dragon));
            }
        }
    }

    // this.add('-copyboost', pokemon, ally, '[from] ability: Costar');
    battle.add("-copyboost", &[
        Arg::Pokemon(pokemon),
        Arg::String(ally_name),
        Arg::Str("[from] ability: Costar")
    ]);

    AbilityHandlerResult::Undefined
}
