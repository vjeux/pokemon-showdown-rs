//! Fairy Aura Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Fairy Aura');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some(pokemon_pos)) {
        return EventResult::Continue;
    }

    // this.add('-ability', pokemon, 'Fairy Aura');
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-ability", &[
        Arg::String(pokemon_id),
        Arg::Str("Fairy Aura"),
    ]);

    EventResult::Continue
}

/// onAnyBasePower(basePower, source, target, move) {
///     if (target === source || move.category === 'Status' || move.type !== 'Fairy') return;
///     if (!move.auraBooster?.hasAbility('Fairy Aura')) move.auraBooster = this.effectState.target;
///     if (move.auraBooster !== this.effectState.target) return;
///     return this.chainModify([move.hasAuraBreak ? 3072 : 5448, 4096]);
/// }
pub fn on_any_base_power(battle: &mut Battle, _base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // Get the Fairy Aura holder
    let aura_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target === source || move.category === 'Status' || move.type !== 'Fairy') return;
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let (category, move_type) = if let Some(ref active_move) = battle.active_move {
        (active_move.category.clone(), active_move.move_type.clone())
    } else {
        return EventResult::Continue;
    };

    if category == "Status" || move_type != "Fairy" {
        return EventResult::Continue;
    }

    // if (!move.auraBooster?.hasAbility('Fairy Aura')) move.auraBooster = this.effectState.target;
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
    let has_aura_break = if let Some(ref active_move) = battle.active_move {
        active_move.has_aura_break.unwrap_or(false)
    } else {
        false
    };

    let modifier = if has_aura_break {
        battle.chain_modify_fraction(3072, 4096) // 0.75x with Aura Break
    } else {
        battle.chain_modify_fraction(5448, 4096) // 1.33x normally
    };

    EventResult::Number(modifier)
}

