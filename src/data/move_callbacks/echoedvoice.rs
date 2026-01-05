//! Echoed Voice Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     let bp = move.basePower;
///     if (this.field.pseudoWeather.echoedvoice) {
///         bp = move.basePower * this.field.pseudoWeather.echoedvoice.multiplier;
///     }
///     this.debug(`BP: ${move.basePower}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // let bp = move.basePower;
    let base_power = match &battle.active_move {
        Some(active_move) => active_move.base_power,
        None => return EventResult::Continue,
    };

    eprintln!("[ECHOED_VOICE] base_power_callback called! base_power={}", base_power);

    let mut bp = base_power;

    // if (this.field.pseudoWeather.echoedvoice) {
    //     bp = move.basePower * this.field.pseudoWeather.echoedvoice.multiplier;
    // }
    eprintln!("[ECHOED_VOICE] Checking for echoedvoice pseudoweather. pseudo_weather map size: {}", battle.field.pseudo_weather.len());
    if let Some(echoedvoice_condition) = battle.field.pseudo_weather.get(&ID::from("echoedvoice")) {
        // Get multiplier from effect state
        let multiplier = echoedvoice_condition
            .data
            .get("multiplier")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        eprintln!("[ECHOED_VOICE] Found echoedvoice pseudoweather! multiplier={}", multiplier);
        bp = base_power * multiplier;
        eprintln!("[ECHOED_VOICE] Multiplied base power: {} * {} = {}", base_power, multiplier, bp);
    } else {
        eprintln!("[ECHOED_VOICE] No echoedvoice pseudoweather found!");
    }

    // this.debug(`BP: ${move.basePower}`);
    battle.debug(&format!("BP: {}", base_power));

    // return bp;
    EventResult::Number(bp)
}

/// onTryMove() {
///     this.field.addPseudoWeather('echoedvoice');
/// }
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    eprintln!("[ECHOED_VOICE] on_try_move called! Adding pseudoweather via Battle method");

    // this.field.addPseudoWeather('echoedvoice');
    battle.add_pseudoweather("echoedvoice", Some(source_pos), None);

    eprintln!("[ECHOED_VOICE] Pseudoweather added via Battle.add_pseudoweather");

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onFieldStart() {
    ///     this.effectState.multiplier = 1;
    /// }
    pub fn on_field_start(battle: &mut Battle) -> EventResult {
        eprintln!("[ECHOED_VOICE] on_field_start called!");

        // this.effectState.multiplier = 1;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            eprintln!("[ECHOED_VOICE] Found current_effect_state, setting multiplier to 1");
            effect_state.data.insert(
                "multiplier".to_string(),
                serde_json::Value::Number(1.into()),
            );
        } else {
            eprintln!("[ECHOED_VOICE] WARNING: current_effect_state is None!");
        }

        EventResult::Continue
    }

    /// onFieldRestart() {
    ///     if (this.effectState.duration !== 2) {
    ///         this.effectState.duration = 2;
    ///         if (this.effectState.multiplier < 5) {
    ///             this.effectState.multiplier++;
    ///         }
    ///     }
    /// }
    pub fn on_field_restart(battle: &mut Battle) -> EventResult {
        eprintln!("[ECHOED_VOICE] on_field_restart called!");

        // if (this.effectState.duration !== 2) {
        //     this.effectState.duration = 2;
        //     if (this.effectState.multiplier < 5) {
        //         this.effectState.multiplier++;
        //     }
        // }
        if let Some(ref mut effect_state) = battle.current_effect_state {
            eprintln!("[ECHOED_VOICE] Found current_effect_state");
            let duration = effect_state.duration.unwrap_or(0);
            eprintln!("[ECHOED_VOICE] Current duration: {:?}", effect_state.duration);

            if duration != 2 {
                // this.effectState.duration = 2;
                effect_state.duration = Some(2);
                eprintln!("[ECHOED_VOICE] Set duration to 2");

                // if (this.effectState.multiplier < 5) {
                //     this.effectState.multiplier++;
                // }
                let current_multiplier = effect_state
                    .data
                    .get("multiplier")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(1);
                eprintln!("[ECHOED_VOICE] Current multiplier: {}", current_multiplier);

                if current_multiplier < 5 {
                    effect_state.data.insert(
                        "multiplier".to_string(),
                        serde_json::Value::Number((current_multiplier + 1).into()),
                    );
                    eprintln!("[ECHOED_VOICE] Incremented multiplier to {}", current_multiplier + 1);
                } else {
                    eprintln!("[ECHOED_VOICE] Multiplier already at max (5)");
                }
            } else {
                eprintln!("[ECHOED_VOICE] Duration is already 2, not incrementing");
            }
        } else {
            eprintln!("[ECHOED_VOICE] WARNING: current_effect_state is None!");
        }

        EventResult::Continue
    }
}
