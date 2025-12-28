//! Destiny Bond Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return !pokemon.removeVolatile('destinybond');
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // return !pokemon.removeVolatile('destinybond');
    let pokemon = pokemon_pos;

    let removed = {
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.remove_volatile(&ID::from("destinybond"))
    };

    // return !pokemon.removeVolatile('destinybond');
    EventResult::Boolean(!removed)
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Destiny Bond');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-singlemove', pokemon, 'Destiny Bond');
        let pokemon = pokemon_pos;

        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };

        battle.add("-singlemove", &[pokemon_arg, "Destiny Bond".into()]);

        EventResult::Continue
    }

    /// onFaint(target, source, effect) {
    ///     if (!source || !effect || target.isAlly(source)) return;
    ///     if (effect.effectType === 'Move' && !effect.flags['futuremove']) {
    ///         if (source.volatiles['dynamax']) {
    ///             this.add('-hint', "Dynamaxed Pokémon are immune to Destiny Bond.");
    ///             return;
    ///         }
    ///         this.add('-activate', target, 'move: Destiny Bond');
    ///         source.faint();
    ///     }
    /// }
    pub fn on_faint(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (!source || !effect || target.isAlly(source)) return;
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let _effect = match effect_id {
            Some(id) => id,
            None => return EventResult::Continue,
        };

        // if (target.isAlly(source)) return;
        let is_ally = battle.is_ally(target, source);
        if is_ally {
            return EventResult::Continue;
        }

        // if (effect.effectType === 'Move' && !effect.flags['futuremove']) {
        // TODO: Check effect type and flags - requires access to effect data
        // For now, we'll check if the effect ID looks like a move
        let effect_data = battle.dex.get_move_by_id(&ID::from(_effect));
        if let Some(move_data) = effect_data {
            // Check if move has futuremove flag
            if move_data.flags.contains_key("futuremove") {
                return EventResult::Continue;
            }

            // if (source.volatiles['dynamax']) {
            //     this.add('-hint', "Dynamaxed Pokémon are immune to Destiny Bond.");
            //     return;
            // }
            let source_has_dynamax = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.volatiles.contains_key(&ID::from("dynamax"))
            };

            if source_has_dynamax {
                // this.add('-hint', "Dynamaxed Pokémon are immune to Destiny Bond.");
                battle.hint("Dynamaxed Pokémon are immune to Destiny Bond.");
                // return;
                return EventResult::Continue;
            }

            // this.add('-activate', target, 'move: Destiny Bond');
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(target_pokemon)
            };

            battle.add("-activate", &[target_arg, "move: Destiny Bond".into()]);

            // source.faint();
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.faint();
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (move.id === 'destinybond') return;
    ///     this.debug('removing Destiny Bond before attack');
    ///     pokemon.removeVolatile('destinybond');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.id === 'destinybond') return;
        if move_id == "destinybond" {
            // return;
            return EventResult::Continue;
        }

        // this.debug('removing Destiny Bond before attack');
        battle.debug("removing Destiny Bond before attack");

        // pokemon.removeVolatile('destinybond');
        let pokemon = pokemon_pos;
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.remove_volatile(&ID::from("destinybond"));

        EventResult::Continue
    }

    /// onMoveAborted(pokemon, target, move) {
    ///     pokemon.removeVolatile('destinybond');
    /// }
    pub fn on_move_aborted(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // pokemon.removeVolatile('destinybond');
        let pokemon = pokemon_pos;
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.remove_volatile(&ID::from("destinybond"));

        EventResult::Continue
    }
}
