//! Magic Room Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Magic Room');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // if (source?.hasAbility('persistent')) {
        let has_persistent = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Number(5),
            };
            source_pokemon.has_ability(battle, &["persistent"])
        } else {
            false
        };

        if has_persistent {
            //     this.add('-activate', source, 'ability: Persistent', '[move] Magic Room');
            if let Some(source) = source_pos {
                let source_arg = {
                    let pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,

                        None => return EventResult::Continue,
                    };

                    pokemon.get_slot()
                };
                battle.add(
                    "-activate",
                    &[
                        source_arg.into(),
                        "ability: Persistent".into(),
                        "[move] Magic Room".into(),
                    ],
                );
            }
            //     return 7;
            return EventResult::Number(7);
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onFieldStart(target, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-fieldstart', 'move: Magic Room', `[of] ${source}`, '[persistent]');
    ///     } else {
    ///         this.add('-fieldstart', 'move: Magic Room', `[of] ${source}`);
    ///     }
    ///     for (const mon of this.getAllActive()) {
    ///         this.singleEvent('End', mon.getItem(), mon.itemState, mon);
    ///     }
    /// }
    pub fn on_field_start(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasAbility('persistent')) {
        //     this.add('-fieldstart', 'move: Magic Room', `[of] ${source}`, '[persistent]');
        // } else {
        //     this.add('-fieldstart', 'move: Magic Room', `[of] ${source}`);
        // }
        let has_persistent = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.has_ability(battle, &["persistent"])
        } else {
            false
        };

        if let Some(source) = source_pos {
            let source_arg = {
                let pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,

                    None => return EventResult::Continue,
                };

                pokemon.get_slot()
            };
            if has_persistent {
                battle.add(
                    "-fieldstart",
                    &[
                        "move: Magic Room".into(),
                        format!("[of] {}", source_arg).into(),
                        "[persistent]".into(),
                    ],
                );
            } else {
                battle.add(
                    "-fieldstart",
                    &[
                        "move: Magic Room".into(),
                        format!("[of] {}", source_arg).into(),
                    ],
                );
            }
        }

        // for (const mon of this.getAllActive()) {
        //     this.singleEvent('End', mon.getItem(), mon.itemState, mon);
        // }
        let all_active = battle.get_all_active(false);
        for pokemon_pos in all_active {
            let item_id = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.item.clone()
            };

            // Check if item is not empty
            if item_id != ID::from("") {
                battle.single_event("End", &crate::battle::Effect::item(item_id), None, Some(pokemon_pos), Some(pokemon_pos), None, None);
            }
        }

        EventResult::Continue
    }

    /// onFieldRestart(target, source) {
    ///     this.field.removePseudoWeather('magicroom');
    /// }
    pub fn on_field_restart(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // this.field.removePseudoWeather('magicroom');
        battle.remove_pseudo_weather(&ID::from("magicroom"));

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Magic Room', '[of] ' + this.effectState.source);
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Magic Room', '[of] ' + this.effectState.source);
        let source = battle.with_effect_state_ref(|state| state.source).flatten();

        if let Some(source_pos) = source {
            let source_arg = {
                let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p,

                    None => return EventResult::Continue,
                };

                pokemon.get_slot()
            };
            battle.add(
                "-fieldend",
                &[
                    "move: Magic Room".into(),
                    format!("[of] {}", source_arg).into(),
                ],
            );
        }

        EventResult::Continue
    }
}
