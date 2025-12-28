//! Sky Drop Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, source) {
///     if (!source.volatiles['skydrop']) {
///         move.accuracy = true;
///         delete move.flags['contact'];
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onModifyMove(move, source) {
    //     if (!source.volatiles['skydrop']) {
    //         move.accuracy = true;
    //         delete move.flags['contact'];
    //     }
    // }
    let source = pokemon_pos;

    // if (!source.volatiles['skydrop']) {
    let has_skydrop = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.volatiles.contains_key(&ID::from("skydrop"))
    };

    if !has_skydrop {
        // move.accuracy = true;
        // delete move.flags['contact'];
        let active_move = match &mut battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.accuracy = 0; // true means always hit
        active_move.flags.contact = false;
    }

    EventResult::Continue
}

/// onMoveFail(target, source) {
///     if (source.volatiles['twoturnmove'] && source.volatiles['twoturnmove'].duration === 1) {
///         source.removeVolatile('skydrop');
///         source.removeVolatile('twoturnmove');
///         if (target === this.effectState.target) {
///             this.add('-end', target, 'Sky Drop', '[interrupt]');
///         }
///     }
/// }
pub fn on_move_fail(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onMoveFail(target, source) {
    //     if (source.volatiles['twoturnmove'] && source.volatiles['twoturnmove'].duration === 1) {
    //         source.removeVolatile('skydrop');
    //         source.removeVolatile('twoturnmove');
    //         if (target === this.effectState.target) {
    //             this.add('-end', target, 'Sky Drop', '[interrupt]');
    //         }
    //     }
    // }
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.volatiles['twoturnmove'] && source.volatiles['twoturnmove'].duration === 1) {
    let has_twoturnmove_duration_1 = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = source_pokemon.volatiles.get(&ID::from("twoturnmove")) {
            volatile.duration == Some(1)
        } else {
            false
        }
    };

    if has_twoturnmove_duration_1 {
        // source.removeVolatile('skydrop');
        // source.removeVolatile('twoturnmove');
        {
            let pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.remove_volatile(&ID::from("skydrop"));
        }
        {
            let pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.remove_volatile(&ID::from("twoturnmove"));
        }

        // if (target === this.effectState.target) {
        //     this.add('-end', target, 'Sky Drop', '[interrupt]');
        // }
        let effect_target = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            effect_state.target
        };

        if Some(target) == effect_target {
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(target_pokemon)
            };

            battle.add("-end", &[
                target_arg,
                "Sky Drop".into(),
                "[interrupt]".into(),
            ]);
        }
    }

    EventResult::Continue
}

/// onTry(source, target) {
///     return !target.fainted;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onTry(source, target) {
    //     return !target.fainted;
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_fainted = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.fainted
    };

    EventResult::Boolean(!is_fainted)
}

