//! Enigma Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target, source, move) {
///     if (move && target.getMoveHitData(move).typeMod > 0) {
///         if (target.eatItem()) {
///             this.heal(target.baseMaxhp / 4);
///         }
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let _move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // if (move && target.getMoveHitData(move).typeMod > 0)
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // JavaScript: getMoveHitData only has data for moves that dealt damage
    // Status moves (like Shore Up) don't deal damage, so they won't have hit data
    // and typeMod will be undefined/0. We should skip status moves.
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Status moves don't have hit data - they didn't deal damage
    if active_move_ref.category == "Status" {
        return EventResult::Continue;
    }

    // Also skip if target is the same as source (self-targeting damaging moves shouldn't trigger)
    if let Some(source) = source_pos {
        if source == target_pos {
            return EventResult::Continue;
        }
    }

    let move_id = active_move_ref.id.as_str();

    // JavaScript: target.getMoveHitData(move).typeMod > 0
    // IMPORTANT: Use the stored type_mod from move hit data, NOT recalculated run_effectiveness!
    // For moves with damageCallback (like Counter), modify_damage is never called,
    // so type_mod is never set and defaults to 0. This correctly prevents Enigma Berry
    // from triggering for these fixed-damage moves.
    let type_effectiveness = battle.get_move_hit_data(target_pos)
        .map(|hit_data| hit_data.type_mod as i32)
        .unwrap_or(0);

    let (is_super_effective, target_base_maxhp, target_hp) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        (type_effectiveness > 0, target.base_maxhp, target.hp)
    };

    debug_elog!("[ENIGMABERRY] Turn {}: on_hit called for move '{}', type_effectiveness={}, is_super_effective={}, target HP={}/{}",
        battle.turn, move_id, type_effectiveness, is_super_effective, target_hp, target_base_maxhp);

    if !is_super_effective {
        debug_elog!("[ENIGMABERRY] Turn {}: NOT super-effective, returning early (no heal)", battle.turn);
        return EventResult::Continue;
    }

    // if (target.eatItem())
    let item_eaten = {
        let _target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::eat_item(battle, target_pos, false, None, None).is_some()
    };

    if item_eaten {
        // this.heal(target.baseMaxhp / 4);
        let heal_amount = hp_fraction(target_base_maxhp, 4);
        debug_elog!("[ENIGMABERRY] Turn {}: Berry eaten! Healing {} HP", battle.turn, heal_amount);
        battle.heal(heal_amount, Some(target_pos), source_pos, Some(&Effect::item(crate::dex_data::ID::from("enigmaberry"))));
    } else {
        debug_elog!("[ENIGMABERRY] Turn {}: Berry NOT eaten", battle.turn);
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     if (!this.runEvent('TryHeal', pokemon, null, this.effect, pokemon.baseMaxhp / 4)) return false;
/// }
pub fn on_try_eat_item(battle: &mut Battle, _item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.runEvent('TryHeal', pokemon, null, this.effect, pokemon.baseMaxhp / 4)) return false;
    let heal_amount = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(pokemon.base_maxhp, 4)
    };

    use crate::dex_data::ID;
    // Run TryHeal event to check if healing would succeed
    let event_result = battle.run_event("TryHeal", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, Some(&Effect::item(ID::from("enigmaberry"))), EventResult::Number(heal_amount), false, false);

    match event_result {
        EventResult::Number(amount) if amount > 0 => EventResult::Continue,
        _ => EventResult::Boolean(false),
    }
}

/// onEat() {
///     num: 208,
///     gen: 3,
/// }
pub fn on_eat(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // onEat callback has no implementation - just metadata
    EventResult::Continue
}
