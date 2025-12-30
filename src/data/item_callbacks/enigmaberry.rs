//! Enigma Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
    let (is_super_effective, target_base_maxhp) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get move type from active_move
        let move_type = match &battle.active_move {
            Some(active_move) => &active_move.move_type,
            None => return EventResult::Continue,
        };

        // target.getMoveHitData(move).typeMod > 0 means super effective
        let type_effectiveness = target.run_effectiveness(move_type);
        (type_effectiveness > 1.0, target.base_maxhp)
    };

    if !is_super_effective {
        return EventResult::Continue;
    }

    // if (target.eatItem())
    let item_eaten = {
        let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_mut.eat_item(false).is_some()
    };

    if item_eaten {
        // this.heal(target.baseMaxhp / 4);
        let heal_amount = target_base_maxhp / 4;
        use crate::dex_data::ID;
        battle.heal(heal_amount, Some(target_pos), source_pos, Some(&ID::from("enigmaberry")));
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
    let event_result = battle.run_event(
        "TryHeal",
        Some(pokemon_pos),
        None,
        Some(&ID::from("enigmaberry")),
        Some(heal_amount)
    );

    match event_result {
        Some(amount) if amount > 0 => EventResult::Continue,
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
