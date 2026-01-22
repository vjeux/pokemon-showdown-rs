//! Battle::call_duration_callback - Call durationCallback for side conditions
//!
//! JavaScript equivalent: status.durationCallback.call(this.battle, target, source, sourceEffect)
//!
//! This method calls the durationCallback for a side condition if it exists,
//! allowing conditions to calculate their duration dynamically (e.g., Light Clay extending screens).

use crate::*;
use crate::event::EventResult;
use crate::battle::Effect;

impl Battle {
    /// Call durationCallback for a side condition
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// if (status.durationCallback) {
    ///     this.sideConditions[status.id].duration =
    ///         status.durationCallback.call(this.battle, this.active[0], source, sourceEffect);
    /// }
    /// ```
    ///
    /// Parameters:
    /// - condition_id: ID of the side condition
    /// - target_pos: Target Pokemon position (usually side's active[0])
    /// - source_pos: Source Pokemon position
    /// - source_effect: Source effect
    ///
    /// Returns: EventResult::Number with duration, or EventResult::Continue if no callback
    pub fn call_duration_callback(
        &mut self,
        condition_id: &ID,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
    ) -> EventResult {
        // Dispatch to condition-specific duration callback
        match condition_id.as_str() {
            "auroraveil" => {
                crate::data::move_callbacks::auroraveil::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "electricterrain" => {
                crate::data::move_callbacks::electricterrain::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "grassyterrain" => {
                crate::data::move_callbacks::grassyterrain::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "gravity" => {
                crate::data::move_callbacks::gravity::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "healblock" => {
                crate::data::move_callbacks::healblock::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "lightscreen" => {
                crate::data::move_callbacks::lightscreen::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "magicroom" => {
                crate::data::move_callbacks::magicroom::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "mistyterrain" => {
                crate::data::move_callbacks::mistyterrain::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "psychicterrain" => {
                crate::data::move_callbacks::psychicterrain::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "reflect" => {
                crate::data::move_callbacks::reflect::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "safeguard" => {
                crate::data::move_callbacks::safeguard::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "tailwind" => {
                crate::data::move_callbacks::tailwind::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "trickroom" => {
                crate::data::move_callbacks::trickroom::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            "wonderroom" => {
                crate::data::move_callbacks::wonderroom::condition::duration_callback(
                    self,
                    target_pos,
                    source_pos,
                    source_effect,
                )
            }
            // Weather duration callbacks
            "hail" => {
                crate::data::condition_callbacks::hail::duration_callback(
                    self,
                    target_pos.unwrap_or((0, 0)),
                    source_pos,
                    source_effect,
                )
            }
            "raindance" => {
                crate::data::condition_callbacks::raindance::duration_callback(
                    self,
                    target_pos.unwrap_or((0, 0)),
                    source_pos,
                    source_effect,
                )
            }
            "sandstorm" => {
                crate::data::condition_callbacks::sandstorm::duration_callback(
                    self,
                    target_pos.unwrap_or((0, 0)),
                    source_pos,
                    source_effect,
                )
            }
            "snowscape" => {
                crate::data::condition_callbacks::snowscape::duration_callback(
                    self,
                    target_pos.unwrap_or((0, 0)),
                    source_pos,
                    source_effect,
                )
            }
            "sunnyday" => {
                crate::data::condition_callbacks::sunnyday::duration_callback(
                    self,
                    target_pos.unwrap_or((0, 0)),
                    source_pos,
                    source_effect,
                )
            }
            _ => EventResult::Continue,
        }
    }
}
