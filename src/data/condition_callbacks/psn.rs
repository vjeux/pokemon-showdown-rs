//! Psn Condition
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
///     if (sourceEffect && sourceEffect.effectType === 'Ability') {
///         this.add('-status', target, 'psn', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-status', target, 'psn');
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

    // Check if sourceEffect is an ability
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // this.add('-status', target, 'psn', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
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
                Arg::Str("psn"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-status', target, 'psn');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("psn")]);
    }

    EventResult::Continue
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidualOrder: 9,
/// onResidual(pokemon) {
///     this.damage(pokemon.baseMaxhp / 8);
/// }
/// ```
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // this.damage(pokemon.baseMaxhp / 8);
    // JavaScript uses float division: 1/8 = 0.125, then clampIntRange(0.125, 1) = 1
    // Rust integer division: 1/8 = 0, which wouldn't get clamped
    // Fix: ensure minimum damage of 1 when base_maxhp > 0
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    let damage = if base_maxhp > 0 {
        (base_maxhp / 8).max(1)
    } else {
        0
    };

    battle.damage(damage, Some(pokemon_pos), None, None, false);

    EventResult::Continue
}

