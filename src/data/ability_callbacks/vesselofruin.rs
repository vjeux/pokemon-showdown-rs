//! Vessel of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Vessel of Ruin');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    if battle.suppressing_ability(Some(pokemon_pos)) {
        return EventResult::Continue;
    }

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
        Arg::Str("Vessel of Ruin"),
    ]);

    EventResult::Continue
}

/// onAnyModifySpA(spa, source, target, move) {
///     const abilityHolder = this.effectState.target;
///     if (source.hasAbility('Vessel of Ruin')) return;
///     if (!move.ruinedSpA) move.ruinedSpA = abilityHolder;
///     if (move.ruinedSpA !== abilityHolder) return;
///     this.debug('Vessel of Ruin SpA drop');
///     return this.chainModify(0.75);
/// }
pub fn on_any_modify_sp_a(battle: &mut Battle, _spa: i32, source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if let Some(spos) = source_pos {
        let source_has_vessel = {
            let source = match battle.pokemon_at(spos.0, spos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source.has_ability(battle, &["vesselofruin"])
        };

        if source_has_vessel {
            return EventResult::Continue;
        }
    }

    let (_current_ruined, need_to_update) = {
        if let Some(ref active_move) = battle.active_move {
            if let Some(ruined_pos) = active_move.ruined_spa {
                let ruined_has_vessel = {
                    if let Some(ruined) = battle.pokemon_at(ruined_pos.0, ruined_pos.1) {
                        ruined.has_ability(battle, &["vesselofruin"])
                    } else {
                        false
                    }
                };
                (Some(ruined_pos), !ruined_has_vessel)
            } else {
                (None, true)
            }
        } else {
            return EventResult::Continue;
        }
    };

    if need_to_update {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.ruined_spa = Some(ability_holder);
        }
    }

    let should_apply = {
        if let Some(ref active_move) = battle.active_move {
            active_move.ruined_spa == Some(ability_holder)
        } else {
            false
        }
    };

    if should_apply {
        eprintln!("Vessel of Ruin SpA drop");
        return EventResult::Number(battle.chain_modify(0.75));
    }

    EventResult::Continue
}

