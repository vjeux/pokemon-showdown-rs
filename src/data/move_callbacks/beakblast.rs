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

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('brn', target);
        // }
        // Note: In JavaScript, checkMoveMakesContact takes the active move object (not move_id)
        // so we need to check the active move's flags directly
        // This is important for moves like Shell Side Arm that dynamically set contact in onModifyMove
        // Note: pokemon_pos (the Beak Blast user) is the source of the burn status
        // This is important for Synchronize to know who to pass the status back to

        // Check if the active move has contact flag set (using two-phase borrow)
        let has_contact = battle.active_move.as_ref().map(|m| m.borrow().flags.contact).unwrap_or(false);

        // Check for Protective Pads on the attacker
        let has_protective_pads = {
            battle.pokemon_at(source.0, source.1)
                .map(|p| p.item.as_str() == "protectivepads")
                .unwrap_or(false)
        };

        let makes_contact = has_contact && !has_protective_pads;

        if makes_contact {
            Pokemon::try_set_status(battle, source, ID::from("brn"), Some(pokemon_pos), None);
        }

        EventResult::Continue
    }
}
