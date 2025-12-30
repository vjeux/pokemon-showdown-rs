//! Teatime Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitField(target, source, move) {
///     const targets: Pokemon[] = [];
///     for (const pokemon of this.getAllActive()) {
///         if (this.runEvent('Invulnerability', pokemon, source, move) === false) {
///             this.add('-miss', source, pokemon);
///         } else if (this.runEvent('TryHit', pokemon, source, move) && pokemon.getItem().isBerry) {
///             targets.push(pokemon);
///         }
///     }
///     this.add('-fieldactivate', 'move: Teatime');
///     if (!targets.length) {
///         this.add('-fail', source, 'move: Teatime');
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     for (const pokemon of targets) {
///         pokemon.eatItem(true);
///     }
/// }
pub fn on_hit_field(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {

    let source = source_pos;

    // const targets: Pokemon[] = [];
    // for (const pokemon of this.getAllActive()) {
    let all_active = battle.get_all_active(false);
    let mut targets: Vec<(usize, usize)> = Vec::new();

    for pokemon_pos in all_active {
        // if (this.runEvent('Invulnerability', pokemon, source, move) === false)
        let invuln_result = battle.run_event_bool(
            "Invulnerability",
            Some(pokemon_pos),
            source,
            None,
        );

        if !invuln_result {
            // this.add('-miss', source, pokemon);
            if let Some(src_pos) = source {
                let (source_slot, pokemon_slot) = {
                    let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let target_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    (source_pokemon.get_slot(), target_pokemon.get_slot())
                };

                battle.add(
                    "-miss",
                    &[
                        crate::battle::Arg::from(source_slot),
                        crate::battle::Arg::from(pokemon_slot),
                    ],
                );
            }
        } else {
            // else if (this.runEvent('TryHit', pokemon, source, move) && pokemon.getItem().isBerry)
            let try_hit_result = battle.run_event_bool(
                "TryHit",
                Some(pokemon_pos),
                source,
                None,
            );

            let has_berry = {
                let pokemon_ref = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let item_id = pokemon_ref.get_item();
                if let Some(item_data) = battle.dex.items().get_by_id(item_id) {
                    item_data.is_berry
                } else {
                    false
                }
            };

            if try_hit_result && has_berry {
                // targets.push(pokemon);
                targets.push(pokemon_pos);
            }
        }
    }

    // this.add('-fieldactivate', 'move: Teatime');
    battle.add("-fieldactivate", &[crate::battle::Arg::from("move: Teatime")]);

    // if (!targets.length) {
    if targets.is_empty() {
        // this.add('-fail', source, 'move: Teatime');
        if let Some(src_pos) = source {
            let source_slot = {
                let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.get_slot()
            };

            battle.add(
                "-fail",
                &[
                    crate::battle::Arg::from(source_slot),
                    crate::battle::Arg::from("move: Teatime"),
                ],
            );
        }

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // for (const pokemon of targets) {
    //     pokemon.eatItem(true);
    // }
    for pokemon_pos in targets {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => continue,
        };
        pokemon_mut.eat_item(true);
    }

    EventResult::Continue
}
