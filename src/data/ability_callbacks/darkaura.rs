//! Dark Aura Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Dark Aura');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some(pokemon_pos)) {
        return EventResult::Continue;
    }

    // this.add('-ability', pokemon, 'Dark Aura');
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-ability", &[
        Arg::String(pokemon_id),
        Arg::Str("Dark Aura"),
    ]);

    EventResult::Continue
}

/// onAnyBasePower(basePower, source, target, move) {
///     if (target === source || move.category === 'Status' || move.type !== 'Dark') return;
///     if (!move.auraBooster?.hasAbility('Dark Aura')) move.auraBooster = this.effectState.target;
///     if (move.auraBooster !== this.effectState.target) return;
///     return this.chainModify([move.hasAuraBreak ? 3072 : 5448, 4096]);
/// }
pub fn on_any_base_power(battle: &mut Battle, _base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Get the Dark Aura holder
    let aura_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target === source || move.category === 'Status' || move.type !== 'Dark') return;
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let (category, move_type) = if let Some(ref active_move) = battle.active_move {
        (active_move.category.clone(), active_move.move_type.clone())
    } else {
        return EventResult::Continue;
    };

    if category == "Status" || move_type != "Dark" {
        return EventResult::Continue;
    }

    // if (!move.auraBooster?.hasAbility('Dark Aura')) move.auraBooster = this.effectState.target;
    // if (move.auraBooster !== this.effectState.target) return;
    let should_boost = if let Some(ref mut active_move) = battle.active_move {
        if active_move.aura_booster.is_none() {
            active_move.aura_booster = Some(aura_holder);
        }
        active_move.aura_booster == Some(aura_holder)
    } else {
        false
    };

    if !should_boost {
        return EventResult::Continue;
    }

    // return this.chainModify([move.hasAuraBreak ? 3072 : 5448, 4096]);
    // IMPORTANT: In JavaScript, chainModify() returns undefined, not the new modifier!
    // So "return this.chainModify(...)" actually returns undefined, keeping relay_var unchanged.
    // The modifier is accumulated in event.modifier and applied at the end of runEvent.
    let has_aura_break = if let Some(ref active_move) = battle.active_move {
        active_move.has_aura_break.unwrap_or(false)
    } else {
        false
    };

    if has_aura_break {
        battle.chain_modify_fraction(3072, 4096); // 0.75x with Aura Break
    } else {
        battle.chain_modify_fraction(5448, 4096); // 1.33x normally
    }

    // Return Continue (undefined in JS) - the modifier is applied via event.modifier
    EventResult::Continue
}
