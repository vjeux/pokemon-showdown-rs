//! Sword of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Sword of Ruin');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some(pokemon_pos)) {
        return EventResult::Continue;
    }

    // this.add('-ability', pokemon, 'Sword of Ruin');
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let side_id = format!("p{}", pokemon.side_index + 1);
        if pokemon.is_active {
            let pos_letter = (b'a' + pokemon.position as u8) as char;
            format!("{}{}: {}", side_id, pos_letter, pokemon.name)
        } else {
            format!("{}: {}", side_id, pokemon.name)
        }
    };

    battle.add("-ability", &[
        Arg::String(pokemon_id),
        Arg::Str("Sword of Ruin"),
    ]);

    EventResult::Continue
}

/// onAnyModifyDef(def, target, source, move) {
///     const abilityHolder = this.effectState.target;
///     if (target.hasAbility('Sword of Ruin')) return;
///     if (!move.ruinedDef?.hasAbility('Sword of Ruin')) move.ruinedDef = abilityHolder;
///     if (move.ruinedDef !== abilityHolder) return;
///     this.debug('Sword of Ruin Def drop');
///     return this.chainModify(0.75);
/// }
pub fn on_any_modify_def(battle: &mut Battle, _def: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // const abilityHolder = this.effectState.target;
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.hasAbility('Sword of Ruin')) return;
    if let Some(tpos) = target_pos {
        let target_has_sword = {
            let target = match battle.pokemon_at(tpos.0, tpos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.has_ability(battle, &["swordofruin"])
        };

        if target_has_sword {
            return EventResult::Continue;
        }
    }

    // if (!move.ruinedDef?.hasAbility('Sword of Ruin')) move.ruinedDef = abilityHolder;
    // if (move.ruinedDef !== abilityHolder) return;

    // First check if ruinedDef is set and if that Pokemon still has the ability
    let (_current_ruined, need_to_update) = {
        if let Some(ref active_move) = battle.active_move {
            if let Some(ruined_pos) = active_move.ruined_def {
                // Check if the Pokemon at ruined_pos still has Sword of Ruin
                let ruined_has_sword = {
                    if let Some(ruined) = battle.pokemon_at(ruined_pos.0, ruined_pos.1) {
                        ruined.has_ability(battle, &["swordofruin"])
                    } else {
                        false
                    }
                };
                (Some(ruined_pos), !ruined_has_sword)
            } else {
                (None, true)
            }
        } else {
            return EventResult::Continue;
        }
    };

    // Now update if needed
    if need_to_update {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.ruined_def = Some(ability_holder);
        }
    }

    // Check if this ability holder is the one affecting the move
    let should_apply = {
        if let Some(ref active_move) = battle.active_move {
            active_move.ruined_def == Some(ability_holder)
        } else {
            false
        }
    };

    if should_apply {
        // this.debug('Sword of Ruin Def drop');
        eprintln!("Sword of Ruin Def drop");
        // return this.chainModify(0.75);
        battle.chain_modify(0.75); return EventResult::Continue;
    }

    EventResult::Continue
}

