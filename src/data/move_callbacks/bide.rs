//! Bide Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// beforeMoveCallback(pokemon) {
///     if (pokemon.volatiles['bide']) return true;
/// }
pub fn before_move_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.volatiles['bide']) return true;
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if pokemon.volatiles.contains_key(&ID::from("bide")) {
        return EventResult::Boolean(true);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.effectState.totalDamage = 0;
    ///     this.add('-start', pokemon, 'move: Bide');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.effectState.totalDamage = 0;
        battle.with_effect_state(|state| {
            state.damage = Some(0);
        });

        // this.add('-start', pokemon, 'move: Bide');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-start",
            &[pokemon_ident.as_str().into(), "move: Bide".into()],
        );

        EventResult::Continue
    }

    /// onDamage(damage, target, source, move) {
    ///     if (!move || move.effectType !== 'Move' || !source) return;
    ///     this.effectState.totalDamage += damage;
    ///     this.effectState.lastDamageSource = source;
    /// }
    pub fn on_damage(
        battle: &mut Battle,
        damage: i32,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // if (!move || move.effectType !== 'Move' || !source) return;
        // effect_id represents the move, source_pos is the source
        if effect_id.is_none() || source_pos.is_none() {
            return EventResult::Continue;
        }

        let source_pos = source_pos.unwrap();

        // this.effectState.totalDamage += damage;
        // this.effectState.lastDamageSource = source;
        // JavaScript: this.effectState.totalDamage, this.effectState.lastDamageSource
        battle.with_effect_state(|state| {
            let current_damage = state.damage.unwrap_or(0);
            state.damage = Some(current_damage + damage);
            // Store source position - we'll use source field which already exists
            // Note: JavaScript stores lastDamageSource, but we can reuse source field
        });

        // We also need to update the source on the effect state
        battle.with_effect_state(|state| {
            state.source = Some(source_pos);
        });

        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (this.effectState.duration === 1) {
    ///         this.add('-end', pokemon, 'move: Bide');
    ///         target = this.effectState.lastDamageSource;
    ///         if (!target || !this.effectState.totalDamage) {
    ///             this.attrLastMove('[still]');
    ///             this.add('-fail', pokemon);
    ///             return false;
    ///         }
    ///         if (!target.isActive) {
    ///             const possibleTarget = this.getRandomTarget(pokemon, this.dex.moves.get('pound'));
    ///             if (!possibleTarget) {
    ///                 this.add('-miss', pokemon);
    ///                 return false;
    ///             }
    ///             target = possibleTarget;
    ///         }
    ///         const moveData: Partial<ActiveMove> = {
    ///             id: 'bide' as ID,
    ///             name: "Bide",
    ///             accuracy: true,
    ///             damage: this.effectState.totalDamage * 2,
    ///             category: "Physical",
    ///             priority: 1,
    ///             flags: { contact: 1, protect: 1 },
    ///             effectType: 'Move',
    ///             type: 'Normal',
    ///         };
    ///         this.actions.tryMoveHit(target, pokemon, moveData as ActiveMove);
    ///         pokemon.removeVolatile('bide');
    ///         return false;
    ///     }
    ///     this.add('-activate', pokemon, 'move: Bide');
    /// }
    pub fn on_before_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        _active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        // Get the volatile state using with_effect_state_ref
        // JavaScript: this.effectState.duration, this.effectState.totalDamage, this.effectState.lastDamageSource
        let (duration, total_damage, last_damage_source) = match battle.with_effect_state_ref(|state| {
            let duration = state.duration.unwrap_or(0);
            let total_damage = state.damage.unwrap_or(0);
            let last_damage_source = state.source;

            (duration, total_damage, last_damage_source)
        }) {
            Some(result) => result,
            None => return EventResult::Continue,
        };

        // if (this.effectState.duration === 1) {
        if duration == 1 {
            // this.add('-end', pokemon, 'move: Bide');
            {
                let pokemon_ident = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.get_slot()
                };
                battle.add(
                    "-end",
                    &[pokemon_ident.as_str().into(), "move: Bide".into()],
                );
            }

            // target = this.effectState.lastDamageSource;
            // if (!target || !this.effectState.totalDamage) {
            let target = last_damage_source;
            if target.is_none() || total_damage == 0 {
                // this.attrLastMove('[still]');
                battle.attr_last_move(&["[still]"]);

                // this.add('-fail', pokemon);
                let pokemon_ident = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.get_slot()
                };
                battle.add("-fail", &[pokemon_ident.as_str().into()]);

                // return false;
                return EventResult::Boolean(false);
            }

            let target = target.unwrap();

            // if (!target.isActive) {
            let target_active = battle
                .pokemon_at(target.0, target.1)
                .map(|p| p.is_active)
                .unwrap_or(false);

            let final_target = if !target_active {
                // const possibleTarget = this.getRandomTarget(pokemon, this.dex.moves.get('pound'));
                let possible_target =
                    battle.get_random_target(pokemon_pos.0, pokemon_pos.1, "normal");

                // if (!possibleTarget) {
                if possible_target.is_none() {
                    // this.add('-miss', pokemon);
                    let pokemon_ident = {
                        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        pokemon.get_slot()
                    };
                    battle.add("-miss", &[pokemon_ident.as_str().into()]);

                    // return false;
                    return EventResult::Boolean(false);
                }

                possible_target.unwrap()
            } else {
                target
            };

            // const moveData: Partial<ActiveMove> = { ... };
            // this.actions.tryMoveHit(target, pokemon, moveData as ActiveMove);
            // Create custom move data for Bide's attack
            use crate::battle_actions::ActiveMove;
            use crate::dex::Accuracy;
            use crate::battle_actions::MoveFlags;

            let bide_move = ActiveMove {
                id: ID::from("bide"),
                name: "Bide".to_string(),
                fullname: "move: Bide".to_string(),
                accuracy: Accuracy::AlwaysHits,
                damage: Some(crate::dex::MoveDamage::Fixed(total_damage * 2)), // Fixed damage: totalDamage * 2
                category: "Physical".to_string(),
                priority: 1,
                flags: MoveFlags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                },
                move_type: "Normal".to_string(),
                target: "normal".to_string(),
                ..Default::default()
            };

            // Set the custom ActiveMove on the battle
            battle.active_move = Some(bide_move);
            battle.active_pokemon = Some(pokemon_pos);
            battle.active_target = Some(final_target);

            // Call tryMoveHit with the target
            use crate::battle_actions::try_move_hit;
            let bide_id_for_hit = ID::from("bide");
            try_move_hit(battle, &[final_target], pokemon_pos, &bide_id_for_hit);

            // pokemon.removeVolatile('bide');
            let bide_id = ID::from("bide");
            Pokemon::remove_volatile(battle, pokemon_pos, &bide_id);

            // return false;
            return EventResult::Boolean(false);
        }

        // this.add('-activate', pokemon, 'move: Bide');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add(
            "-activate",
            &[pokemon_ident.as_str().into(), "move: Bide".into()],
        );

        EventResult::Continue
    }

    /// onMoveAborted(pokemon) {
    ///     pokemon.removeVolatile('bide');
    /// }
    pub fn on_move_aborted(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // pokemon.removeVolatile('bide');
        Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("bide"));

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'move: Bide', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-end', pokemon, 'move: Bide', '[silent]');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                pokemon_ident.as_str().into(),
                "move: Bide".into(),
                "[silent]".into(),
            ],
        );

        EventResult::Continue
    }
}
