//! Magic Coat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     this.add('-singleturn', target, 'move: Magic Coat');
    ///     if (effect?.effectType === 'Move') {
    ///         this.effectState.pranksterBoosted = effect.pranksterBoosted;
    ///     }
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let effect_id = _effect.map(|e| e.id.as_str());
        let target = pokemon_pos;

        // this.add('-singleturn', target, 'move: Magic Coat');
        let target_arg = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add(
            "-singleturn",
            &[target_arg.into(), "move: Magic Coat".into()],
        );

        // if (effect?.effectType === 'Move') {
        //     this.effectState.pranksterBoosted = effect.pranksterBoosted;
        // }
        if let Some(eff_id) = effect_id {
            let effect = battle
                .dex
                .moves().get_by_id(&crate::dex_data::ID::from(eff_id));
            if effect.is_some() {
                // It's a move
                if let Some(ref move_data) = battle.active_move {
                    let prankster_boosted = move_data.prankster_boosted;
                    battle.with_effect_state(|state| {
                        state.prankster_boosted = prankster_boosted;
                    });
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
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        _move_id: Option<&str>,
    ) -> EventResult {
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
        let has_bounced = battle
            .active_move
            .as_ref()
            .map(|m| m.has_bounced)
            .unwrap_or(false);
        if has_bounced {
            return EventResult::Continue;
        }

        // !move.flags['reflectable']
        let is_reflectable = battle
            .active_move
            .as_ref()
            .map(|m| m.flags.reflectable)
            .unwrap_or(false);
        if !is_reflectable {
            return EventResult::Continue;
        }

        // target.isSemiInvulnerable()
        let is_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, target);
        if is_semi_invulnerable {
            return EventResult::Continue;
        }

        // const newMove = this.dex.getActiveMove(move.id);
        // newMove.hasBounced = true;
        // newMove.pranksterBoosted = this.effectState.pranksterBoosted;
        let (move_data, _prankster_boosted) = {
            let move_id = match &battle.active_move {
                Some(active_move) => active_move.id.clone(),
                None => return EventResult::Continue,
            };
            let prankster_boosted = battle.with_effect_state_ref(|state| state.prankster_boosted).unwrap_or(false);
            let move_data = match battle.dex.moves().get(move_id.as_str()).cloned() {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            (move_data, prankster_boosted)
        };

        // this.actions.useMove(newMove, target, { target: source });
        battle.use_move_with_bounced(&move_data, target, Some(source), true, _prankster_boosted);

        // return null;
        // EventResult::Null tells the TryHit handler to treat this as "miss" - prevent the original move from hitting
        EventResult::Null
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
    pub fn on_ally_try_hit_side(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get active_move from parameter
        let active_move_ref = match active_move {
            Some(m) => m,
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
        if active_move_ref.has_bounced {
            return EventResult::Continue;
        }

        // !move.flags['reflectable']
        if !active_move_ref.flags.reflectable {
            return EventResult::Continue;
        }

        // target.isSemiInvulnerable()
        let is_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, target);
        if is_semi_invulnerable {
            return EventResult::Continue;
        }

        // const newMove = this.dex.getActiveMove(move.id);
        // newMove.hasBounced = true;
        // newMove.pranksterBoosted = false;
        // this.actions.useMove(newMove, this.effectState.target, { target: source });
        let (move_data, _effect_state_target) = {
            let effect_state_target = match battle.with_effect_state_ref(|state| state.target).flatten() {
                Some(t) => t,
                None => return EventResult::Continue,
            };
            let move_data = match battle.dex.moves().get(active_move_ref.id.as_str()).cloned() {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            (move_data, effect_state_target)
        };

        battle.use_move_with_bounced(&move_data, _effect_state_target, Some(source), true, false);

        // move.hasBounced = true; // only bounce once in free-for-all battles
        if let Some(ref mut active_move_mut) = battle.active_move {
            active_move_mut.has_bounced = true;
        }

        // return null;
        // EventResult::Null tells the TryHit handler to treat this as "miss" - prevent the original move from hitting
        EventResult::Null
    }
}
