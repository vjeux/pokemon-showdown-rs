//! Rest Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.status === 'slp' || source.hasAbility('comatose')) return false;
/// 
///     if (source.hp === source.maxhp) {
///         this.add('-fail', source, 'heal');
///         return null;
///     }
///     // insomnia and vital spirit checks are separate so that the message is accurate in multi-ability mods
///     if (source.hasAbility('insomnia')) {
///         this.add('-fail', source, '[from] ability: Insomnia', `[of] ${source}`);
///         return null;
///     }
///     if (source.hasAbility('vitalspirit')) {
///         this.add('-fail', source, '[from] ability: Vital Spirit', `[of] ${source}`);
///         return null;
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.status === 'slp' || source.hasAbility('comatose')) return false;
    let (status, has_comatose) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.status.clone(), source_pokemon.has_ability(&["comatose"]))
    };

    if status == ID::from("slp") || has_comatose {
        return EventResult::Boolean(false);
    }

    // if (source.hp === source.maxhp) {
    //     this.add('-fail', source, 'heal');
    //     return null;
    // }
    let (hp, maxhp) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.hp, source_pokemon.maxhp)
    };

    if hp == maxhp {
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(source_pokemon)
        };

        battle.add("-fail", &[source_arg, "heal".into()]);
        return EventResult::Stop;
    }

    // insomnia and vital spirit checks are separate so that the message is accurate in multi-ability mods
    // if (source.hasAbility('insomnia')) {
    //     this.add('-fail', source, '[from] ability: Insomnia', `[of] ${source}`);
    //     return null;
    // }
    let has_insomnia = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_ability(&["insomnia"])
    };

    if has_insomnia {
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(source_pokemon)
        };

        battle.add("-fail", &[
            source_arg.clone(),
            "[from] ability: Insomnia".into(),
            format!("[of] {}", source_arg).into(),
        ]);
        return EventResult::Stop;
    }

    // if (source.hasAbility('vitalspirit')) {
    //     this.add('-fail', source, '[from] ability: Vital Spirit', `[of] ${source}`);
    //     return null;
    // }
    let has_vital_spirit = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_ability(&["vitalspirit"])
    };

    if has_vital_spirit {
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(source_pokemon)
        };

        battle.add("-fail", &[
            source_arg.clone(),
            "[from] ability: Vital Spirit".into(),
            format!("[of] {}", source_arg).into(),
        ]);
        return EventResult::Stop;
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     const result = target.setStatus('slp', source, move);
///     if (!result) return result;
///     target.statusState.time = 3;
///     target.statusState.startTime = 3;
///     this.heal(target.maxhp); // Aesthetic only as the healing happens after you fall asleep in-game
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = pokemon_pos;
    let source = pokemon_pos;

    // const result = target.setStatus('slp', source, move);
    let move_id = {
        let active_move = match &battle.active_move {
            Some(active_move) => &active_move.id,
            None => return EventResult::Continue,
        };
        active_move.clone()
    };

    let result = battle.set_status(&ID::from("slp"), target, Some(source), Some(&move_id));

    // if (!result) return result;
    if matches!(result, EventResult::Boolean(false)) {
        return result;
    }

    // target.statusState.time = 3;
    // target.statusState.startTime = 3;
    let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_pokemon.status_state.time = Some(3);
    target_pokemon.status_state.data.insert("startTime".to_string(), serde_json::json!(3));

    // this.heal(target.maxhp); // Aesthetic only as the healing happens after you fall asleep in-game
    let maxhp = target_pokemon.maxhp;
    battle.heal(maxhp, Some(target), None, None);

    EventResult::Continue
}

