//! Deltastream Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;

/// onEffectiveness
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEffectivenessPriority: -1,
/// onEffectiveness(typeMod, target, type, move) {
///     if (move && move.effectType === 'Move' && move.category !== 'Status' && type === 'Flying' && typeMod > 0) {
///         this.add('-fieldactivate', 'Delta Stream');
///         return 0;
///     }
/// }
/// ```
pub fn on_effectiveness(
    battle: &mut Battle,
    type_mod: i32,
    target_type: &str,
    _pokemon_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    // Get move category from active_move
    // if (move && move.effectType === 'Move' && move.category !== 'Status' && type === 'Flying' && typeMod > 0)
    let category = match &battle.active_move {
        Some(m) => &m.category,
        None => return EventResult::Continue,
    };

    if category != "Status" && target_type == "Flying" && type_mod > 0 {
        // this.add('-fieldactivate', 'Delta Stream');
        battle.add("-fieldactivate", &[Arg::Str("Delta Stream")]);

        // return 0;
        return EventResult::Number(0);
    }

    EventResult::Continue
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(field, source, effect) {
///     this.add('-weather', 'DeltaStream', '[from] ability: ' + effect.name, `[of] ${source}`);
/// }
/// ```
pub fn on_field_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'DeltaStream', '[from] ability: ' + effect.name, `[of] ${source}`);
    let ability_name = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .map(|ab| ab.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    let source_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-weather",
        &[
            Arg::Str("DeltaStream"),
            Arg::String(format!("[from] ability: {}", ability_name)),
            Arg::String(format!("[of] {}", source_ident)),
        ],
    );

    EventResult::Continue
}

/// onFieldResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldResidualOrder: 1,
/// onFieldResidual() {
///     this.add('-weather', 'DeltaStream', '[upkeep]');
///     this.eachEvent('Weather');
/// }
/// ```
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'DeltaStream', '[upkeep]');
    battle.add("-weather", &[Arg::Str("DeltaStream"), Arg::Str("[upkeep]")]);

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

