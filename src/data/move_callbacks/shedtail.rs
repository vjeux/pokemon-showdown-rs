//! Shed Tail Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(source) {
///     if (!this.canSwitch(source.side) || source.volatiles['commanded']) {
///         this.add('-fail', source);
///         return this.NOT_FAIL;
///     }
///     if (source.volatiles['substitute']) {
///         this.add('-fail', source, 'move: Shed Tail');
///         return this.NOT_FAIL;
///     }
///     if (source.hp <= Math.ceil(source.maxhp / 2)) {
///         this.add('-fail', source, 'move: Shed Tail', '[weak]');
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

    // if (!this.canSwitch(source.side) || source.volatiles['commanded']) {
    //     this.add('-fail', source);
    //     return this.NOT_FAIL;
    // }
    let can_switch = battle.can_switch(source.0);
    let has_commanded = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon
            .volatiles
            .contains_key(&ID::from("commanded"))
    };

    if can_switch == 0 || has_commanded {
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add("-fail", &[source_arg.into()]);
        return EventResult::Boolean(false); // this.NOT_FAIL
    }

    // if (source.volatiles['substitute']) {
    //     this.add('-fail', source, 'move: Shed Tail');
    //     return this.NOT_FAIL;
    // }
    let has_substitute = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon
            .volatiles
            .contains_key(&ID::from("substitute"))
    };

    if has_substitute {
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add("-fail", &[source_arg.into(), "move: Shed Tail".into()]);
        return EventResult::Boolean(false); // this.NOT_FAIL
    }

    // if (source.hp <= Math.ceil(source.maxhp / 2)) {
    //     this.add('-fail', source, 'move: Shed Tail', '[weak]');
    //     return this.NOT_FAIL;
    // }
    let (hp, maxhp) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.hp, source_pokemon.maxhp)
    };

    if hp <= (maxhp + 1) / 2 {
        // Ceiling division
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-fail",
            &[source_arg.into(), "move: Shed Tail".into(), "[weak]".into()],
        );
        return EventResult::Boolean(false); // this.NOT_FAIL
    }

    EventResult::Continue
}

/// onHit(target) {
///     this.directDamage(Math.ceil(target.maxhp / 2));
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // onHit(target) {
    //     this.directDamage(Math.ceil(target.maxhp / 2));
    // }
    let target = pokemon_pos;

    let damage = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.maxhp + 1) / 2 // Ceiling division
    };

    battle.direct_damage(damage, Some(target), None, None);

    EventResult::Continue
}

/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(source) {
    ///                 source.skipBeforeSwitchOutEventFlag = true;
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // source.skipBeforeSwitchOutEventFlag = true;
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        source_pokemon.skip_before_switch_out_event_flag = true;

        EventResult::Continue
    }
}
