//! Enigma Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target, source, move) {
///     if (move && target.getMoveHitData(move).typeMod > 0) {
///         if (target.eatItem()) {
///             this.heal(target.baseMaxhp / 4);
///         }
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move && target.getMoveHitData(move).typeMod > 0)
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if move is super effective
    // Get move ID from active_move (clone to avoid borrow issues)
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // target.getMoveHitData(move).typeMod > 0 means super effective
    let type_effectiveness = Pokemon::run_effectiveness(battle, target_pos, &move_id);

    let (is_super_effective, target_base_maxhp, target_hp) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        (type_effectiveness > 0, target.base_maxhp, target.hp)
    };

    eprintln!("[ENIGMABERRY] Turn {}: on_hit called for move '{}', type_effectiveness={}, is_super_effective={}, target HP={}/{}",
        battle.turn, move_id, type_effectiveness, is_super_effective, target_hp, target_base_maxhp);

    if !is_super_effective {
        eprintln!("[ENIGMABERRY] Turn {}: NOT super-effective, returning early (no heal)", battle.turn);
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
        let heal_amount = target_base_maxhp / 4;
        eprintln!("[ENIGMABERRY] Turn {}: Berry eaten! Healing {} HP", battle.turn, heal_amount);
        battle.heal(heal_amount, Some(target_pos), source_pos, Some(&Effect::item(crate::dex_data::ID::from("enigmaberry"))));
    } else {
        eprintln!("[ENIGMABERRY] Turn {}: Berry NOT eaten", battle.turn);
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
        pokemon.base_maxhp / 4
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
