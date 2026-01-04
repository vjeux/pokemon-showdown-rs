//! Steel Beam Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterMove(pokemon, target, move) {
///     if (move.mindBlownRecoil && !move.multihit) {
///         const hpBeforeRecoil = pokemon.hp;
///         this.damage(Math.round(pokemon.maxhp / 2), pokemon, pokemon, this.dex.conditions.get('Steel Beam'), true);
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

    // Get the current move data
    let (mind_blown_recoil, is_multihit) = {
        let active_move = match &battle.active_move {
            Some(active_move) => &active_move.id,
            None => return EventResult::Continue,
        };
        let move_data = battle.dex.moves().get_by_id(active_move);
        if let Some(m) = move_data {
            let mind_blown_recoil = m.flags.contains_key("mindBlownRecoil");
            let is_multihit = m.multihit.is_some();
            (mind_blown_recoil, is_multihit)
        } else {
            return EventResult::Continue;
        }
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

        // this.damage(Math.round(pokemon.maxhp / 2), pokemon, pokemon, this.dex.conditions.get('Steel Beam'), true);
        let damage_amount = (max_hp as f64 / 2.0).round() as i32;
        battle.damage(
            damage_amount,
            Some(pokemon),
            Some(pokemon),
            Some(&ID::from("steelbeam")),
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
            battle.run_event("EmergencyExit", Some(pokemon), Some(pokemon), None, EventResult::Continue, false, false);
        }
    }

    EventResult::Continue
}
