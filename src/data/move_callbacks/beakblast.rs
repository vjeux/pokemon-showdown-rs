//! Beak Blast Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// priorityChargeCallback(pokemon) {
///     pokemon.addVolatile('beakblast');
/// }
pub fn priority_charge_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.addVolatile('beakblast');
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("beakblast"), None, None, None, None);

    EventResult::Continue
}

/// onAfterMove(pokemon) {
///     pokemon.removeVolatile('beakblast');
/// }
pub fn on_after_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // pokemon.removeVolatile('beakblast');
    Pokemon::remove_volatile(battle, source_pos, &ID::from("beakblast"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'move: Beak Blast');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // this.add('-singleturn', pokemon, 'move: Beak Blast');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[pokemon_ident.as_str().into(), "move: Beak Blast".into()],
        );

        EventResult::Continue
    }

    /// onHit(target, source, move) {
    ///     if (this.checkMoveMakesContact(move, source, target)) {
    ///         source.trySetStatus('brn', target);
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // Get source from target_pos (in condition context, pokemon_pos is the pokemon with beakblast, target is the attacker)
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get the active move
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        };

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('brn', target);
        // }
        if battle.check_move_makes_contact(&move_id, source, pokemon_pos, false) {
            Pokemon::try_set_status(battle, source, ID::from("brn"), None);
        }

        EventResult::Continue
    }
}
