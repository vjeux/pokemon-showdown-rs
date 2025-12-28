//! Perish Song Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitField(target, source, move) {
///     let result = false;
///     let message = false;
///     for (const pokemon of this.getAllActive()) {
///         if (this.runEvent('Invulnerability', pokemon, source, move) === false) {
///             this.add('-miss', source, pokemon);
///             result = true;
///         } else if (this.runEvent('TryHit', pokemon, source, move) === null) {
///             result = true;
///         } else if (!pokemon.volatiles['perishsong']) {
///             pokemon.addVolatile('perishsong');
///             this.add('-start', pokemon, 'perish3', '[silent]');
///             result = true;
///             message = true;
///         }
///     }
///     if (!result) return false;
///     if (message) this.add('-fieldactivate', 'move: Perish Song');
/// }
pub fn on_hit_field(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // let result = false;
    let mut result = false;
    // let message = false;
    let mut message = false;

    // for (const pokemon of this.getAllActive()) {
    let all_active = battle.get_all_active();

    for pokemon_pos in all_active {
        // if (this.runEvent('Invulnerability', pokemon, source, move) === false) {
        let invulnerability_result = battle.run_event("Invulnerability", pokemon_pos, source, Some(&ID::from(move_id)));

        if matches!(invulnerability_result, EventResult::Boolean(false)) {
            // this.add('-miss', source, pokemon);
            let (source_arg, pokemon_arg) = {
                let source_arg = if let Some(src) = source {
                    let src_pokemon = match battle.pokemon_at(src.0, src.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    crate::battle::Arg::from(src_pokemon)
                } else {
                    continue
                };

                let pokemon_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let pokemon_arg = crate::battle::Arg::from(pokemon_pokemon);
                (source_arg, pokemon_arg)
            };

            battle.add("-miss", &[source_arg, pokemon_arg]);
            // result = true;
            result = true;
        // } else if (this.runEvent('TryHit', pokemon, source, move) === null) {
        } else {
            let try_hit_result = battle.run_event("TryHit", pokemon_pos, source, Some(&ID::from(move_id)));

            if matches!(try_hit_result, EventResult::Stop) {
                // result = true;
                result = true;
            // } else if (!pokemon.volatiles['perishsong']) {
            } else {
                let has_perishsong = {
                    let pokemon_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    pokemon_pokemon.volatiles.contains_key(&ID::from("perishsong"))
                };

                if !has_perishsong {
                    // pokemon.addVolatile('perishsong');
                    battle.add_volatile(&ID::from("perishsong"), pokemon_pos, source, None);

                    // this.add('-start', pokemon, 'perish3', '[silent]');
                    let pokemon_arg = {
                        let pokemon_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => continue,
                        };
                        crate::battle::Arg::from(pokemon_pokemon)
                    };

                    battle.add("-start", &[pokemon_arg, "perish3".into(), "[silent]".into()]);

                    // result = true;
                    result = true;
                    // message = true;
                    message = true;
                }
            }
        }
    }

    // if (!result) return false;
    if !result {
        return EventResult::Boolean(false);
    }

    // if (message) this.add('-fieldactivate', 'move: Perish Song');
    if message {
        battle.add("-fieldactivate", &["move: Perish Song".into()]);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onEnd(target) {
    ///     this.add('-start', target, 'perish0');
    ///     target.faint();
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-start', target, 'perish0');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-start", &[target_arg, "perish0".into()]);

        // target.faint();
        battle.faint(target, None, None);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     const duration = pokemon.volatiles['perishsong'].duration;
    ///     this.add('-start', pokemon, `perish${duration}`);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // const duration = pokemon.volatiles['perishsong'].duration;
        let duration = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.volatiles.get(&ID::from("perishsong"))
                .and_then(|v| v.duration)
                .unwrap_or(0)
        };

        // this.add('-start', pokemon, `perish${duration}`);
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };

        battle.add("-start", &[pokemon_arg, format!("perish{}", duration).into()]);

        EventResult::Continue
    }
}
