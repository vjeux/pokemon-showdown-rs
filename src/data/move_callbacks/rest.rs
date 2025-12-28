//! Rest Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
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
/// ```
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
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
            source_pokemon.get_slot()
        };

        battle.add("-fail", &[source_arg.into(), "heal".into()]);
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
        let (source_arg, source_str) = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let arg = source_pokemon.get_slot();
            let str_repr = arg.to_string();
            (arg, str_repr)
        };

        battle.add("-fail", &[
            source_arg.into(),
            "[from] ability: Insomnia".into(),
            format!("[of] {}", source_str).into(),
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
        let (source_arg, source_str) = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let arg = source_pokemon.get_slot();
            let str_repr = arg.to_string();
            (arg, str_repr)
        };

        battle.add("-fail", &[
            source_arg.into(),
            "[from] ability: Vital Spirit".into(),
            format!("[of] {}", source_str).into(),
        ]);
        return EventResult::Stop;
    }

    EventResult::Continue
}

/// ```ignore
/// onHit(target, source, move) {
///     const result = target.setStatus('slp', source, move);
///     if (!result) return result;
///     target.statusState.time = 3;
///     target.statusState.startTime = 3;
///     this.heal(target.maxhp); // Aesthetic only as the healing happens after you fall asleep in-game
/// }
/// ```
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = pokemon_pos;
    let _source = pokemon_pos;

    // const result = target.setStatus('slp', source, move);
    let result = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_status(ID::from("slp"))
    };

    // if (!result) return result;
    if !result {
        return EventResult::Boolean(false);
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

