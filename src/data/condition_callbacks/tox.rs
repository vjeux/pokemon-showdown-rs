//! Tox Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::Arg;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, sourceEffect) {
///     this.effectState.stage = 0;
///     if (sourceEffect && sourceEffect.id === 'toxicorb') {
///         this.add('-status', target, 'tox', '[from] item: Toxic Orb');
///     } else if (sourceEffect && sourceEffect.effectType === 'Ability') {
///         this.add('-status', target, 'tox', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-status', target, 'tox');
///     }
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // Get target ident
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // this.effectState.stage = 0;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon.status_state.stage = Some(0);
    }

    // Check if sourceEffect is toxicorb or ability
    let is_toxicorb = battle.effect.as_ref().map(|e| e.as_str() == "toxicorb").unwrap_or(false);

    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_toxicorb {
        // this.add('-status', target, 'tox', '[from] item: Toxic Orb');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("tox"), Arg::Str("[from] item: Toxic Orb")]);
    } else if is_ability {
        // this.add('-status', target, 'tox', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let source_ident = battle.active_pokemon
            .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
            .map(|p| p.get_slot())
            .unwrap_or_else(|| String::new());

        battle.add(
            "-status",
            &[
                Arg::String(target_ident),
                Arg::Str("tox"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-status', target, 'tox');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("tox")]);
    }

    EventResult::Continue
}

/// onSwitchIn
/// JavaScript source (data/conditions.ts):
/// ```js
/// onSwitchIn() {
///     this.effectState.stage = 0;
/// }
/// ```
pub fn on_switch_in(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.effectState.stage = 0;
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon.status_state.stage = Some(0);

    EventResult::Continue
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidualOrder: 9,
/// onResidual(pokemon) {
///     if (this.effectState.stage < 15) {
///         this.effectState.stage++;
///     }
///     this.damage(this.clampIntRange(pokemon.baseMaxhp / 16, 1) * this.effectState.stage);
/// }
/// ```
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // if (this.effectState.stage < 15)
    let stage = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon.status_state.stage.unwrap_or(0)
    };

    // this.effectState.stage++;
    let new_stage = if stage < 15 {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let incremented = stage + 1;
        pokemon.status_state.stage = Some(incremented);
        incremented
    } else {
        stage
    };

    // this.damage(this.clampIntRange(pokemon.baseMaxhp / 16, 1) * this.effectState.stage);
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    // clampIntRange(pokemon.baseMaxhp / 16, 1) means max(baseMaxhp / 16, 1)
    let damage_per_stage = (base_maxhp / 16).max(1);
    let damage = damage_per_stage * new_stage;

    battle.damage(damage, Some(pokemon_pos), None, None, false);

    EventResult::Continue
}

