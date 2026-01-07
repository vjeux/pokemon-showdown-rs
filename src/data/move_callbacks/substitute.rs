//! Substitute Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;
use crate::Pokemon;

/// onTryHit(source) {
///     if (source.volatiles['substitute']) {
///         this.add('-fail', source, 'move: Substitute');
///         return this.NOT_FAIL;
///     }
///     if (source.hp <= source.maxhp / 4 || source.maxhp === 1) { // Shedinja clause
///         this.add('-fail', source, 'move: Substitute', '[weak]');
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.volatiles['substitute'])
    let has_substitute = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_volatile(&ID::from("substitute"))
    };

    if has_substitute {
        // this.add('-fail', source, 'move: Substitute');
        let source_slot = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-fail",
            &[
                crate::battle::Arg::from(source_slot),
                crate::battle::Arg::from("move: Substitute"),
            ],
        );

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // if (source.hp <= source.maxhp / 4 || source.maxhp === 1)
    let (hp, maxhp) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.hp, source_pokemon.maxhp)
    };

    if hp <= maxhp / 4 || maxhp == 1 {
        // this.add('-fail', source, 'move: Substitute', '[weak]');
        let source_slot = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-fail",
            &[
                crate::battle::Arg::from(source_slot),
                crate::battle::Arg::from("move: Substitute"),
                crate::battle::Arg::from("[weak]"),
            ],
        );

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    EventResult::Continue
}

