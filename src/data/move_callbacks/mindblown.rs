//! Mind Blown Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onAfterMove(pokemon, target, move) {
///     if (move.mindBlownRecoil && !move.multihit) {
///         const hpBeforeRecoil = pokemon.hp;
///         this.damage(Math.round(pokemon.maxhp / 2), pokemon, pokemon, this.dex.conditions.get('Mind Blown'), true);
///         if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2) {
///             this.runEvent('EmergencyExit', pokemon, pokemon);
///         }
///     }
/// }
pub fn on_after_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = source_pos;

    // Get the current move data from active_move (NOT dex - the flag may have been cleared)
    let (mind_blown_recoil, is_multihit) = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        // JS: if (move.mindBlownRecoil && !move.multihit)
        // In JS, 'move' is the active move, which may have mindBlownRecoil set to false
        // if the hit loop already applied recoil.
        (active_move.mindblown_recoil, active_move.multi_hit.is_some())
    };

    // if (move.mindBlownRecoil && !move.multihit) {
    if mind_blown_recoil && !is_multihit {
        // const hpBeforeRecoil = pokemon.hp;
        let (hp_before_recoil, max_hp) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (pokemon_pokemon.hp, pokemon_pokemon.maxhp)
        };

        // this.damage(Math.round(pokemon.maxhp / 2), pokemon, pokemon, this.dex.conditions.get('Mind Blown'), true);
        let damage_amount = (max_hp as f64 / 2.0).round() as i32;
        battle.damage(
            damage_amount,
            Some(pokemon),
            Some(pokemon),
            Some(&Effect::move_(ID::from("mindblown"))),
            true, // ignoreability
        );

        // if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2) {
        let hp_after_recoil = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.hp
        };

        let half_hp = max_hp / 2;
        if hp_after_recoil <= half_hp && hp_before_recoil > half_hp {
            // this.runEvent('EmergencyExit', pokemon, pokemon);
            battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(pokemon)), Some(pokemon), None, EventResult::Continue, false, false);
        }
    }

    EventResult::Continue
}
