//! G-Max Chi Strike Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     this.effectState.layers = 1;
    ///     if (!['costar', 'imposter', 'psychup', 'transform'].includes(effect?.id)) {
    ///         this.add('-start', target, 'move: G-Max Chi Strike');
    ///     }
    /// }
    pub fn on_start(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // this.effectState.layers = 1;
        if let Some(effect_state) = &mut battle.current_effect_state {
            effect_state
                .data
                .insert("layers".to_string(), serde_json::Value::Number(1.into()));
        }

        // if (!['costar', 'imposter', 'psychup', 'transform'].includes(effect?.id)) {
        let should_show_message = match effect_id {
            Some(id) => !matches!(id, "costar" | "imposter" | "psychup" | "transform"),
            None => true,
        };

        if should_show_message {
            // this.add('-start', target, 'move: G-Max Chi Strike');
            if let Some(target) = target_pos {
                let target_ident = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target_pokemon.get_slot()
                };
                battle.add(
                    "-start",
                    &[
                        target_ident.as_str().into(),
                        "move: G-Max Chi Strike".into(),
                    ],
                );
            }
        }

        EventResult::Continue
    }

    /// onRestart(target, source, effect) {
    ///     if (this.effectState.layers >= 3) return false;
    ///     this.effectState.layers++;
    ///     if (!['costar', 'imposter', 'psychup', 'transform'].includes(effect?.id)) {
    ///         this.add('-start', target, 'move: G-Max Chi Strike');
    ///     }
    /// }
    pub fn on_restart(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // if (this.effectState.layers >= 3) return false;
        let layers = battle
            .current_effect_state
            .as_ref()
            .and_then(|es| es.data.get("layers"))
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        if layers >= 3 {
            return EventResult::Boolean(false);
        }

        // this.effectState.layers++;
        let new_layers = layers + 1;
        if let Some(effect_state) = &mut battle.current_effect_state {
            effect_state.data.insert(
                "layers".to_string(),
                serde_json::Value::Number(new_layers.into()),
            );
        }

        // if (!['costar', 'imposter', 'psychup', 'transform'].includes(effect?.id)) {
        let should_show_message = match effect_id {
            Some(id) => !matches!(id, "costar" | "imposter" | "psychup" | "transform"),
            None => true,
        };

        if should_show_message {
            // this.add('-start', target, 'move: G-Max Chi Strike');
            if let Some(target) = target_pos {
                let target_ident = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target_pokemon.get_slot()
                };
                battle.add(
                    "-start",
                    &[
                        target_ident.as_str().into(),
                        "move: G-Max Chi Strike".into(),
                    ],
                );
            }
        }

        EventResult::Continue
    }

    /// onModifyCritRatio(critRatio) {
    ///     return critRatio + this.effectState.layers;
    /// }
    pub fn on_modify_crit_ratio(battle: &mut Battle) -> EventResult {
        // return critRatio + this.effectState.layers;
        let layers = battle
            .current_effect_state
            .as_ref()
            .and_then(|es| es.data.get("layers"))
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        EventResult::Number(layers)
    }
}

/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(source) {
    ///                 for (const pokemon of source.alliesAndSelf()) {
    ///                   pokemon.addVolatile("gmaxchistrike");
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        use crate::dex_data::ID;
        use crate::pokemon::Pokemon;

        // for (const pokemon of source.alliesAndSelf()) {
        //     pokemon.addVolatile("gmaxchistrike");
        // }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let ally_positions = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.allies_and_self(battle, false)
        };

        for ally_pos in ally_positions {
            Pokemon::add_volatile(battle, ally_pos, ID::from("gmaxchistrike"), Some(source), None, None, None);
        }

        EventResult::Continue
    }
}
