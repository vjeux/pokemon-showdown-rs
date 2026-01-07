//! Beads of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Beads of Ruin');
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
        Arg::Str("Beads of Ruin"),
    ]);

    EventResult::Continue
}

/// onAnyModifySpD(spd, target, source, move) {
///     const abilityHolder = this.effectState.target;
///     if (target.hasAbility('Beads of Ruin')) return;
///     if (!move.ruinedSpD?.hasAbility('Beads of Ruin')) move.ruinedSpD = abilityHolder;
///     if (move.ruinedSpD !== abilityHolder) return;
///     this.debug('Beads of Ruin SpD drop');
///     return this.chainModify(0.75);
/// }
pub fn on_any_modify_sp_d(battle: &mut Battle, _spd: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if let Some(tpos) = target_pos {
        let target_has_beads = {
            let target = match battle.pokemon_at(tpos.0, tpos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.has_ability(battle, &["beadsofruin"])
        };

        if target_has_beads {
            return EventResult::Continue;
        }
    }

    let (_current_ruined, need_to_update) = {
        if let Some(ref active_move) = battle.active_move {
            if let Some(ruined_pos) = active_move.ruined_spd {
                let ruined_has_beads = {
                    if let Some(ruined) = battle.pokemon_at(ruined_pos.0, ruined_pos.1) {
                        ruined.has_ability(battle, &["beadsofruin"])
                    } else {
                        false
                    }
                };
                (Some(ruined_pos), !ruined_has_beads)
            } else {
                (None, true)
            }
        } else {
            return EventResult::Continue;
        }
    };

    if need_to_update {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.ruined_spd = Some(ability_holder);
        }
    }

    let should_apply = {
        if let Some(ref active_move) = battle.active_move {
            active_move.ruined_spd == Some(ability_holder)
        } else {
            false
        }
    };

    if should_apply {
        eprintln!("Beads of Ruin SpD drop");
        return EventResult::Number(battle.chain_modify(0.75));
    }

    EventResult::Continue
}

