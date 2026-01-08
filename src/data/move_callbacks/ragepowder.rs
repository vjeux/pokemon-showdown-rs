//! Rage Powder Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// ```ignore
/// onTry(source) {
///     return this.activePerHalf > 1;
/// }
/// ```
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // return this.activePerHalf > 1;
    if battle.active_per_half > 1 {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'move: Rage Powder');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singleturn', pokemon, 'move: Rage Powder');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-singleturn",
            &[pokemon_arg.into(), "move: Rage Powder".into()],
        );

        EventResult::Continue
    }

    /// ```ignore
    /// onFoeRedirectTarget(target, source, source2, move) {
    ///     const ragePowderUser = this.effectState.target;
    ///     if (ragePowderUser.isSkyDropped()) return;
    ///
    ///     if (source.runStatusImmunity('powder') && this.validTarget(ragePowderUser, source, move.target)) {
    ///         if (move.smartTarget) move.smartTarget = false;
    ///         this.debug("Rage Powder redirected target of move");
    ///         return ragePowderUser;
    ///     }
    /// }
    /// ```
    pub fn on_foe_redirect_target(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
        // const ragePowderUser = this.effectState.target;
        let rage_powder_user = match battle.with_effect_state_ref(|state| state.target).flatten() {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (ragePowderUser.isSkyDropped()) return;
        let is_sky_dropped = Pokemon::is_sky_dropped(battle, rage_powder_user);

        if is_sky_dropped {
            return EventResult::Continue;
        }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (source.runStatusImmunity('powder') && this.validTarget(ragePowderUser, source, move.target))
        let has_powder_immunity = Pokemon::run_status_immunity(battle, source, "powder", false);

        if !has_powder_immunity {
            return EventResult::Continue;
        }

        // Get move target type
        let move_target = match battle
            .dex
            .moves().get_by_id(&crate::dex_data::ID::from(move_id))
        {
            Some(move_data) => move_data.target.clone(),
            None => return EventResult::Continue,
        };

        let is_valid_target = battle.valid_target(rage_powder_user, source, move_target.as_str());

        if is_valid_target {
            // if (move.smartTarget) move.smartTarget = false;
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }

            // this.debug("Rage Powder redirected target of move");
            battle.debug("Rage Powder redirected target of move");

            // return ragePowderUser;
            return EventResult::Position(rage_powder_user);
        }

        EventResult::Continue
    }
}
