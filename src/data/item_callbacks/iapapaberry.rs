//! Iapapa Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

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
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     if (!this.runEvent('TryHeal', pokemon, null, this.effect, pokemon.baseMaxhp / 3)) return false;
/// }
pub fn on_try_eat_item(battle: &mut Battle, _item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.runEvent('TryHeal', pokemon, null, this.effect, pokemon.baseMaxhp / 3)) return false;

    let heal_amount = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp / 3
    };

    let result = battle.run_event("TryHeal", Some(pokemon_pos), None, None, Some(heal_amount));

    if result.is_none() {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.heal(pokemon.baseMaxhp / 3);
///     if (pokemon.getNature().minus === 'def') {
///         pokemon.addVolatile('confusion');
///     }
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.heal(pokemon.baseMaxhp / 3);
    // if (pokemon.getNature().minus === 'def') {
    //     pokemon.addVolatile('confusion');
    // }

    // Phase 1: Get heal amount and nature
    let (heal_amount, nature_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.base_maxhp / 3, pokemon.nature.clone())
    };

    // Phase 2: Heal
    battle.heal(heal_amount, Some(pokemon_pos), None, None);

    // Phase 3: Check nature and add confusion if minus stat is 'def'
    if let Some(nature_data) = battle.dex.natures().get(&nature_name) {
        if nature_data.minus.as_deref() == Some("def") {
            Pokemon::add_volatile(battle, pokemon_pos, "confusion".into(), None, None, None, None);
        }
    }

    EventResult::Continue
}