/// onHit(target) {
///     this.directDamage(target.maxhp / 4);
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.directDamage(target.maxhp / 4);
    let damage = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.maxhp / 4
    };

    battle.direct_damage(damage, Some(target), None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (effect?.id === 'shedtail') {
    ///         this.add('-start', target, 'Substitute', '[from] move: Shed Tail');
    ///     } else {
    ///         this.add('-start', target, 'Substitute');
    ///     }
    ///     this.effectState.hp = Math.floor(target.maxhp / 4);
    ///     if (target.volatiles['partiallytrapped']) {
    ///         this.add('-end', target, target.volatiles['partiallytrapped'].sourceEffect, '[partiallytrapped]', '[silent]');
    ///         delete target.volatiles['partiallytrapped'];
    ///     }
    /// }
    pub fn on_start(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (effect?.id === 'shedtail')
        let is_shedtail = effect_id.map(|id| id == "shedtail").unwrap_or(false);

        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        if is_shedtail {
            // this.add('-start', target, 'Substitute', '[from] move: Shed Tail');
            battle.add(
                "-start",
                &[
                    crate::battle::Arg::from(target_slot.clone()),
                    crate::battle::Arg::from("Substitute"),
                    crate::battle::Arg::from("[from] move: Shed Tail"),
                ],
            );
        } else {
            // this.add('-start', target, 'Substitute');
            battle.add(
                "-start",
                &[
                    crate::battle::Arg::from(target_slot.clone()),
                    crate::battle::Arg::from("Substitute"),
                ],
            );
        }

        // this.effectState.hp = Math.floor(target.maxhp / 4);
        let sub_hp = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.maxhp / 4
        };

        battle.with_effect_state(|state| {
            state
                .data
                .insert("hp".to_string(), serde_json::to_value(sub_hp).unwrap());
        });

        // if (target.volatiles['partiallytrapped'])
        let partially_trapped_info = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon
                .volatiles
                .get(&ID::from("partiallytrapped"))
                .map(|effect_state| {
                    (
                        effect_state.source_effect.clone(),
                    )
                })
        };

        if let Some((source_effect_opt,)) = partially_trapped_info {
            // this.add('-end', target, target.volatiles['partiallytrapped'].sourceEffect, '[partiallytrapped]', '[silent]');
            let source_effect_str = source_effect_opt
                .map(|id| id.to_string())
                .unwrap_or_else(|| "partially trapped".to_string());

            battle.add(
                "-end",
                &[
                    crate::battle::Arg::from(target_slot),
                    crate::battle::Arg::from(source_effect_str),
                    crate::battle::Arg::from("[partiallytrapped]"),
                    crate::battle::Arg::from("[silent]"),
                ],
            );

            // delete target.volatiles['partiallytrapped'];
            Pokemon::remove_volatile(battle, target, &ID::from("partiallytrapped"));
        }

        EventResult::Continue
    }

    /// onTryPrimaryHit(target, source, move) {
    ///     if (target === source || move.flags['bypasssub'] || move.infiltrates) {
    ///         return;
    ///     }
    ///     let damage = this.actions.getDamage(source, target, move);
    ///     if (!damage && damage !== 0) {
    ///         this.add('-fail', source);
    ///         this.attrLastMove('[still]');
    ///         return null;
    ///     }
    ///     if (damage > target.volatiles['substitute'].hp) {
    ///         damage = target.volatiles['substitute'].hp as number;
    ///     }
    ///     target.volatiles['substitute'].hp -= damage;
    ///     source.lastDamage = damage;
    ///     if (target.volatiles['substitute'].hp <= 0) {
    ///         if (move.ohko) this.add('-ohko');
    ///         target.removeVolatile('substitute');
    ///     } else {
    ///         this.add('-activate', target, 'move: Substitute', '[damage]');
    ///     }
    ///     if (move.recoil || move.id === 'chloroblast') {
    ///         this.damage(this.actions.calcRecoilDamage(damage, move, source), source, target, 'recoil');
    ///     }
    ///     if (move.drain) {
    ///         this.heal(Math.ceil(damage * move.drain[0] / move.drain[1]), source, target, 'drain');
    ///     }
    ///     this.singleEvent('AfterSubDamage', move, null, target, source, move, damage);
    ///     this.runEvent('AfterSubDamage', target, source, move, damage);
    ///     return this.HIT_SUBSTITUTE;
    /// }
    pub fn on_try_primary_hit(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _move_id: &str,
    ) -> EventResult {
        use crate::battle_actions::BattleActions;
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target === source || move.flags['bypasssub'] || move.infiltrates)
        if target == source {
            return EventResult::Continue;
        }

        let (bypasses_sub, infiltrates, is_ohko, move_id) = match &battle.active_move {
            Some(m) => {
                let bypasses = m.flags.bypasssub;
                eprintln!("[SUBSTITUTE onTryPrimaryHit] move={}, bypasssub={}, sound={}, infiltrates={}",
                    m.name, m.flags.bypasssub, m.flags.sound, m.infiltrates);
                (
                    bypasses,
                    m.infiltrates,
                    m.ohko.clone(),
                    m.id.clone(),
                )
            }
            None => return EventResult::Continue,
        };

        if bypasses_sub || infiltrates {
            return EventResult::Continue;
        }

        // Get recoil and drain from move data
        let (recoil, drain) = {
            battle.dex.moves.get(&move_id)
                .map(|m| (m.recoil, m.drain))
                .unwrap_or((None, None))
        };

        // let damage = this.actions.getDamage(source, target, move);
        let damage = match crate::battle_actions::get_damage(battle, source, target, &move_id) {
            Some(d) => d,
            None => {
                // if (!damage && damage !== 0)
                // this.add('-fail', source);
                let source_slot = {
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    source_pokemon.get_slot()
                };

                battle.add("-fail", &[crate::battle::Arg::from(source_slot)]);

                // this.attrLastMove('[still]');
                battle.attr_last_move(&["[still]"]);

                // return null;
                return EventResult::Null;
            }
        };

        // Get substitute HP from effect state
        let sub_hp = battle.with_effect_state_ref(|state| {
            state
                .data
                .get("hp")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32)
                .unwrap_or(0)
        }).unwrap_or(0);

        // if (damage > target.volatiles['substitute'].hp)
        let actual_damage = if damage > sub_hp {
            sub_hp
        } else {
            damage
        };

        // target.volatiles['substitute'].hp -= damage;
        battle.with_effect_state(|state| {
            let new_hp = sub_hp - actual_damage;
            state
                .data
                .insert("hp".to_string(), serde_json::to_value(new_hp).unwrap());
        });

        // source.lastDamage = damage;
        {
            let source_mut = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_mut.last_damage = actual_damage;
        }

        // if (target.volatiles['substitute'].hp <= 0)
        let new_sub_hp = battle.with_effect_state_ref(|state| {
            state
                .data
                .get("hp")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32)
                .unwrap_or(0)
        }).unwrap_or(0);

        if new_sub_hp <= 0 {
            // if (move.ohko) this.add('-ohko');
            if is_ohko.is_some() {
                battle.add("-ohko", &[]);
            }

            // target.removeVolatile('substitute');
            Pokemon::remove_volatile(battle, target, &ID::from("substitute"));
        } else {
            // this.add('-activate', target, 'move: Substitute', '[damage]');
            let target_slot = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.get_slot()
            };

            battle.add(
                "-activate",
                &[
                    crate::battle::Arg::from(target_slot),
                    crate::battle::Arg::from("move: Substitute"),
                    crate::battle::Arg::from("[damage]"),
                ],
            );
        }

        // if (move.recoil || move.id === 'chloroblast')
        if recoil.is_some() || move_id.as_str() == "chloroblast" {
            // this.damage(this.actions.calcRecoilDamage(damage, move, source), source, target, 'recoil');
            let source_max_hp = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.maxhp
            };

            let recoil_damage =
                BattleActions::calc_recoil_damage(actual_damage, move_id.as_str(), recoil, source_max_hp);

            battle.damage(recoil_damage, Some(source), Some(target), Some(&Effect::move_(ID::from("recoil"))), false);
        }

        // if (move.drain)
        if let Some((drain_num, drain_denom)) = drain {
            // this.heal(Math.ceil(damage * move.drain[0] / move.drain[1]), source, target, 'drain');
            let heal_amount = ((actual_damage * drain_num + drain_denom - 1) / drain_denom).max(1);
            battle.heal(heal_amount, Some(source), Some(target), Some(&Effect::move_(ID::from("drain"))));
        }

        // this.singleEvent('AfterSubDamage', move, null, target, source, move, damage);
        battle.single_event("AfterSubDamage", &crate::battle::Effect::move_(move_id.clone()), Some(target), Some(source), Some(&Effect::move_(move_id.clone())), Some(EventResult::Number(actual_damage)));

        // this.runEvent('AfterSubDamage', target, source, move, damage);
        battle.run_event("AfterSubDamage", Some(crate::event::EventTarget::Pokemon(target)), Some(source), Some(&Effect::move_(move_id.clone())), EventResult::Number(actual_damage), false, false);

        // return this.HIT_SUBSTITUTE;
        EventResult::HitSubstitute
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Substitute');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Substitute');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Substitute"),
            ],
        );

        EventResult::Continue
    }
}
