//! Snatch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'Snatch');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // onStart(pokemon) {
        //     this.add('-singleturn', pokemon, 'Snatch');
        // }
        let pokemon = pokemon_pos;

        // this.add('-singleturn', pokemon, 'Snatch');
        let pokemon_arg = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.get_slot()
        };

        battle.add("-singleturn", &[pokemon_arg.into(), "Snatch".into()]);

        EventResult::Continue
    }

    /// onAnyPrepareHit(source, target, move) {
    ///     const snatchUser = this.effectState.source;
    ///     if (snatchUser.isSkyDropped()) return;
    ///     if (!move || move.isZ || move.isMax || !move.flags['snatch'] || move.sourceEffect === 'snatch') {
    ///         return;
    ///     }
    ///     snatchUser.removeVolatile('snatch');
    ///     this.add('-activate', snatchUser, 'move: Snatch', `[of] ${source}`);
    ///     this.actions.useMove(move.id, snatchUser);
    ///     return null;
    /// }
    pub fn on_any_prepare_hit(
        battle: &mut Battle,
        source_pos: Option<(usize, usize)>,
        _target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        use crate::dex_data::ID;

        // onAnyPrepareHit(source, target, move) {
        //     const snatchUser = this.effectState.source;
        //     if (snatchUser.isSkyDropped()) return;
        //     if (!move || move.isZ || move.isMax || !move.flags['snatch'] || move.sourceEffect === 'snatch') {
        //         return;
        //     }
        //     snatchUser.removeVolatile('snatch');
        //     this.add('-activate', snatchUser, 'move: Snatch', `[of] ${source}`);
        //     this.actions.useMove(move.id, snatchUser);
        //     return null;
        // }
        let source = source_pos;

        // const snatchUser = this.effectState.source;
        let snatch_user = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            match effect_state.source {
                Some(s) => s,
                None => return EventResult::Continue,
            }
        };

        // if (snatchUser.isSkyDropped()) return;
        let is_skydropped = Pokemon::is_sky_dropped(battle, snatch_user);
        if is_skydropped {
            return EventResult::Continue;
        }

        // if (!move || move.isZ || move.isMax || !move.flags['snatch'] || move.sourceEffect === 'snatch') {
        //     return;
        // }
        let (is_z, is_max, has_snatch_flag, source_effect) = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            (
                active_move.is_z,
                active_move.is_max,
                active_move.flags.snatch,
                active_move.source_effect.clone(),
            )
        };

        if is_z || is_max || !has_snatch_flag || source_effect == Some(ID::from("snatch")) {
            return EventResult::Continue;
        }

        // snatchUser.removeVolatile('snatch');
        {
            Pokemon::remove_volatile(battle, snatch_user, &ID::from("snatch"));
        }

        // this.add('-activate', snatchUser, 'move: Snatch', `[of] ${source}`);
        let snatch_user_arg = {
            let snatch_user_pokemon = match battle.pokemon_at(snatch_user.0, snatch_user.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            snatch_user_pokemon.get_slot()
        };

        if let Some(src) = source {
            let source_arg = {
                let source_pokemon = match battle.pokemon_at(src.0, src.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.get_slot()
            };

            battle.add(
                "-activate",
                &[
                    snatch_user_arg.into(),
                    "move: Snatch".into(),
                    format!("[of] {}", source_arg).into(),
                ],
            );
        } else {
            battle.add(
                "-activate",
                &[snatch_user_arg.into(), "move: Snatch".into()],
            );
        }

        // this.actions.useMove(move.id, snatchUser);
        let _move_id = ID::from(move_id);
        battle.use_move(&_move_id, snatch_user, None, None, None, None);

        // return null;
        EventResult::Stop
    }
}
