//! Metronome Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onStart(pokemon) {
///     pokemon.addVolatile('metronome');
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // pokemon.addVolatile('metronome');
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    Pokemon::add_volatile(battle, pokemon_pos, ID::new("metronome"), None, None, None, None);

    EventResult::Continue
}

/// Condition callbacks for the metronome volatile
/// The volatile is added by the Metronome item and tracks consecutive move usage
pub mod condition {
    use crate::battle::Battle;
    use crate::battle_actions::ActiveMove;
    use crate::dex_data::ID;
    use crate::event::EventResult;

    /// condition.onStart(pokemon) {
    ///     this.effectState.lastMove = '';
    ///     this.effectState.numConsecutive = 0;
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get the metronome volatile and initialize its state
        let metronome_id = ID::new("metronome");
        if let Some(side) = battle.sides.get_mut(pokemon_pos.0) {
            if let Some(pokemon) = side.pokemon.get_mut(pokemon_pos.1) {
                if let Some(state) = pokemon.volatiles.get_mut(&metronome_id) {
                    // Use move_id for lastMove (empty string initially)
                    state.borrow_mut().move_id = Some(String::new());
                    // Use counter for numConsecutive
                    state.borrow_mut().counter = Some(0);
                }
            }
        }
        EventResult::Continue
    }

    /// condition.onTryMove(pokemon, target, move) {
    ///     if (!pokemon.hasItem('metronome')) {
    ///         pokemon.removeVolatile('metronome');
    ///         return;
    ///     }
    ///     if (move.callsMove) return;
    ///     if (this.effectState.lastMove === move.id && pokemon.moveLastTurnResult) {
    ///         this.effectState.numConsecutive++;
    ///     } else if (pokemon.volatiles['twoturnmove']) {
    ///         if (this.effectState.lastMove !== move.id) {
    ///             this.effectState.numConsecutive = 1;
    ///         } else {
    ///             this.effectState.numConsecutive++;
    ///         }
    ///     } else {
    ///         this.effectState.numConsecutive = 0;
    ///     }
    ///     this.effectState.lastMove = move.id;
    /// }
    pub fn on_try_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        active_move: Option<&ActiveMove>,
    ) -> EventResult {
        let move_id_str = match active_move {
            Some(m) => m.id.as_str().to_string(),
            None => return EventResult::Continue,
        };

        // Check if move.callsMove - if so, skip tracking
        if let Some(m) = active_move {
            if m.calls_move {
                return EventResult::Continue;
            }
        }

        // Get data we need from pokemon
        let (has_item, last_move_str, move_last_turn_result, has_two_turn_move, current_num) = {
            if let Some(side) = battle.sides.get(pokemon_pos.0) {
                if let Some(pokemon) = side.pokemon.get(pokemon_pos.1) {
                    // Use has_item which checks if item is being ignored (Magic Room, etc.)
                    let has_item = pokemon.has_item(battle, &["metronome"]);
                    let move_result = pokemon.move_last_turn_result == crate::battle_actions::MoveResult::Success;
                    let has_ttm = pokemon.volatiles.contains_key(&ID::new("twoturnmove"));

                    // Get metronome volatile state
                    let metronome_id = ID::new("metronome");
                    let last_mv = pokemon.volatiles.get(&metronome_id)
                        .map(|s| s.borrow().move_id.clone())
                        .flatten()
                        .unwrap_or_default();
                    let num = pokemon.volatiles.get(&metronome_id)
                        .map(|s| s.borrow().counter)
                        .flatten()
                        .unwrap_or(0);

                    (has_item, last_mv, move_result, has_ttm, num)
                } else {
                    return EventResult::Continue;
                }
            } else {
                return EventResult::Continue;
            }
        };

        // If pokemon doesn't have metronome item, remove the volatile
        if !has_item {
            crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::new("metronome"));
            return EventResult::Continue;
        }

        // Calculate new numConsecutive
        let new_num_consecutive = if last_move_str == move_id_str && move_last_turn_result {
            // Same move and it succeeded last turn - increment
            current_num + 1
        } else if has_two_turn_move {
            // Two-turn move handling
            if last_move_str != move_id_str {
                1
            } else {
                current_num + 1
            }
        } else {
            // Different move or first use - reset to 0
            0
        };

        // Update volatile state
        let metronome_id = ID::new("metronome");
        if let Some(side) = battle.sides.get_mut(pokemon_pos.0) {
            if let Some(pokemon) = side.pokemon.get_mut(pokemon_pos.1) {
                if let Some(state) = pokemon.volatiles.get_mut(&metronome_id) {
                    state.borrow_mut().move_id = Some(move_id_str);
                    state.borrow_mut().counter = Some(new_num_consecutive);
                }
            }
        }

        EventResult::Continue
    }

    /// condition.onModifyDamage(damage, source, target, move) {
    ///     const dmgMod = [4096, 4915, 5734, 6553, 7372, 8192];
    ///     const numConsecutive = this.effectState.numConsecutive > 5 ? 5 : this.effectState.numConsecutive;
    ///     this.debug(`Current Metronome boost: ${dmgMod[numConsecutive]}/4096`);
    ///     return this.chainModify([dmgMod[numConsecutive], 4096]);
    /// }
    ///
    /// This is called for the SOURCE (attacker) pokemon's moves, not the defender
    pub fn on_modify_damage(
        battle: &mut Battle,
        _damage: i32,
        source_pos: Option<(usize, usize)>,
        _target_pos: Option<(usize, usize)>,
        _active_move: Option<&ActiveMove>,
    ) -> EventResult {
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get numConsecutive from the source pokemon's metronome volatile
        let num_consecutive = {
            if let Some(side) = battle.sides.get(source.0) {
                if let Some(pokemon) = side.pokemon.get(source.1) {
                    let metronome_id = ID::new("metronome");
                    pokemon.volatiles.get(&metronome_id)
                        .map(|s| s.borrow().counter)
                        .flatten()
                        .map(|c| if c > 5 { 5 } else { c })
                        .unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        };

        // dmgMod = [4096, 4915, 5734, 6553, 7372, 8192]
        let dmg_mod: [i32; 6] = [4096, 4915, 5734, 6553, 7372, 8192];
        let modifier = dmg_mod[num_consecutive as usize];

        // Apply the modifier using chain_modify_fraction
        battle.chain_modify_fraction(modifier, 4096);

        EventResult::Continue
    }
}
