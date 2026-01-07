//! Frz Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, sourceEffect) {
///     if (sourceEffect && sourceEffect.effectType === 'Ability') {
///         this.add('-status', target, 'frz', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-status', target, 'frz');
///     }
///     if (target.species.name === 'Shaymin-Sky' && target.baseSpecies.baseSpecies === 'Shaymin') {
///         target.formeChange('Shaymin', this.effect, true);
///     }
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Get target ident
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // if (sourceEffect && sourceEffect.effectType === 'Ability')
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // this.add('-status', target, 'frz', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
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
                Arg::Str("frz"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-status', target, 'frz');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("frz")]);
    }

    // if (target.species.name === 'Shaymin-Sky' && target.baseSpecies.baseSpecies === 'Shaymin')
    let is_shaymin_sky = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.set.species == "Shaymin-Sky" && pokemon.base_species.as_str() == "shaymin"
    };

    if is_shaymin_sky {
        // target.formeChange('Shaymin', this.effect, true);
        crate::pokemon::Pokemon::forme_change(
            battle,
            pokemon_pos,
            crate::dex_data::ID::from("shaymin"),
            battle.effect.clone(),
            true,
            "0",
            None,
        );
    }

    EventResult::Continue
}

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeMovePriority: 10,
/// onBeforeMove(pokemon, target, move) {
///     if (move.flags['defrost'] && !(move.id === 'burnup' && !pokemon.hasType('Fire'))) return;
///     if (this.randomChance(1, 5)) {
///         pokemon.cureStatus();
///         return;
///     }
///     this.add('cant', pokemon, 'frz');
///     return false;
/// }
/// ```
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // Get move info
    let (has_defrost, move_id, _move_name) = match &battle.active_move {
        Some(m) => (m.flags.defrost, m.id.as_str(), m.name.clone()),
        None => return EventResult::Continue,
    };

    // if (move.flags['defrost'] && !(move.id === 'burnup' && !pokemon.hasType('Fire'))) return;
    if has_defrost {
        if move_id == "burnup" {
            let has_fire_type = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.has_type(battle, "fire")
            };
            if !has_fire_type {
                // Don't return early
            } else {
                return EventResult::Continue;
            }
        } else {
            return EventResult::Continue;
        }
    }

    // if (this.randomChance(1, 5))
    if battle.random_chance(1, 5) {
        // pokemon.cureStatus();
        crate::pokemon::Pokemon::cure_status(battle, pokemon_pos, false);
        // return;
        return EventResult::Continue;
    }

    // this.add('cant', pokemon, 'frz');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("cant", &[Arg::String(pokemon_ident), Arg::Str("frz")]);

    // return false;
    EventResult::Boolean(false)
}

/// onModifyMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onModifyMove(move, pokemon) {
///     if (move.flags['defrost']) {
///         this.add('-curestatus', pokemon, 'frz', `[from] move: ${move}`);
///         pokemon.clearStatus();
///     }
/// }
/// ```
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get move info
    let (has_defrost, move_name) = match &battle.active_move {
        Some(m) => (m.flags.defrost, m.name.clone()),
        None => return EventResult::Continue,
    };

    // if (move.flags['defrost'])
    if has_defrost {
        // this.add('-curestatus', pokemon, 'frz', `[from] move: ${move}`);
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-curestatus",
            &[
                Arg::String(pokemon_ident),
                Arg::Str("frz"),
                Arg::String(format!("[from] move: {}", move_name)),
            ],
        );

        // pokemon.clearStatus();
        crate::pokemon::Pokemon::clear_status(battle, pokemon_pos);
    }

    EventResult::Continue
}

/// onAfterMoveSecondary
/// JavaScript source (data/conditions.ts):
/// ```js
/// onAfterMoveSecondary(target, source, move) {
///     if (move.thawsTarget) {
///         target.cureStatus();
///     }
/// }
/// ```
pub fn on_after_move_secondary(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // if (move.thawsTarget)
    let thaws_target = match &battle.active_move {
        Some(m) => m.thaws_target.unwrap_or(false),
        None => false,
    };

    if thaws_target {
        // target.cureStatus();
        crate::pokemon::Pokemon::cure_status(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

/// onDamagingHit
/// JavaScript source (data/conditions.ts):
/// ```js
/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Fire' && move.category !== 'Status' && move.id !== 'polarflare') {
///         target.cureStatus();
///     }
/// }
/// ```
pub fn on_damaging_hit(
    battle: &mut Battle,
    _damage: i32,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // if (move.type === 'Fire' && move.category !== 'Status' && move.id !== 'polarflare')
    let should_thaw = match &battle.active_move {
        Some(m) => {
            m.move_type.as_str() == "fire"
                && m.category != "status"
                && m.id.as_str() != "polarflare"
        },
        None => false,
    };

    if should_thaw {
        // target.cureStatus();
        crate::pokemon::Pokemon::cure_status(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

