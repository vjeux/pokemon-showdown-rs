//! Tailwind Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// durationCallback(target, source, effect) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Tailwind');
    ///         return 6;
    ///     }
    ///     return 4;
    /// }
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // if (source?.hasAbility('persistent')) {
        if let Some(source) = source_pos {
            let has_persistent = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(4),
                };
                source_pokemon.has_ability(&["persistent"])
            };

            if has_persistent {
                // this.add('-activate', source, 'ability: Persistent', '[move] Tailwind');
                let source_arg = {
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Number(4),
                    };
                    source_pokemon.get_slot()
                };

                battle.add(
                    "-activate",
                    &[
                        source_arg.into(),
                        "ability: Persistent".into(),
                        "[move] Tailwind".into(),
                    ],
                );

                // return 6;
                return EventResult::Number(6);
            }
        }

        // return 4;
        EventResult::Number(4)
    }

    /// onSideStart(side, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-sidestart', side, 'move: Tailwind', '[persistent]');
    ///     } else {
    ///         this.add('-sidestart', side, 'move: Tailwind');
    ///     }
    /// }
    pub fn on_side_start(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);

            // if (source?.hasAbility('persistent')) {
            if let Some(source) = source_pos {
                let has_persistent = {
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => {
                            battle.add("-sidestart", &[side_arg, "move: Tailwind".into()]);
                            return EventResult::Continue;
                        }
                    };
                    source_pokemon.has_ability(&["persistent"])
                };

                if has_persistent {
                    // this.add('-sidestart', side, 'move: Tailwind', '[persistent]');
                    battle.add(
                        "-sidestart",
                        &[
                            side_arg,
                            "move: Tailwind".into(),
                            "[persistent]".into(),
                        ],
                    );
                } else {
                    // this.add('-sidestart', side, 'move: Tailwind');
                    battle.add("-sidestart", &[side_arg, "move: Tailwind".into()]);
                }
            } else {
                // this.add('-sidestart', side, 'move: Tailwind');
                battle.add("-sidestart", &[side_arg, "move: Tailwind".into()]);
            }
        }

        EventResult::Continue
    }

    /// onModifySpe(spe, pokemon) {
    ///     return this.chainModify(2);
    /// }
    pub fn on_modify_spe(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // return this.chainModify(2);
        EventResult::Number(battle.chain_modify(2.0))
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'move: Tailwind');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'move: Tailwind');
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "move: Tailwind".into()]);
        }

        EventResult::Continue
    }
}
