//! Tablets of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Tablets of Ruin');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
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
        Arg::Str("Tablets of Ruin"),
    ]);

    EventResult::Continue
}

/// onAnyModifyAtk(atk, source, target, move) {
///     const abilityHolder = this.effectState.target;
///     if (source.hasAbility('Tablets of Ruin')) return;
///     if (!move.ruinedAtk) move.ruinedAtk = abilityHolder;
///     if (move.ruinedAtk !== abilityHolder) return;
///     this.debug('Tablets of Ruin Atk drop');
///     return this.chainModify(0.75);
/// }
pub fn on_any_modify_atk(battle: &mut Battle, _atk: i32, source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if let Some(spos) = source_pos {
        let source_has_tablets = {
            let source = match battle.pokemon_at(spos.0, spos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source.has_ability(battle, &["tabletsofruin"])
        };

        if source_has_tablets {
            return EventResult::Continue;
        }
    }

    let (_current_ruined, need_to_update) = {
        if let Some(ref active_move) = battle.active_move {
            if let Some(ruined_pos) = active_move.ruined_atk {
                let ruined_has_tablets = {
                    if let Some(ruined) = battle.pokemon_at(ruined_pos.0, ruined_pos.1) {
                        ruined.has_ability(battle, &["tabletsofruin"])
                    } else {
                        false
                    }
                };
                (Some(ruined_pos), !ruined_has_tablets)
            } else {
                (None, true)
            }
        } else {
            return EventResult::Continue;
        }
    };

    if need_to_update {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.ruined_atk = Some(ability_holder);
        }
    }

    let should_apply = {
        if let Some(ref active_move) = battle.active_move {
            active_move.ruined_atk == Some(ability_holder)
        } else {
            false
        }
    };

    if should_apply {
        eprintln!("Tablets of Ruin Atk drop");
        battle.chain_modify(0.75); return EventResult::Continue;
    }

    EventResult::Continue
}

