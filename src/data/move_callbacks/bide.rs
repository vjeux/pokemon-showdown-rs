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
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "totalDamage".to_string(),
                serde_json::Value::Number(0.into()),
            );
        }

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
        target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // if (!move || move.effectType !== 'Move' || !source) return;
        // effect_id represents the move, source_pos is the source
        if effect_id.is_none() || source_pos.is_none() {
            return EventResult::Continue;
        }

        let source_pos = source_pos.unwrap();

        // Get the volatile state for bide on the target pokemon
        let bide_id = ID::from("bide");
        let pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon.volatiles.get_mut(&bide_id) {
            // this.effectState.totalDamage += damage;
            let current_damage = volatile
                .data
                .get("totalDamage")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            volatile.data.insert(
                "totalDamage".to_string(),
                serde_json::Value::Number((current_damage + damage).into()),
            );

            // this.effectState.lastDamageSource = source;
            volatile.data.insert(
                "lastDamageSource".to_string(),
                serde_json::to_value(source_pos).unwrap_or(serde_json::Value::Null),
            );
        }

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
        _move_id: &str,
    ) -> EventResult {
        // Get the volatile state
        let bide_id = ID::from("bide");
        let (duration, total_damage, last_damage_source) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let volatile = match pokemon.volatiles.get(&bide_id) {
                Some(v) => v,
                None => return EventResult::Continue,
            };

            let duration = volatile.duration.unwrap_or(0);
            let total_damage = volatile
                .data
                .get("totalDamage")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let last_damage_source = volatile
                .data
                .get("lastDamageSource")
                .and_then(|v| serde_json::from_value::<(usize, usize)>(v.clone()).ok());

            (duration, total_damage, last_damage_source)
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
                damage: Some(total_damage * 2), // Fixed damage: totalDamage * 2
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
