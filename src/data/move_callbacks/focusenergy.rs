//! Focus Energy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (target.volatiles['dragoncheer']) return false;
    ///     if (effect?.id === 'zpower') {
    ///         this.add('-start', target, 'move: Focus Energy', '[zeffect]');
    ///     } else if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
    ///         this.add('-start', target, 'move: Focus Energy', '[silent]');
    ///     } else {
    ///         this.add('-start', target, 'move: Focus Energy');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.volatiles['dragoncheer']) return false;
        let has_dragoncheer = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.volatiles.contains_key(&ID::from("dragoncheer"))
        };

        if has_dragoncheer {
            return EventResult::Boolean(false);
        }

        // if (effect?.id === 'zpower') {
        //     this.add('-start', target, 'move: Focus Energy', '[zeffect]');
        // } else if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
        //     this.add('-start', target, 'move: Focus Energy', '[silent]');
        // } else {
        //     this.add('-start', target, 'move: Focus Energy');
        // }
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        if let Some(effect) = effect_id {
            if effect == "zpower" {
                battle.add("-start", &[target_ident.as_str().into(), "move: Focus Energy".into(), "[zeffect]".into()]);
            } else if effect == "costar" || effect == "imposter" || effect == "psychup" || effect == "transform" {
                battle.add("-start", &[target_ident.as_str().into(), "move: Focus Energy".into(), "[silent]".into()]);
            } else {
                battle.add("-start", &[target_ident.as_str().into(), "move: Focus Energy".into()]);
            }
        } else {
            battle.add("-start", &[target_ident.as_str().into(), "move: Focus Energy".into()]);
        }

        EventResult::Continue
    }

    /// onModifyCritRatio(critRatio) {
    ///     return critRatio + 2;
    /// }
    pub fn on_modify_crit_ratio(_battle: &mut Battle, crit_ratio: i32) -> EventResult {
        // return critRatio + 2;
        EventResult::Number(crit_ratio + 2)
    }
}
