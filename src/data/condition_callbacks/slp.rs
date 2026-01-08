//! Slp Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, sourceEffect) {
///     if (sourceEffect && sourceEffect.effectType === 'Ability') {
///         this.add('-status', target, 'slp', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else if (sourceEffect && sourceEffect.effectType === 'Move') {
///         this.add('-status', target, 'slp', `[from] move: ${sourceEffect.name}`);
///     } else {
///         this.add('-status', target, 'slp');
///     }
///     // 1-3 turns
///     this.effectState.startTime = this.random(2, 5);
///     this.effectState.time = this.effectState.startTime;
///
///     if (target.removeVolatile('nightmare')) {
///         this.add('-end', target, 'Nightmare', '[silent]');
///     }
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // Get target ident
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // Check if sourceEffect is an ability or move
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    let is_move = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.moves().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // this.add('-status', target, 'slp', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Get source Pokemon ident
        let source_ident = battle.active_pokemon
            .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
            .map(|p| p.get_slot())
            .unwrap_or_else(|| String::new());

        battle.add(
            "-status",
            &[
                Arg::String(target_ident.clone()),
                Arg::Str("slp"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else if is_move {
        // this.add('-status', target, 'slp', `[from] move: ${sourceEffect.name}`);
        let move_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.moves().get(eff_id.as_str()))
            .map(|m| m.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        battle.add(
            "-status",
            &[
                Arg::String(target_ident.clone()),
                Arg::Str("slp"),
                Arg::String(format!("[from] move: {}", move_name)),
            ],
        );
    } else {
        // this.add('-status', target, 'slp');
        battle.add("-status", &[Arg::String(target_ident.clone()), Arg::Str("slp")]);
    }

    // this.effectState.startTime = this.random(2, 5);
    // this.effectState.time = this.effectState.startTime;
    let start_time = battle.random_with_range(2, 5);

    // Set status state time using with_effect_state
    // JavaScript: this.effectState.startTime, this.effectState.time
    battle.with_effect_state(|state| {
        state.start_time = Some(start_time);
        state.time = Some(start_time);
    });

    // if (target.removeVolatile('nightmare'))
    let nightmare_id = ID::from("nightmare");
    let removed_nightmare = crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &nightmare_id);

    if removed_nightmare {
        // this.add('-end', target, 'Nightmare', '[silent]');
        battle.add("-end", &[Arg::String(target_ident), Arg::Str("Nightmare"), Arg::Str("[silent]")]);
    }

    EventResult::Continue
}

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeMovePriority: 10,
/// onBeforeMove(pokemon, target, move) {
///     if (pokemon.hasAbility('earlybird')) {
///         pokemon.statusState.time--;
///     }
///     pokemon.statusState.time--;
///     if (pokemon.statusState.time <= 0) {
///         pokemon.cureStatus();
///         return;
///     }
///     this.add('cant', pokemon, 'slp');
///     if (move.sleepUsable) {
///         return;
///     }
///     return false;
/// }
/// ```
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // if (pokemon.hasAbility('earlybird'))
    let has_earlybird = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_ability(battle, &["earlybird"])
    };

    if has_earlybird {
        // pokemon.statusState.time--;
        // JavaScript: this.effectState.time-- (extra decrement for Early Bird)
        battle.with_effect_state(|state| {
            if let Some(time) = state.time {
                state.time = Some(time - 1);
            }
        });
    }

    // pokemon.statusState.time--;
    // JavaScript: this.effectState.time--
    battle.with_effect_state(|state| {
        if let Some(time) = state.time {
            state.time = Some(time - 1);
        }
    });

    // if (pokemon.statusState.time <= 0)
    // JavaScript: this.effectState.time <= 0
    let time_expired = battle.with_effect_state_ref(|state| {
        state.time.map(|t| t <= 0).unwrap_or(false)
    }).unwrap_or(false);

    if time_expired {
        // pokemon.cureStatus();
        crate::pokemon::Pokemon::cure_status(battle, pokemon_pos, false);
        // return;
        return EventResult::Continue;
    }

    // this.add('cant', pokemon, 'slp');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("cant", &[Arg::String(pokemon_ident), Arg::Str("slp")]);

    // if (move.sleepUsable)
    let sleep_usable = match &battle.active_move {
        Some(m) => m.sleep_usable,
        None => false,
    };

    if sleep_usable {
        // return;
        return EventResult::Continue;
    }

    // return false;
    EventResult::Boolean(false)
}

