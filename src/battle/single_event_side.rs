//! Battle::single_event_side - Fire single event on a Side
//!
//! JavaScript equivalent: singleEvent with Side as target parameter
//!
//! This method provides side-level event firing, which is used for side condition
//! callbacks like onSideStart, onSideEnd, onSideRestart.

use crate::*;
use crate::battle::EventInfo;
use crate::battle::Effect;

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
    /// - source_effect: Optional source effect
    ///
    /// Returns: EventResult from the callback
    pub fn single_event_side(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        side_idx: usize,
        source: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
        target_pos: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        // JavaScript: if (this.eventDepth >= 8) throw Error
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // JavaScript: if (this.log.length - this.sentLogPos > 1000) throw Error
        if self.log.len() - self.sent_log_pos > 1000 {
            self.add("message", &["LINE LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.event {
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
            if self.suppressing_weather() {
                self.debug(&format!("{} handler suppressed by Air Lock", event_id));
                return EventResult::Continue;
            }
        }

        // Check if callback exists
        if !self.has_side_condition_callback(effect_id, event_id) {
            return EventResult::Continue;
        }

        // Save parent state
        let parent_effect = self.effect.clone();
        let parent_event = self.event.clone();

        // Look up side condition name (usually from moves like Reflect, Light Screen)
        let condition_name = self.dex.conditions().get_by_id(&effect_id)
            .and_then(|c| c.name.clone())
            .or_else(|| self.dex.moves().get(effect_id.as_str()).map(|m| m.name.clone()))
            .unwrap_or_else(|| effect_id.to_string());

        // Set current event context with SideCondition effect type
        self.effect = Some(crate::Effect {
            id: effect_id.clone(),
            name: condition_name.into(),
            effect_type: crate::battle::EffectType::SideCondition,
            effect_holder: None,
            side_index: Some(side_idx),
            prankster_boosted: false,
        });

        // source_effect is already an Effect, so just clone it
        let source_effect_obj = source_effect.cloned();

        let mut event_info = EventInfo::new(event_id);
        event_info.target = None; // Side events don't have a Pokemon target
        event_info.source = source;
        event_info.effect = source_effect_obj;
        self.event = Some(event_info);

        self.event_depth += 1;

        // Dispatch to the side condition callback
        let result = self.dispatch_single_event_side(event_id, effect_id, side_idx, source, target_pos);

        // Restore parent state
        self.event_depth -= 1;
        self.effect = parent_effect;
        self.event = parent_event;

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
        target_pos: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::data::move_callbacks;

        // Dispatch to the appropriate side condition callback based on event type
        match event_id {
            "SideStart" => move_callbacks::dispatch_side_condition_on_side_start_by_id(
                self,
                condition_id.as_str(),
                source,
            ),
            "SideRestart" => move_callbacks::dispatch_side_condition_on_side_restart_by_id(
                self,
                condition_id.as_str(),
            ),
            "SideEnd" => move_callbacks::dispatch_side_condition_on_side_end_by_id(
                self,
                condition_id.as_str(),
            ),
            "Residual" => move_callbacks::dispatch_side_condition_on_residual_by_id(
                self,
                condition_id.as_str(),
                target_pos,
            ),
            _ => crate::event::EventResult::Continue,
        }
    }
}
