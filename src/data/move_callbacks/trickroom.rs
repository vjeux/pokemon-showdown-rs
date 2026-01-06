//! Trick Room Move
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
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Trick Room');
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
        // if (source?.hasAbility('persistent'))
        if let Some(source) = source_pos {
            let has_persistent = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(5),
                };
                source_pokemon.has_ability(battle, &["persistent"])
            };

            if has_persistent {
                // this.add('-activate', source, 'ability: Persistent', '[move] Trick Room');
                let source_slot = {
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Number(5),
                    };
                    source_pokemon.get_slot()
                };

                battle.add(
                    "-activate",
                    &[
                        crate::battle::Arg::from(source_slot),
                        crate::battle::Arg::from("ability: Persistent"),
                        crate::battle::Arg::from("[move] Trick Room"),
                    ],
                );

                // return 7;
                return EventResult::Number(7);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onFieldStart(target, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-fieldstart', 'move: Trick Room', `[of] ${source}`, '[persistent]');
    ///     } else {
    ///         this.add('-fieldstart', 'move: Trick Room', `[of] ${source}`);
    ///     }
    /// }
    pub fn on_field_start(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // if (source?.hasAbility('persistent'))
        if let Some(source) = source_pos {
            let (has_persistent, source_slot) = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => {
                        // No source, just add basic message
                        battle.add("-fieldstart", &[crate::battle::Arg::from("move: Trick Room")]);
                        return EventResult::Continue;
                    },
                };
                (source_pokemon.has_ability(battle, &["persistent"]), source_pokemon.get_slot())
            };

            if has_persistent {
                // this.add('-fieldstart', 'move: Trick Room', `[of] ${source}`, '[persistent]');
                battle.add(
                    "-fieldstart",
                    &[
                        crate::battle::Arg::from("move: Trick Room"),
                        crate::battle::Arg::from(format!("[of] {}", source_slot)),
                        crate::battle::Arg::from("[persistent]"),
                    ],
                );
            } else {
                // this.add('-fieldstart', 'move: Trick Room', `[of] ${source}`);
                battle.add(
                    "-fieldstart",
                    &[
                        crate::battle::Arg::from("move: Trick Room"),
                        crate::battle::Arg::from(format!("[of] {}", source_slot)),
                    ],
                );
            }
        } else {
            // No source, just add basic message
            battle.add("-fieldstart", &[crate::battle::Arg::from("move: Trick Room")]);
        }

        EventResult::Continue
    }

    /// onFieldRestart(target, source) {
    ///     this.field.removePseudoWeather('trickroom');
    /// }
    pub fn on_field_restart(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // this.field.removePseudoWeather('trickroom');
        use crate::dex_data::ID;
        battle.remove_pseudo_weather(&ID::from("trickroom"));

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Trick Room');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Trick Room');
        battle.add("-fieldend", &[crate::battle::Arg::from("move: Trick Room")]);

        EventResult::Continue
    }
}
