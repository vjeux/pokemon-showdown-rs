//! Dragon Cheer Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (target.volatiles['focusenergy']) return false;
    ///     if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
    ///         this.add('-start', target, 'move: Dragon Cheer', '[silent]');
    ///     } else {
    ///         this.add('-start', target, 'move: Dragon Cheer');
    ///     }
    ///     // Store at the start because the boost doesn't change if a Pokemon
    ///     // Terastallizes into Dragon while having this volatile
    ///     // Found by DarkFE:
    ///     // https://www.smogon.com/forums/threads/scarlet-violet-battle-mechanics-research.3709545/post-9894139
    ///     this.effectState.hasDragonType = target.hasType("Dragon");
    /// }
    pub fn on_start(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.volatiles['focusenergy']) return false;
        let has_focus_energy = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon
                .volatiles
                .contains_key(&ID::from("focusenergy"))
        };

        if has_focus_energy {
            // return false;
            return EventResult::Boolean(false);
        }

        // if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
        //     this.add('-start', target, 'move: Dragon Cheer', '[silent]');
        // } else {
        //     this.add('-start', target, 'move: Dragon Cheer');
        // }
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        if let Some(effect) = effect_id {
            if effect == "costar"
                || effect == "imposter"
                || effect == "psychup"
                || effect == "transform"
            {
                // this.add('-start', target, 'move: Dragon Cheer', '[silent]');
                battle.add(
                    "-start",
                    &[
                        target_ident.as_str().into(),
                        "move: Dragon Cheer".into(),
                        "[silent]".into(),
                    ],
                );
            } else {
                // this.add('-start', target, 'move: Dragon Cheer');
                battle.add(
                    "-start",
                    &[target_ident.as_str().into(), "move: Dragon Cheer".into()],
                );
            }
        } else {
            // this.add('-start', target, 'move: Dragon Cheer');
            battle.add(
                "-start",
                &[target_ident.as_str().into(), "move: Dragon Cheer".into()],
            );
        }

        // this.effectState.hasDragonType = target.hasType("Dragon");
        let has_dragon_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_type(battle, "dragon")
        };

        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "hasDragonType".to_string(),
                serde_json::Value::Bool(has_dragon_type),
            );
        }

        EventResult::Continue
    }

    /// onModifyCritRatio(critRatio, source) {
    ///     return critRatio + (this.effectState.hasDragonType ? 2 : 1);
    /// }
    pub fn on_modify_crit_ratio(
        battle: &mut Battle,
        crit_ratio: i32,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // return critRatio + (this.effectState.hasDragonType ? 2 : 1);
        let has_dragon_type = if let Some(ref effect_state) = battle.current_effect_state {
            effect_state
                .data
                .get("hasDragonType")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        } else {
            false
        };

        let boost = if has_dragon_type { 2 } else { 1 };

        EventResult::Number(crit_ratio + boost)
    }
}
