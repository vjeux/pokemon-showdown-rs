//! Heal Bell Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     this.add('-activate', source, 'move: Heal Bell');
///     let success = false;
///     const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
///     for (const ally of allies) {
///         if (ally !== source && !this.suppressingAbility(ally)) {
///             if (ally.hasAbility('soundproof')) {
///                 this.add('-immune', ally, '[from] ability: Soundproof');
///                 continue;
///             }
///             if (ally.hasAbility('goodasgold')) {
///                 this.add('-immune', ally, '[from] ability: Good as Gold');
///                 continue;
///             }
///         }
///         if (ally.cureStatus()) success = true;
///     }
///     return success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.add('-activate', source, 'move: Heal Bell');
    let source_arg = crate::battle::Arg::Pos(source.0, source.1);
    battle.add("-activate", &[source_arg, "move: Heal Bell".into()]);

    // let success = false;
    let mut success = false;

    // const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
    // For now, we'll just iterate through the target's side pokemon
    // TODO: Add ally side support when double battles are fully implemented
    let allies: Vec<(usize, usize)> = battle.sides[target.0].active.iter()
        .enumerate()
        .filter_map(|(i, &active)| if active.is_some() { Some((target.0, i)) } else { None })
        .collect();

    // for (const ally of allies) {
    for ally_pos in allies {
        // if (ally !== source && !this.suppressingAbility(ally)) {
        let is_not_source = ally_pos != source;
        let suppressing_ability = if is_not_source {
            battle.suppressing_ability(Some(ally_pos))
        } else {
            false
        };

        if is_not_source && !suppressing_ability {
            // if (ally.hasAbility('soundproof')) {
            let has_soundproof = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                ally.has_ability(&["soundproof"])
            };

            if has_soundproof {
                // this.add('-immune', ally, '[from] ability: Soundproof');
                let ally_arg = crate::battle::Arg::Pos(ally_pos.0, ally_pos.1);
                battle.add("-immune", &[ally_arg, "[from] ability: Soundproof".into()]);
                // continue;
                continue;
            }

            // if (ally.hasAbility('goodasgold')) {
            let has_goodasgold = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                ally.has_ability(&["goodasgold"])
            };

            if has_goodasgold {
                // this.add('-immune', ally, '[from] ability: Good as Gold');
                let ally_arg = crate::battle::Arg::Pos(ally_pos.0, ally_pos.1);
                battle.add("-immune", &[ally_arg, "[from] ability: Good as Gold".into()]);
                // continue;
                continue;
            }
        }

        // if (ally.cureStatus()) success = true;
        let cured = {
            let ally = match battle.pokemon_at_mut(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally.cure_status(battle)
        };

        if cured {
            success = true;
        }
    }

    // return success;
    EventResult::Boolean(success)
}

