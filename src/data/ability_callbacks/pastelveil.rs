//! Pastel Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const ally of pokemon.alliesAndSelf()) {
///         if (['psn', 'tox'].includes(ally.status)) {
///             this.add('-activate', pokemon, 'ability: Pastel Veil');
///             ally.cureStatus();
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::Pokemon;

    // for (const ally of pokemon.alliesAndSelf())
    let allies_and_self = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.allies_and_self(battle, false)
    };

    let mut activated = false;
    for ally_pos in allies_and_self {
        // if (['psn', 'tox'].includes(ally.status))
        let has_poison = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally.status.as_str() == "psn" || ally.status.as_str() == "tox"
        };

        if has_poison {
            if !activated {
                // this.add('-activate', pokemon, 'ability: Pastel Veil');
                let pokemon_slot = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.get_slot()
                };

                battle.add("-activate", &[
                    Arg::String(pokemon_slot),
                    Arg::Str("ability: Pastel Veil"),
                ]);
                activated = true;
            }

            // ally.cureStatus();
            Pokemon::cure_status(battle, ally_pos, false);
        }
    }

    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (['psn', 'tox'].includes(pokemon.status)) {
///         this.add('-activate', pokemon, 'ability: Pastel Veil');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::Pokemon;

    // if (['psn', 'tox'].includes(pokemon.status))
    let has_poison = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.as_str() == "psn" || pokemon.status.as_str() == "tox"
    };

    if has_poison {
        // this.add('-activate', pokemon, 'ability: Pastel Veil');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(pokemon_slot),
            Arg::Str("ability: Pastel Veil"),
        ]);

        // pokemon.cureStatus();
        Pokemon::cure_status(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

/// onAnySwitchIn() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
    // This just calls onStart for the ability holder
    let ability_holder_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    on_start(battle, ability_holder_pos)
}

/// onSetStatus(status, target, source, effect) {
///     if (!['psn', 'tox'].includes(status.id)) return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Pastel Veil');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (!['psn', 'tox'].includes(status.id)) return;
    if status_id != "psn" && status_id != "tox" {
        return EventResult::Continue;
    }

    // if ((effect as Move)?.status)
    let is_move_effect = if let Some(effect_id_str) = effect_id {
        if let Some(move_data) = battle.dex.moves().get_by_id(&ID::from(effect_id_str)) {
            move_data.status.is_some()
        } else {
            false
        }
    } else {
        false
    };

    if is_move_effect {
        // this.add('-immune', target, '[from] ability: Pastel Veil');
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };

        battle.add("-immune", &[
            Arg::String(target_slot),
            Arg::Str("[from] ability: Pastel Veil"),
        ]);
    }

    // return false;
    EventResult::Boolean(false)
}

/// onAllySetStatus(status, target, source, effect) {
///     if (!['psn', 'tox'].includes(status.id)) return;
///     if ((effect as Move)?.status) {
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Pastel Veil', `[of] ${effectHolder}`);
///     }
///     return false;
/// }
pub fn on_ally_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (!['psn', 'tox'].includes(status.id)) return;
    if status_id != "psn" && status_id != "tox" {
        return EventResult::Continue;
    }

    // if ((effect as Move)?.status)
    let is_move_effect = if let Some(effect_id_str) = effect_id {
        if let Some(move_data) = battle.dex.moves().get_by_id(&ID::from(effect_id_str)) {
            move_data.status.is_some()
        } else {
            false
        }
    } else {
        false
    };

    if is_move_effect {
        // const effectHolder = this.effectState.target;
        let effect_holder_pos = match battle.effect_state.target {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let (target_slot, effect_holder_slot) = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let effect_holder = match battle.pokemon_at(effect_holder_pos.0, effect_holder_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target.get_slot(), effect_holder.get_slot())
        };

        // this.add('-block', target, 'ability: Pastel Veil', `[of] ${effectHolder}`);
        battle.add("-block", &[
            Arg::String(target_slot),
            Arg::Str("ability: Pastel Veil"),
            Arg::String(format!("[of] {}", effect_holder_slot)),
        ]);
    }

    // return false;
    EventResult::Boolean(false)
}

