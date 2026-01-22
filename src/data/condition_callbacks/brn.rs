//! Brn Condition
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
///     if (sourceEffect && sourceEffect.id === 'flameorb') {
///         this.add('-status', target, 'brn', '[from] item: Flame Orb');
///     } else if (sourceEffect && sourceEffect.effectType === 'Ability') {
///         this.add('-status', target, 'brn', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-status', target, 'brn');
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

    // if (sourceEffect && sourceEffect.id === 'flameorb')
    let is_flameorb = battle.effect.as_ref().map(|e| e.as_str() == "flameorb").unwrap_or(false);

    // else if (sourceEffect && sourceEffect.effectType === 'Ability')
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_flameorb {
        // this.add('-status', target, 'brn', '[from] item: Flame Orb');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("brn"), Arg::Str("[from] item: Flame Orb")]);
    } else if is_ability {
        // this.add('-status', target, 'brn', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
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
                Arg::Str("brn"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-status', target, 'brn');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("brn")]);
    }

    EventResult::Continue
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidualOrder: 10,
/// onResidual(pokemon) {
///     this.damage(pokemon.baseMaxhp / 16);
/// }
/// ```
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // this.damage(pokemon.baseMaxhp / 16);
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    // JavaScript uses floating point division, then spreadDamage clamps to at least 1
    let damage = (base_maxhp / 16).max(1);

    battle.damage(damage, Some(pokemon_pos), None, None, false);

    EventResult::Continue
}

