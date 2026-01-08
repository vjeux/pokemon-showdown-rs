//! Ripen Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHeal(damage, target, source, effect) {
///     if (!effect) return;
///     if (effect.name === 'Berry Juice' || effect.name === 'Leftovers') {
///         this.add('-activate', target, 'ability: Ripen');
///     }
///     if ((effect as Item).isBerry) return this.chainModify(2);
/// }
pub fn on_try_heal(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (!effect) return;
    let effect_id = match effect_id {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // if (effect.name === 'Berry Juice' || effect.name === 'Leftovers')
    if effect_id == "berryjuice" || effect_id == "leftovers" {
        // this.add('-activate', target, 'ability: Ripen');
        if let Some(target_pos) = target_pos {
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target.get_slot()
            };

            battle.add("-activate", &[
                Arg::String(target_slot),
                Arg::Str("ability: Ripen"),
            ]);
        }
    }

    // if ((effect as Item).isBerry) return this.chainModify(2);
    if let Some(item_data) = battle.dex.items().get(effect_id) {
        if item_data.is_berry {
            return EventResult::Number(2);
        }
    }

    EventResult::Continue
}

/// onChangeBoost(boost, target, source, effect) {
///     if (effect && (effect as Item).isBerry) {
///         let b: BoostID;
///         for (b in boost) {
///             boost[b]! *= 2;
///         }
///     }
/// }
pub fn on_change_boost(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // if (effect && (effect as Item).isBerry)
    if let Some(effect_id) = effect_id {
        if let Some(item_data) = battle.dex.items().get(effect_id) {
            if item_data.is_berry {
                // for (b in boost) { boost[b]! *= 2; }
                // The boost table is in battle.event.relay_var
                if let Some(ref mut event) = battle.event {
                    if let Some(EventResult::Boost(ref mut boost_table)) = event.relay_var {
                        // Double all boosts
                        boost_table.atk *= 2;
                        boost_table.def *= 2;
                        boost_table.spa *= 2;
                        boost_table.spd *= 2;
                        boost_table.spe *= 2;
                        boost_table.accuracy *= 2;
                        boost_table.evasion *= 2;
                    }
                }
            }
        }
    }

    EventResult::Continue
}

/// onSourceModifyDamage(damage, source, target, move) {
///     if (target.abilityState.berryWeaken) {
///         target.abilityState.berryWeaken = false;
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target.abilityState.berryWeaken)
    let berry_weaken = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.ability_state.berry_weaken.unwrap_or(false)
    };

    if berry_weaken {
        // target.abilityState.berryWeaken = false;
        {
            let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.ability_state.berry_weaken = Some(false);
        }

        // return this.chainModify(0.5);
        battle.chain_modify_fraction(1, 2); return EventResult::Continue;
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     this.add('-activate', pokemon, 'ability: Ripen');
/// }
pub fn on_try_eat_item(battle: &mut Battle, _item_id: Option<&str>, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // this.add('-activate', pokemon, 'ability: Ripen');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(pokemon_slot),
        Arg::Str("ability: Ripen"),
    ]);

    EventResult::Continue
}

/// onEatItem(item, pokemon) {
///     const weakenBerries = [
///         'Babiri Berry', 'Charti Berry', 'Chilan Berry', 'Chople Berry', 'Coba Berry', 'Colbur Berry', 'Haban Berry', 'Kasib Berry', 'Kebia Berry', 'Occa Berry', 'Passho Berry', 'Payapa Berry', 'Rindo Berry', 'Roseli Berry', 'Shuca Berry', 'Tanga Berry', 'Wacan Berry', 'Yache Berry',
///     ];
///     // Record if the pokemon ate a berry to resist the attack
///     pokemon.abilityState.berryWeaken = weakenBerries.includes(item.name);
/// }
pub fn on_eat_item(battle: &mut Battle, _item_id: Option<&str>, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // const weakenBerries = [...]
    const WEAKEN_BERRIES: &[&str] = &[
        "babiriberry",
        "chartiberry",
        "chilanberry",
        "chopleberry",
        "cobaberry",
        "colburberry",
        "habanberry",
        "kasibberry",
        "kebiaberry",
        "occaberry",
        "passhoberry",
        "payapaberry",
        "rindoberry",
        "roseliberry",
        "shucaberry",
        "tangaberry",
        "wacanberry",
        "yacheberry",
    ];

    // Get the item from battle.event.effect
    let item_id = match &battle.event {
        Some(event) => match &event.effect {
            Some(id) => id.id.as_str(),
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // pokemon.abilityState.berryWeaken = weakenBerries.includes(item.name);
    let berry_weaken = WEAKEN_BERRIES.contains(&item_id);

    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon.ability_state.berry_weaken = Some(berry_weaken);

    EventResult::Continue
}

