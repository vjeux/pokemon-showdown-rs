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
        _pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
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

        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state
                .data
                .insert("hp".to_string(), serde_json::to_value(hp).unwrap());
        }

        // this.effectState.startingTurn = this.getOverflowedTurnCount();
        let starting_turn = battle.get_overflowed_turn_count();

        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "startingTurn".to_string(),
                serde_json::to_value(starting_turn).unwrap(),
            );
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
    pub fn on_residual(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
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
            let hp_to_heal = if let Some(ref effect_state) = battle.current_effect_state {
                effect_state
                    .data
                    .get("hp")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32)
                    .unwrap_or(0)
            } else {
                0
            };

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
                                if let Some(ref effect_state) = battle.current_effect_state {
                                    if let Some(source_pos) = effect_state.source {
                                        match battle.pokemon_at(source_pos.0, source_pos.1) {
                                            Some(p) => p.name.clone(),
                                            None => "".to_string(),
                                        }
                                    } else {
                                        "".to_string()
                                    }
                                } else {
                                    "".to_string()
                                };

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
