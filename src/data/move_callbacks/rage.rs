//! Rage Move
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
    ///     this.add('-singlemove', pokemon, 'Rage');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singlemove', pokemon, 'Rage');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-singlemove",
            &[pokemon_arg.into(), "Rage".into()],
        );

        EventResult::Continue
    }

    /// onHit(target, source, move) {
    ///     if (target !== source && move.category !== 'Status') {
    ///         this.boost({ atk: 1 });
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let pokemon = pokemon_pos; // "target" in JS - pokemon with Rage volatile
        let source = match target_pos {
            // "source" in JS - attacker
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target !== source && move.category !== 'Status')
        if pokemon != source {
            // Get the active move
            let move_id = match &battle.active_move {
                Some(active_move) => active_move.id.clone(),
                None => return EventResult::Continue,
            };

            let move_data = match battle.dex.moves().get_by_id(&move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };

            if move_data.category.as_str() != "Status" {
                // this.boost({ atk: 1 });
                battle.boost(&[("atk", 1)], pokemon, Some(pokemon), None, false, false);
            }
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon) {
    ///     this.debug('removing Rage before attack');
    ///     pokemon.removeVolatile('rage');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // this.debug('removing Rage before attack');
        // (debug call skipped)

        // pokemon.removeVolatile('rage');
        Pokemon::remove_volatile(battle, pokemon, &ID::from("rage"));

        EventResult::Continue
    }
}
