//! Magic Coat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     this.add('-singleturn', target, 'move: Magic Coat');
    ///     if (effect?.effectType === 'Move') {
    ///         this.effectState.pranksterBoosted = effect.pranksterBoosted;
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'move: Magic Coat');
        let target_arg = {

            let pokemon = match battle.pokemon_at(target.0, target.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            pokemon.get_slot()

        };
        battle.add("-singleturn", &[target_arg.into(), "move: Magic Coat".into()]);

        // if (effect?.effectType === 'Move') {
        //     this.effectState.pranksterBoosted = effect.pranksterBoosted;
        // }
        if let Some(eff_id) = effect_id {
            let effect = battle.dex.get_move_by_id(&crate::dex_data::ID::from(eff_id));
            if effect.is_some() {
                // It's a move
                if let Some(ref move_data) = battle.active_move {
                    if let Some(ref mut effect_state) = battle.current_effect_state {
                        effect_state.prankster_boosted = move_data.prankster_boosted;
                    }
                }
            }
        }

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
    ///         return;
    ///     }
    ///     const newMove = this.dex.getActiveMove(move.id);
    ///     newMove.hasBounced = true;
    ///     newMove.pranksterBoosted = this.effectState.pranksterBoosted;
    ///     this.actions.useMove(newMove, target, { target: source });
    ///     return null;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        let target = target_pos;
        let source = source_pos;

        // if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
        //     return;
        // }

        // target === source
        if target == source {
            return EventResult::Continue;
        }

        // move.hasBounced
        let has_bounced = battle.active_move.as_ref().map(|m| m.has_bounced).unwrap_or(false);
        if has_bounced {
            return EventResult::Continue;
        }

        // !move.flags['reflectable']
        let is_reflectable = battle.active_move.as_ref().map(|m| m.flags.reflectable).unwrap_or(false);
        if !is_reflectable {
            return EventResult::Continue;
        }

        // target.isSemiInvulnerable()
        let is_semi_invulnerable = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.is_semi_invulnerable()
        };
        if is_semi_invulnerable {
            return EventResult::Continue;
        }

        // const newMove = this.dex.getActiveMove(move.id);
        // newMove.hasBounced = true;
        // newMove.pranksterBoosted = this.effectState.pranksterBoosted;
        let (move_id, prankster_boosted) = {
            let move_id = match &battle.active_move {
                Some(active_move) => active_move.id.clone(),
                None => return EventResult::Continue,
            };
            let prankster_boosted = match &battle.current_effect_state {
                Some(es) => es.prankster_boosted,
                None => false,
            };
            (move_id, prankster_boosted)
        };

        // this.actions.useMove(newMove, target, { target: source });
        // TODO: Implement use_move_with_bounced method in Battle
        // battle.use_move_with_bounced(&move_id, target, Some(source), true, prankster_boosted);

        // return null;
        EventResult::Stop
    }

    /// onAllyTryHitSide(target, source, move) {
    ///     if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
    ///         return;
    ///     }
    ///     const newMove = this.dex.getActiveMove(move.id);
    ///     newMove.hasBounced = true;
    ///     newMove.pranksterBoosted = false;
    ///     this.actions.useMove(newMove, this.effectState.target, { target: source });
    ///     move.hasBounced = true; // only bounce once in free-for-all battles
    ///     return null;
    /// }
    pub fn on_ally_try_hit_side(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
        //     return;
        // }

        // target.isAlly(source)
        let is_ally = battle.is_ally(target, source);
        if is_ally {
            return EventResult::Continue;
        }

        // move.hasBounced
        let has_bounced = battle.active_move.as_ref().map(|m| m.has_bounced).unwrap_or(false);
        if has_bounced {
            return EventResult::Continue;
        }

        // !move.flags['reflectable']
        let is_reflectable = battle.active_move.as_ref().map(|m| m.flags.reflectable).unwrap_or(false);
        if !is_reflectable {
            return EventResult::Continue;
        }

        // target.isSemiInvulnerable()
        let is_semi_invulnerable = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.is_semi_invulnerable()
        };
        if is_semi_invulnerable {
            return EventResult::Continue;
        }

        // const newMove = this.dex.getActiveMove(move.id);
        // newMove.hasBounced = true;
        // newMove.pranksterBoosted = false;
        // this.actions.useMove(newMove, this.effectState.target, { target: source });
        let effect_state_target = match &battle.current_effect_state {
            Some(es) => es.target,
            None => return EventResult::Continue,
        };

        // TODO: Implement use_move_with_bounced method in Battle
        // let move_id_id = battle.active_move.as_ref().map(|m| m.clone());
        // if let Some(move_id_id) = move_id_id {
        //     battle.use_move_with_bounced(&move_id_id, effect_state_target, Some(source), true, false);
        // }

        // move.hasBounced = true; // only bounce once in free-for-all battles
        if let Some(ref mut active_move) = battle.active_move {
            active_move.has_bounced = true;
        }

        // return null;
        EventResult::Stop
    }
}
