//! Sparkling Aria Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMove(source, target, move) {
///     if (source.fainted || !move.hitTargets || move.hasSheerForce) {
///         // make sure the volatiles are cleared
///         for (const pokemon of this.getAllActive()) delete pokemon.volatiles['sparklingaria'];
///         return;
///     }
///     const numberTargets = move.hitTargets.length;
///     for (const pokemon of move.hitTargets) {
///         // bypasses Shield Dust when hitting multiple targets
///         if (pokemon !== source && pokemon.isActive && (pokemon.removeVolatile('sparklingaria') || numberTargets > 1) &&
///             pokemon.status === 'brn') {
///             pokemon.cureStatus();
///         }
///     }
/// }
pub fn on_after_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onAfterMove(source, target, move) {
    //     if (source.fainted || !move.hitTargets || move.hasSheerForce) {
    //         // make sure the volatiles are cleared
    //         for (const pokemon of this.getAllActive()) delete pokemon.volatiles['sparklingaria'];
    //         return;
    //     }
    //     const numberTargets = move.hitTargets.length;
    //     for (const pokemon of move.hitTargets) {
    //         // bypasses Shield Dust when hitting multiple targets
    //         if (pokemon !== source && pokemon.isActive && (pokemon.removeVolatile('sparklingaria') || numberTargets > 1) &&
    //             pokemon.status === 'brn') {
    //             pokemon.cureStatus();
    //         }
    //     }
    // }
    let source = source_pos;

    // if (source.fainted || !move.hitTargets || move.hasSheerForce) {
    let (source_fainted, hit_targets, has_sheer_force) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };

        (
            source_pokemon.fainted,
            active_move.borrow().hit_targets.clone(),
            active_move.borrow().has_sheer_force,
        )
    };

    if source_fainted || hit_targets.is_empty() || has_sheer_force {
        // make sure the volatiles are cleared
        // for (const pokemon of this.getAllActive()) delete pokemon.volatiles['sparklingaria'];
        let all_active = battle.get_all_active(false);
        for pokemon_pos in all_active {
            Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("sparklingaria"));
        }

        // return;
        return EventResult::Continue;
    }

    // const numberTargets = move.hitTargets.length;
    let number_targets = hit_targets.len();

    // for (const pokemon of move.hitTargets) {
    for pokemon_pos in hit_targets {
        // bypasses Shield Dust when hitting multiple targets
        // if (pokemon !== source && pokemon.isActive && (pokemon.removeVolatile('sparklingaria') || numberTargets > 1) &&
        //     pokemon.status === 'brn') {
        //     pokemon.cureStatus();
        // }
        if pokemon_pos == source {
            continue;
        }

        let (is_active, has_burn, pokemon_ident, pokemon_name) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            (pokemon.is_active, pokemon.status == ID::from("brn"), pokemon.get_slot(), pokemon.name.clone())
        };

        if !is_active {
            continue;
        }

        let removed_volatile = Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("sparklingaria"));

        if (removed_volatile || number_targets > 1) && has_burn {
            let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };

            if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, pokemon_pos, false) {
                let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
                battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
                if removed_nightmare {
                    battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
                }
            }
        }
    }

    EventResult::Continue
}
