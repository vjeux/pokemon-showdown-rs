//! Wonder Room Move
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
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Wonder Room');
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
            let (has_persistent, source_slot) = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(5),
                };
                (source_pokemon.has_ability(battle, &["persistent"]), source_pokemon.get_slot())
            };

            if has_persistent {
                // this.add('-activate', source, 'ability: Persistent', '[move] Wonder Room');
                battle.add(
                    "-activate",
                    &[
                        crate::battle::Arg::from(source_slot),
                        crate::battle::Arg::from("ability: Persistent"),
                        crate::battle::Arg::from("[move] Wonder Room"),
                    ],
                );
                // return 7;
                return EventResult::Number(7);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onModifyMove(move, source, target) {
    ///     // This code is for moves that use defensive stats as the attacking stat; see below for most of the implementation
    ///     if (!move.overrideOffensiveStat) return;
    ///     const statAndBoosts = move.overrideOffensiveStat;
    ///     if (!['def', 'spd'].includes(statAndBoosts)) return;
    ///     move.overrideOffensiveStat = statAndBoosts === 'def' ? 'spd' : 'def';
    ///     this.hint(`${move.name} uses ${statAndBoosts === 'def' ? '' : 'Sp. '}Def boosts when Wonder Room is active.`);
    /// }
    pub fn on_modify_move(
        battle: &mut Battle,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // if (!move.overrideOffensiveStat) return;
        let stat_and_boosts = if let Some(ref mut active_move) = battle.active_move {
            if let Some(ref stat) = active_move.override_offensive_stat {
                stat.clone()
            } else {
                return EventResult::Continue;
            }
        } else {
            return EventResult::Continue;
        };

        // if (!['def', 'spd'].includes(statAndBoosts)) return;
        if stat_and_boosts != "def" && stat_and_boosts != "spd" {
            return EventResult::Continue;
        }

        // move.overrideOffensiveStat = statAndBoosts === 'def' ? 'spd' : 'def';
        let new_stat = if stat_and_boosts == "def" {
            "spd".to_string()
        } else {
            "def".to_string()
        };

        let move_name = if let Some(ref mut active_move) = battle.active_move {
            active_move.override_offensive_stat = Some(new_stat.clone());
            active_move.name.clone()
        } else {
            return EventResult::Continue;
        };

        // this.hint(`${move.name} uses ${statAndBoosts === 'def' ? '' : 'Sp. '}Def boosts when Wonder Room is active.`);
        let hint_message = format!(
            "{} uses {}Def boosts when Wonder Room is active.",
            move_name,
            if stat_and_boosts == "def" { "" } else { "Sp. " }
        );

        battle.hint(&hint_message, true, None);

        EventResult::Continue
    }

    /// onFieldStart(field, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-fieldstart', 'move: Wonder Room', `[of] ${source}`, '[persistent]');
    ///     } else {
    ///         this.add('-fieldstart', 'move: Wonder Room', `[of] ${source}`);
    ///     }
    /// }
    pub fn on_field_start(
        battle: &mut Battle,
        _field_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // if (source?.hasAbility('persistent'))
        if let Some(source) = source_pos {
            let (has_persistent, source_slot) = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => {
                        // No source, just add basic message
                        battle.add("-fieldstart", &[crate::battle::Arg::from("move: Wonder Room")]);
                        return EventResult::Continue;
                    },
                };
                (source_pokemon.has_ability(battle, &["persistent"]), source_pokemon.get_slot())
            };

            if has_persistent {
                // this.add('-fieldstart', 'move: Wonder Room', `[of] ${source}`, '[persistent]');
                battle.add(
                    "-fieldstart",
                    &[
                        crate::battle::Arg::from("move: Wonder Room"),
                        crate::battle::Arg::from(format!("[of] {}", source_slot)),
                        crate::battle::Arg::from("[persistent]"),
                    ],
                );
            } else {
                // this.add('-fieldstart', 'move: Wonder Room', `[of] ${source}`);
                battle.add(
                    "-fieldstart",
                    &[
                        crate::battle::Arg::from("move: Wonder Room"),
                        crate::battle::Arg::from(format!("[of] {}", source_slot)),
                    ],
                );
            }
        } else {
            // No source, just add basic message
            battle.add("-fieldstart", &[crate::battle::Arg::from("move: Wonder Room")]);
        }

        EventResult::Continue
    }

    /// onFieldRestart(target, source) {
    ///     this.field.removePseudoWeather('wonderroom');
    /// }
    pub fn on_field_restart(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // this.field.removePseudoWeather('wonderroom');
        use crate::dex_data::ID;
        battle.remove_pseudo_weather(&ID::from("wonderroom"));

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Wonder Room');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Wonder Room');
        battle.add("-fieldend", &[crate::battle::Arg::from("move: Wonder Room")]);

        EventResult::Continue
    }
}
