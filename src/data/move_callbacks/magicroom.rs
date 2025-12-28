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
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasAbility('persistent')) {
        let has_persistent = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Number(5),
            };
            source_pokemon.has_ability(&ID::from("persistent"))
        } else {
            false
        };

        if has_persistent {
            //     this.add('-activate', source, 'ability: Persistent', '[move] Magic Room');
            if let Some(source) = source_pos {
                let source_arg = crate::battle::Arg::Pos(source.0, source.1);
                battle.add("-activate", &[source_arg, "ability: Persistent".into(), "[move] Magic Room".into()]);
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
    pub fn on_field_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasAbility('persistent')) {
        //     this.add('-fieldstart', 'move: Magic Room', `[of] ${source}`, '[persistent]');
        // } else {
        //     this.add('-fieldstart', 'move: Magic Room', `[of] ${source}`);
        // }
        let has_persistent = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => false,
            };
            source_pokemon.has_ability(&ID::from("persistent"))
        } else {
            false
        };

        if let Some(source) = source_pos {
            let source_arg = crate::battle::Arg::Pos(source.0, source.1);
            if has_persistent {
                battle.add("-fieldstart", &["move: Magic Room".into(), format!("[of] {}", source_arg).into(), "[persistent]".into()]);
            } else {
                battle.add("-fieldstart", &["move: Magic Room".into(), format!("[of] {}", source_arg).into()]);
            }
        }

        // for (const mon of this.getAllActive()) {
        //     this.singleEvent('End', mon.getItem(), mon.itemState, mon);
        // }
        let all_active = battle.get_all_active();
        for pokemon_pos in all_active {
            let item_id = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.item.clone()
            };

            if let Some(item_id) = item_id {
                battle.run_single_event("End", Some(&item_id), pokemon_pos, pokemon_pos, None);
            }
        }

        EventResult::Continue
    }

    /// onFieldRestart(target, source) {
    ///     this.field.removePseudoWeather('magicroom');
    /// }
    pub fn on_field_restart(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
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
        let source = match &battle.effect_state {
            Some(es) => es.source,
            None => return EventResult::Continue,
        };

        let source_arg = crate::battle::Arg::Pos(source.0, source.1);
        battle.add("-fieldend", &["move: Magic Room".into(), format!("[of] {}", source_arg).into()]);

        EventResult::Continue
    }
}