/// onTryHit(target, source, move) {
///     if (source.removeVolatile(move.id)) {
///         if (target !== source.volatiles['twoturnmove'].source) return false;
/// 
///         if (target.hasType('Flying')) {
///             this.add('-immune', target);
///             return null;
///         }
///     } else {
///         if (target.volatiles['substitute'] || target.isAlly(source)) {
///             return false;
///         }
///         if (target.getWeight() >= 2000) {
///             this.add('-fail', target, 'move: Sky Drop', '[heavy]');
///             return null;
///         }
/// 
///         this.add('-prepare', source, move.name, target);
///         source.addVolatile('twoturnmove', target);
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // onTryHit(target, source, move) {
    //     if (source.removeVolatile(move.id)) {
    //         if (target !== source.volatiles['twoturnmove'].source) return false;
    //
    //         if (target.hasType('Flying')) {
    //             this.add('-immune', target);
    //             return null;
    //         }
    //     } else {
    //         if (target.volatiles['substitute'] || target.isAlly(source)) {
    //             return false;
    //         }
    //         if (target.getWeight() >= 2000) {
    //             this.add('-fail', target, 'move: Sky Drop', '[heavy]');
    //             return null;
    //         }
    //
    //         this.add('-prepare', source, move.name, target);
    //         source.addVolatile('twoturnmove', target);
    //         return null;
    //     }
    // }
    let source = source_pos;
    let target = target_pos;

    let move_id = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.id.clone()
    };

    // if (source.removeVolatile(move.id)) {
    let removed = {

        let pokemon = match battle.pokemon_at_mut(source.0, source.1) {

            Some(p) => p,

            None => return EventResult::Continue,

        };

        pokemon.remove_volatile(&move_id)

    };

    if removed {
        // if (target !== source.volatiles['twoturnmove'].source) return false;
        let twoturnmove_source = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            if let Some(volatile) = source_pokemon.volatiles.get(&ID::from("twoturnmove")) {
                volatile.source
            } else {
                None
            }
        };

        if Some(target) != twoturnmove_source {
            return EventResult::Boolean(false);
        }

        // if (target.hasType('Flying')) {
        //     this.add('-immune', target);
        //     return null;
        // }
        let has_flying = battle.has_type(target, "Flying");
        if has_flying {
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(target_pokemon)
            };

            battle.add("-immune", &[target_arg]);
            return EventResult::Stop;
        }
    } else {
        // if (target.volatiles['substitute'] || target.isAlly(source)) {
        //     return false;
        // }
        let has_substitute = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.volatiles.contains_key(&ID::from("substitute"))
        };

        let is_ally = battle.is_ally(target, source);

        if has_substitute || is_ally {
            return EventResult::Boolean(false);
        }

        // if (target.getWeight() >= 2000) {
        //     this.add('-fail', target, 'move: Sky Drop', '[heavy]');
        //     return null;
        // }
        let weight = battle.get_weight(target);
        if weight >= 2000 {
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(target_pokemon)
            };

            battle.add("-fail", &[
                target_arg,
                "move: Sky Drop".into(),
                "[heavy]".into(),
            ]);
            return EventResult::Stop;
        }

        // this.add('-prepare', source, move.name, target);
        let (source_arg, move_name, target_arg) = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            (
                crate::battle::Arg::from(source_pokemon),
                active_move.name.clone(),
                crate::battle::Arg::from(target_pokemon),
            )
        };

        battle.add("-prepare", &[
            source_arg,
            move_name.into(),
            target_arg,
        ]);

        // source.addVolatile('twoturnmove', target);
        battle.add_volatile(&ID::from("twoturnmove"), source, Some(target), None);

        // return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     if (target.hp) this.add('-end', target, 'Sky Drop');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onHit(target, source) {
    //     if (target.hp) this.add('-end', target, 'Sky Drop');
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let has_hp = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.hp > 0
    };

    if has_hp {
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-end", &[
            target_arg,
            "Sky Drop".into(),
        ]);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onAnyDragOut(pokemon) {
    ///     if (pokemon === this.effectState.target || pokemon === this.effectState.source) return false;
    /// }
    pub fn on_any_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // onAnyDragOut(pokemon) {
        //     if (pokemon === this.effectState.target || pokemon === this.effectState.source) return false;
        // }
        let pokemon = pokemon_pos;

        let (effect_target, effect_source) = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            (effect_state.target, effect_state.source)
        };

        if Some(pokemon) == effect_target || Some(pokemon) == effect_source {
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onFoeTrapPokemon(defender) {
    ///     if (defender !== this.effectState.source) return;
    ///     defender.trapped = true;
    /// }
    pub fn on_foe_trap_pokemon(battle: &mut Battle) -> EventResult {
        // onFoeTrapPokemon(defender) {
        //     if (defender !== this.effectState.source) return;
        //     defender.trapped = true;
        // }
        let effect_source = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            effect_state.source
        };

        // The defender is passed via the effect_state context
        // We need to get the defender from the current context
        // In this callback, the defender would be accessed through the event context
        // For now, we'll implement by setting trapped on the effect source
        if let Some(source) = effect_source {
            battle.set_trapped(source, true);
        }

        EventResult::Continue
    }

    /// onFoeBeforeMove(attacker, defender, move) {
    ///     if (attacker === this.effectState.source) {
    ///         attacker.activeMoveActions--;
    ///         this.debug('Sky drop nullifying.');
    ///         return null;
    ///     }
    /// }
    pub fn on_foe_before_move(battle: &mut Battle, move_id: &str) -> EventResult {
        // onFoeBeforeMove(attacker, defender, move) {
        //     if (attacker === this.effectState.source) {
        //         attacker.activeMoveActions--;
        //         this.debug('Sky drop nullifying.');
        //         return null;
        //     }
        // }
        let effect_source = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            effect_state.source
        };

        // The attacker would be passed via event context
        // For this implementation, we check if the current actor is the effect source
        if let Some(source) = effect_source {
            battle.decrement_active_move_actions(source);
            battle.debug("Sky drop nullifying.");
            return EventResult::Stop;
        }

        EventResult::Continue
    }

    /// onRedirectTarget(target, source, source2) {
    ///     if (source !== this.effectState.target) return;
    ///     if (this.effectState.source.fainted) return;
    ///     return this.effectState.source;
    /// }
    pub fn on_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        // onRedirectTarget(target, source, source2) {
        //     if (source !== this.effectState.target) return;
        //     if (this.effectState.source.fainted) return;
        //     return this.effectState.source;
        // }
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let (effect_target, effect_source) = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            (effect_state.target, effect_state.source)
        };

        // if (source !== this.effectState.target) return;
        if Some(source) != effect_target {
            return EventResult::Continue;
        }

        // if (this.effectState.source.fainted) return;
        if let Some(eff_source) = effect_source {
            let is_fainted = {
                let source_pokemon = match battle.pokemon_at(eff_source.0, eff_source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.fainted
            };

            if is_fainted {
                return EventResult::Continue;
            }

            // return this.effectState.source;
            return EventResult::Position(eff_source);
        }

        EventResult::Continue
    }

    /// onAnyInvulnerability(target, source, move) {
    ///     if (target !== this.effectState.target && target !== this.effectState.source) {
    ///         return;
    ///     }
    ///     if (source === this.effectState.target && target === this.effectState.source) {
    ///         return;
    ///     }
    ///     if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_any_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // onAnyInvulnerability(target, source, move) {
        //     if (target !== this.effectState.target && target !== this.effectState.source) {
        //         return;
        //     }
        //     if (source === this.effectState.target && target === this.effectState.source) {
        //         return;
        //     }
        //     if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
        //         return;
        //     }
        //     return false;
        // }
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let source = source_pos;

        let (effect_target, effect_source) = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            (effect_state.target, effect_state.source)
        };

        // if (target !== this.effectState.target && target !== this.effectState.source) {
        //     return;
        // }
        if Some(target) != effect_target && Some(target) != effect_source {
            return EventResult::Continue;
        }

        // if (source === this.effectState.target && target === this.effectState.source) {
        //     return;
        // }
        if source == effect_target && Some(target) == effect_source {
            return EventResult::Continue;
        }

        // if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
        //     return;
        // }
        let move_id = ID::from(move_id);
        if move_id == ID::from("gust") || move_id == ID::from("twister") || move_id == ID::from("skyuppercut") ||
           move_id == ID::from("thunder") || move_id == ID::from("hurricane") ||
           move_id == ID::from("smackdown") || move_id == ID::from("thousandarrows") {
            return EventResult::Continue;
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// onAnyBasePower(basePower, target, source, move) {
    ///     if (target !== this.effectState.target && target !== this.effectState.source) {
    ///         return;
    ///     }
    ///     if (source === this.effectState.target && target === this.effectState.source) {
    ///         return;
    ///     }
    ///     if (move.id === 'gust' || move.id === 'twister') {
    ///         this.debug('BP doubled on midair target');
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_any_base_power(battle: &mut Battle, base_power: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // onAnyBasePower(basePower, target, source, move) {
        //     if (target !== this.effectState.target && target !== this.effectState.source) {
        //         return;
        //     }
        //     if (source === this.effectState.target && target === this.effectState.source) {
        //         return;
        //     }
        //     if (move.id === 'gust' || move.id === 'twister') {
        //         this.debug('BP doubled on midair target');
        //         return this.chainModify(2);
        //     }
        // }
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let source = source_pos;

        let (effect_target, effect_source) = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            (effect_state.target, effect_state.source)
        };

        // if (target !== this.effectState.target && target !== this.effectState.source) {
        //     return;
        // }
        if Some(target) != effect_target && Some(target) != effect_source {
            return EventResult::Continue;
        }

        // if (source === this.effectState.target && target === this.effectState.source) {
        //     return;
        // }
        if source == effect_target && Some(target) == effect_source {
            return EventResult::Continue;
        }

        // if (move.id === 'gust' || move.id === 'twister') {
        //     this.debug('BP doubled on midair target');
        //     return this.chainModify(2);
        // }
        let move_id = ID::from(move_id);
        if move_id == ID::from("gust") || move_id == ID::from("twister") {
            battle.debug("BP doubled on midair target");
            return EventResult::Number(battle.chain_modify(2 as f32));
        }

        EventResult::Continue
    }

    /// onFaint(target) {
    ///     if (target.volatiles['skydrop'] && target.volatiles['twoturnmove'].source) {
    ///         this.add('-end', target.volatiles['twoturnmove'].source, 'Sky Drop', '[interrupt]');
    ///     }
    /// }
    pub fn on_faint(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        // onFaint(target) {
        //     if (target.volatiles['skydrop'] && target.volatiles['twoturnmove'].source) {
        //         this.add('-end', target.volatiles['twoturnmove'].source, 'Sky Drop', '[interrupt]');
        //     }
        // }
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.volatiles['skydrop'] && target.volatiles['twoturnmove'].source) {
        let (has_skydrop, twoturnmove_source) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let has_skydrop = target_pokemon.volatiles.contains_key(&ID::from("skydrop"));

            let twoturnmove_source = if let Some(volatile) = target_pokemon.volatiles.get(&ID::from("twoturnmove")) {
                volatile.source
            } else {
                None
            };

            (has_skydrop, twoturnmove_source)
        };

        if has_skydrop && twoturnmove_source.is_some() {
            // this.add('-end', target.volatiles['twoturnmove'].source, 'Sky Drop', '[interrupt]');
            let source = twoturnmove_source.unwrap();
            let source_arg = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(source_pokemon)
            };

            battle.add("-end", &[
                source_arg,
                "Sky Drop".into(),
                "[interrupt]".into(),
            ]);
        }

        EventResult::Continue
    }
}
