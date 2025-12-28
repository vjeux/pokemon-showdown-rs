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
pub fn base_power_callback(battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // let bp = move.basePower;
    let base_power = match &battle.active_move {
        Some(active_move) => active_move.base_power,
        None => return EventResult::Continue,
    };

    let mut bp = base_power;

    // if (this.field.pseudoWeather.echoedvoice) {
    //     bp = move.basePower * this.field.pseudoWeather.echoedvoice.multiplier;
    // }
    if let Some(echoedvoice_condition) = battle.field.pseudo_weather.get(&ID::from("echoedvoice")) {
        // Get multiplier from effect state
        let multiplier = echoedvoice_condition.data.get("multiplier")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        bp = base_power * multiplier;
    }

    // this.debug(`BP: ${move.basePower}`);
    battle.debug(&format!("BP: {}", base_power));

    // return bp;
    EventResult::Number(bp)
}

/// onTryMove() {
///     this.field.addPseudoWeather('echoedvoice');
/// }
pub fn on_try_move(battle: &mut Battle, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // this.field.addPseudoWeather('echoedvoice');
    battle.field.add_pseudo_weather(ID::from("echoedvoice"), None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onFieldStart() {
    ///     this.effectState.multiplier = 1;
    /// }
    pub fn on_field_start(battle: &mut Battle) -> EventResult {
        // this.effectState.multiplier = 1;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert("multiplier".to_string(), serde_json::Value::Number(1.into()));
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
        // if (this.effectState.duration !== 2) {
        //     this.effectState.duration = 2;
        //     if (this.effectState.multiplier < 5) {
        //         this.effectState.multiplier++;
        //     }
        // }
        if let Some(ref mut effect_state) = battle.current_effect_state {
            let duration = effect_state.duration.unwrap_or(0);

            if duration != 2 {
                // this.effectState.duration = 2;
                effect_state.duration = Some(2);

                // if (this.effectState.multiplier < 5) {
                //     this.effectState.multiplier++;
                // }
                let current_multiplier = effect_state.data.get("multiplier")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(1);

                if current_multiplier < 5 {
                    effect_state.data.insert("multiplier".to_string(), serde_json::Value::Number((current_multiplier + 1).into()));
                }
            }
        }

        EventResult::Continue
    }
}
