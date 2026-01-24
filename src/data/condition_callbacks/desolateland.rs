//! Desolateland Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;

/// onTryMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onTryMovePriority: 1,
/// onTryMove(attacker, defender, move) {
///     if (move.type === 'Water' && move.category !== 'Status') {
///         this.debug('Desolate Land water suppress');
///         this.add('-fail', attacker, move, '[from] Desolate Land');
///         this.attrLastMove('[still]');
///         return null;
///     }
/// }
/// ```
pub fn on_try_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.type === 'Water' && move.category !== 'Status')
    if active_move_ref.move_type == "Water" && active_move_ref.category != "Status" {
        // this.debug('Desolate Land water suppress');
        // this.add('-fail', attacker, move, '[from] Desolate Land');
        let attacker_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        let move_name = active_move_ref.name.clone();

        battle.add(
            "-fail",
            &[
                Arg::String(attacker_ident),
                Arg::String(move_name),
                Arg::Str("[from] Desolate Land"),
            ],
        );

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

/// onWeatherModifyDamage
/// JavaScript source (data/conditions.ts):
/// ```js
/// onWeatherModifyDamage(damage, attacker, defender, move) {
///     if (defender.hasItem('utilityumbrella')) return;
///     if (move.type === 'Fire') {
///         this.debug('Sunny Day fire boost');
///         return this.chainModify(1.5);
///     }
/// }
/// ```
pub fn on_weather_modify_damage(
    battle: &mut Battle,
    _damage: i32,
    _attacker_pos: Option<(usize, usize)>,
    _defender_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Get defender position
    let defender_pos = match battle.active_target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (defender.hasItem('utilityumbrella')) return;
    let defender_has_umbrella = {
        let defender = match battle.pokemon_at(defender_pos.0, defender_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        defender.has_item(battle, &["utilityumbrella"])
    };

    if defender_has_umbrella {
        return EventResult::Continue;
    }

    // if (move.type === 'Fire')
    if active_move_ref.move_type == "Fire" {
        // this.debug('Sunny Day fire boost');
        // return this.chainModify(1.5);
        // JavaScript's chainModify modifies event.modifier and returns void
        // The modifier is then applied at the end of runEvent
        battle.chain_modify(1.5);
        return EventResult::Continue;
    }

    EventResult::Continue
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(field, source, effect) {
///     this.add('-weather', 'DesolateLand', '[from] ability: ' + effect.name, `[of] ${source}`);
/// }
/// ```
pub fn on_field_start(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'DesolateLand', '[from] ability: ' + effect.name, `[of] ${source}`);
    let ability_name = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .map(|ab| ab.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    // Get source Pokemon ident
    let source_ident = battle.field.weather_state.borrow().source
        .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
        .map(|p| p.get_slot())
        .unwrap_or_else(|| String::new());

    battle.add(
        "-weather",
        &[
            Arg::Str("DesolateLand"),
            Arg::String(format!("[from] ability: {}", ability_name)),
            Arg::String(format!("[of] {}", source_ident)),
        ],
    );

    EventResult::Continue
}

/// onImmunity
/// JavaScript source (data/conditions.ts):
/// ```js
/// onImmunity(type, pokemon) {
///     if (pokemon.hasItem('utilityumbrella')) return;
///     if (type === 'frz') return false;
/// }
/// ```
pub fn on_immunity(
    battle: &mut Battle,
    immunity_type: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (pokemon.hasItem('utilityumbrella')) return;
    let has_utility_umbrella = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_item(battle, &["utilityumbrella"])
    };

    if has_utility_umbrella {
        return EventResult::Continue;
    }

    // if (type === 'frz') return false;
    if immunity_type == "frz" {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onFieldResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldResidual() {
///     this.add('-weather', 'DesolateLand', '[upkeep]');
///     this.eachEvent('Weather');
/// }
/// ```
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'DesolateLand', '[upkeep]');
    battle.add("-weather", &[Arg::Str("DesolateLand"), Arg::Str("[upkeep]")]);

    // this.eachEvent('Weather');
    battle.each_event("Weather", None, None);

    EventResult::Continue
}

/// onFieldEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldEnd() {
///     this.add('-weather', 'none');
/// }
/// ```
pub fn on_field_end(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'none');
    battle.add("-weather", &[Arg::Str("none")]);

    EventResult::Continue
}

