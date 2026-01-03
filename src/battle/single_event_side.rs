//! Battle::single_event_side - Fire single event on a Side
//!
//! JavaScript equivalent: singleEvent with Side as target parameter
//!
//! This method provides side-level event firing, which is used for side condition
//! callbacks like onSideStart, onSideEnd, onSideRestart.

use crate::*;
use crate::battle::EventInfo;

impl Battle {
    /// Single event targeting a Side (not a Pokemon)
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// battle.singleEvent('SideStart', status, sideConditions[status.id], this, source, sourceEffect)
    /// ```
    ///
    /// where `this` is the Side object.
    ///
    /// Parameters:
    /// - event_id: Event name (e.g., "SideStart", "SideEnd", "SideRestart")
    /// - effect_id: ID of the side condition
    /// - side_idx: Index of the side being targeted
    /// - source: Optional source Pokemon position
    /// - source_effect: Optional source effect ID
    ///
    /// Returns: EventResult from the callback
    pub fn single_event_side(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        side_idx: usize,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        // JavaScript: if (this.eventDepth >= 8) throw Error
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.current_event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // JavaScript: if (this.log.length - this.sentLogPos > 1000) throw Error
        if self.log.len() - self.sent_log_pos > 1000 {
            self.add("message", &["LINE LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.current_event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // Check if side condition exists
        if side_idx >= self.sides.len() {
            return EventResult::Continue;
        }

        if !self.sides[side_idx].side_conditions.contains_key(effect_id) {
            return EventResult::Continue;
        }

        // Weather suppression check (for weather-based side conditions)
        let effect_type = self.get_effect_type(effect_id);
        if effect_type == "Weather"
            && event_id != "FieldStart"
            && event_id != "FieldResidual"
            && event_id != "FieldEnd"
        {
            if self.field.suppressing_weather() {
                self.debug(&format!("{} handler suppressed by Air Lock", event_id));
                return EventResult::Continue;
            }
        }

        // Check if callback exists
        if !self.has_callback(effect_id, event_id) {
            return EventResult::Continue;
        }

        // Save parent state
        let parent_effect = self.current_effect.clone();
        let parent_effect_state = self.current_effect_state.clone();
        let parent_event = self.current_event.clone();

        // Set current event context
        self.current_effect = Some(effect_id.clone());

        // Note: Side conditions use dex_data::EffectState, but current_effect_state expects event_system::EffectState
        // For now, we set it to None since side condition callbacks don't access effectState
        // TODO: Consider unifying EffectState types or converting between them
        self.current_effect_state = None;

        let mut event_info = EventInfo::new(event_id);
        event_info.target = None; // Side events don't have a Pokemon target
        event_info.source = source;
        event_info.effect = source_effect.cloned();
        self.current_event = Some(event_info);

        self.event_depth += 1;

        // Dispatch to the side condition callback
        let result = self.dispatch_single_event_side(event_id, effect_id, side_idx, source);

        // Restore parent state
        self.event_depth -= 1;
        self.current_effect = parent_effect;
        self.current_effect_state = parent_effect_state;
        self.current_event = parent_event;

        result
    }

    /// Dispatch side event to the appropriate callback
    /// Helper method that routes side condition events to their callbacks
    fn dispatch_single_event_side(
        &mut self,
        event_id: &str,
        condition_id: &ID,
        _side_idx: usize,
        source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        // Dispatch to side condition callbacks based on condition_id
        // For now, we'll implement a simple dispatcher that calls the appropriate callback
        // This matches the pattern used in handle_side_condition_event.rs

        match condition_id.as_str() {
            "auroraveil" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::auroraveil::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::auroraveil::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "craftyshield" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::craftyshield::condition::on_side_start(self, None, source),
                    _ => EventResult::Continue,
                }
            }
            "firepledge" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::firepledge::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::firepledge::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "gmaxcannonade" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::gmaxcannonade::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::gmaxcannonade::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "gmaxsteelsurge" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::gmaxsteelsurge::condition::on_side_start(self),
                    _ => EventResult::Continue,
                }
            }
            "gmaxvinelash" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::gmaxvinelash::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::gmaxvinelash::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "gmaxvolcalith" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::gmaxvolcalith::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::gmaxvolcalith::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "gmaxwildfire" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::gmaxwildfire::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::gmaxwildfire::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "grasspledge" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::grasspledge::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::grasspledge::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "lightscreen" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::lightscreen::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::lightscreen::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "luckychant" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::luckychant::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::luckychant::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "matblock" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::matblock::condition::on_side_start(self, None, source),
                    _ => EventResult::Continue,
                }
            }
            "mist" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::mist::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::mist::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "quickguard" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::quickguard::condition::on_side_start(self, None, source),
                    _ => EventResult::Continue,
                }
            }
            "reflect" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::reflect::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::reflect::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "safeguard" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::safeguard::condition::on_side_start(self, source),
                    "SideEnd" => crate::data::move_callbacks::safeguard::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "spikes" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::spikes::condition::on_side_start(self),
                    _ => EventResult::Continue,
                }
            }
            "stealthrock" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::stealthrock::condition::on_side_start(self),
                    _ => EventResult::Continue,
                }
            }
            "stickyweb" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::stickyweb::condition::on_side_start(self),
                    _ => EventResult::Continue,
                }
            }
            "tailwind" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::tailwind::condition::on_side_start(self, source),
                    "SideEnd" => crate::data::move_callbacks::tailwind::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "toxicspikes" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::toxicspikes::condition::on_side_start(self),
                    _ => EventResult::Continue,
                }
            }
            "waterpledge" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::waterpledge::condition::on_side_start(self),
                    "SideEnd" => crate::data::move_callbacks::waterpledge::condition::on_side_end(self),
                    _ => EventResult::Continue,
                }
            }
            "wideguard" => {
                match event_id {
                    "SideStart" => crate::data::move_callbacks::wideguard::condition::on_side_start(self, None, source),
                    _ => EventResult::Continue,
                }
            }
            // Add side conditions here as they're implemented
            // For now, we return Continue to allow the event system to work
            // without requiring all side conditions to be implemented
            _ => {
                // No callback found - this is fine, not all side conditions have all events
                EventResult::Continue
            }
        }
    }
}
