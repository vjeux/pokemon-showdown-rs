//! Substitute Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
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
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _move_id: &str,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
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
