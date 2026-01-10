//! Wish Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(pokemon, source) {
    ///     this.effectState.hp = source.maxhp / 2;
    ///     this.effectState.startingTurn = this.getOverflowedTurnCount();
    ///     if (this.effectState.startingTurn === 255) {
    ///         this.hint(`In Gen 8+, Wish will never resolve when used on the ${this.turn}th turn.`);
    ///     }
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // this.effectState.hp = source.maxhp / 2;
        let hp = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.maxhp / 2
        } else {
            return EventResult::Continue;
        };

        // this.effectState.startingTurn = this.getOverflowedTurnCount();
        let starting_turn = battle.get_overflowed_turn_count();

        // Store in the slot condition's state directly
        // pokemon_pos is the slot where the wish condition was placed
        let side_idx = pokemon_pos.0;
        let slot = pokemon_pos.1;

        if let Some(side) = battle.sides.get_mut(side_idx) {
            if let Some(slot_conds) = side.slot_conditions.get_mut(slot) {
                if let Some(wish_state) = slot_conds.get_mut(&ID::from("wish")) {
                    wish_state.hp = Some(hp);
                    wish_state.starting_turn = Some(starting_turn);
                    wish_state.source = source_pos;
                    wish_state.source_slot = source_pos.map(|p| p.1);
                }
            }
        }

        // if (this.effectState.startingTurn === 255)
        if starting_turn == 255 {
            battle.hint(
                &format!(
                    "In Gen 8+, Wish will never resolve when used on the {}th turn.",
                    battle.turn
                ),
                false,
                None,
            );
        }

        EventResult::Continue
    }

    /// onResidual(target: Pokemon) {
    ///     if (this.getOverflowedTurnCount() <= this.effectState.startingTurn) return;
    ///     target.side.removeSlotCondition(this.getAtSlot(this.effectState.sourceSlot), 'wish');
    /// }
    pub fn on_residual(battle: &mut Battle) -> EventResult {
        use crate::dex_data::ID;

        // if (this.getOverflowedTurnCount() <= this.effectState.startingTurn) return;
        let (starting_turn, target_pos, source_slot) = battle.with_effect_state_ref(|state| {
            let starting_turn = state.starting_turn.unwrap_or(0);
            (starting_turn, state.target, state.source_slot)
        }).unwrap_or((0, None, None));

        let current_turn = battle.get_overflowed_turn_count();

        if current_turn <= starting_turn {
            return EventResult::Continue;
        }

        // target.side.removeSlotCondition(this.getAtSlot(this.effectState.sourceSlot), 'wish');
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let slot = match source_slot {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        // Use Battle method that fires the End event
        battle.remove_slot_condition(target.0, slot, &ID::from("wish"));

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     if (target && !target.fainted) {
    ///         const damage = this.heal(this.effectState.hp, target, target);
    ///         if (damage) {
    ///             this.add('-heal', target, target.getHealth, '[from] move: Wish', '[wisher] ' + this.effectState.source.name);
    ///         }
    ///     }
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target && !target.fainted)
        let is_fainted = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.fainted
        };

        if !is_fainted {
            // const damage = this.heal(this.effectState.hp, target, target);
            let hp_to_heal = battle.with_effect_state_ref(|state| {
                state.hp.unwrap_or(0)
            }).unwrap_or(0);

            if hp_to_heal > 0 {
                let damage = battle.heal(hp_to_heal, Some(target), Some(target), None);

                // if (damage)
                if let Some(heal_amount) = damage {
                    if heal_amount > 0 {
                        // this.add('-heal', target, target.getHealth, '[from] move: Wish', '[wisher] ' + this.effectState.source.name);
                        let (target_slot, target_health, source_name) = {
                            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                                Some(p) => p,
                                None => return EventResult::Continue,
                            };

                            let source_name =
                                battle.with_effect_state_ref(|state| {
                                    if let Some(source_pos) = state.source {
                                        match battle.pokemon_at(source_pos.0, source_pos.1) {
                                            Some(p) => p.name.clone(),
                                            None => "".to_string(),
                                        }
                                    } else {
                                        "".to_string()
                                    }
                                }).unwrap_or_else(|| "".to_string());

                            (
                                target_pokemon.get_slot(),
                                target_pokemon.get_health(),
                                source_name,
                            )
                        };

                        battle.add(
                            "-heal",
                            &[
                                crate::battle::Arg::from(target_slot),
                                crate::battle::Arg::from(target_health),
                                crate::battle::Arg::from("[from] move: Wish"),
                                crate::battle::Arg::from(format!("[wisher] {}", source_name)),
                            ],
                        );
                    }
                }
            }
        }

        EventResult::Continue
    }
}
