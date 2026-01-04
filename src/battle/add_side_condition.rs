//! Battle::add_side_condition - Add side condition with source tracking
//!
//! JavaScript equivalent: Side.addSideCondition(status, source, sourceEffect)
//!
//! This method provides Battle-level access to add side conditions with proper
//! source tracking, which is needed for moves like Pursuit that track multiple sources.

use crate::*;
use crate::event::EventResult;
use crate::event_system::EffectState;

impl Battle {
    /// Add a side condition to a specific side with source tracking
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// side.addSideCondition(status: string | Condition, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null): boolean
    /// ```
    ///
    /// Parameters:
    /// - side_idx: Index of the side to add the condition to
    /// - condition_id: ID of the side condition
    /// - source_pos: Optional source Pokemon position (side_idx, pokemon_idx)
    /// - source_effect: Optional source effect ID
    ///
    /// Returns: true if condition was added, false if it already exists
    pub fn add_side_condition(
        &mut self,
        side_idx: usize,
        condition_id: ID,
        source_pos: Option<(usize, usize)>,
        _source_effect: Option<&ID>,
    ) -> bool {
        // Check if side exists
        if side_idx >= self.sides.len() {
            return false;
        }

        // Check if condition already exists
        if self.sides[side_idx].side_conditions.contains_key(&condition_id) {
            // if (!(status as any).onSideRestart) return false;
            // return this.battle.singleEvent('SideRestart', status, this.sideConditions[status.id], this, source, sourceEffect);

            // Check if condition has onSideRestart callback
            if self.has_callback(&condition_id, "SideRestart") {
                // Fire SideRestart event
                let result = self.single_event_side(
                    "SideRestart",
                    &condition_id,
                    side_idx,
                    source_pos,
                    _source_effect,
                );

                // Convert EventResult to bool
                return match result {
                    crate::event::EventResult::Boolean(b) => b,
                    crate::event::EventResult::Number(n) => n != 0,
                    _ => false,
                };
            }

            return false;
        }

        // Create effect state with source tracking
        let mut state = EffectState::new(condition_id.clone());
        state.source = source_pos;

        // If source exists, add source_slot field
        if let Some(source) = source_pos {
            if let Some(source_pokemon) = self.pokemon_at(source.0, source.1) {
                state.source_slot = Some(source_pokemon.position);
            }
        }

        // Get condition data for duration
        if let Some(condition_data) = self.dex.conditions().get_by_id(&condition_id) {
            state.duration = condition_data.duration;

            // if (status.durationCallback) {
            //     this.sideConditions[status.id].duration =
            //         status.durationCallback.call(this.battle, this.active[0], source, sourceEffect);
            // }
            // Call durationCallback if exists
            let target_pos = if side_idx < self.sides.len() && !self.sides[side_idx].active.is_empty() {
                Some((side_idx, 0)) // side.active[0]
            } else {
                None
            };

            let result = self.call_duration_callback(
                &condition_id,
                target_pos,
                source_pos,
                _source_effect.map(|id| id.as_str()),
            );

            if let crate::event::EventResult::Number(duration) = result {
                state.duration = Some(duration);
            }
        }

        // Add the condition
        self.sides[side_idx].side_conditions.insert(condition_id.clone(), state);

        // if (!this.battle.singleEvent('SideStart', status, this.sideConditions[status.id], this, source, sourceEffect)) {
        //     delete this.sideConditions[status.id];
        //     return false;
        // }
        if self.has_callback(&condition_id, "SideStart") {
            let result = self.single_event_side(
                "SideStart",
                &condition_id,
                side_idx,
                source_pos,
                _source_effect,
            );

            // Check if event returned false (failure)
            let success = match result {
                crate::event::EventResult::Boolean(false) => false,
                crate::event::EventResult::Number(0) => false,
                crate::event::EventResult::NotFail => false,
                _ => true,
            };

            if !success {
                // Remove the condition we just added
                self.sides[side_idx].side_conditions.remove(&condition_id);
                return false;
            }
        }

        // this.battle.runEvent('SideConditionStart', this, source, status);
        self.run_event_side("SideConditionStart", side_idx, source_pos, Some(&condition_id));

        true
    }

    /// Get mutable reference to side condition data HashMap
    ///
    /// This allows modifying the data field of a side condition's EffectState,
    /// which is used by moves like Pursuit to track multiple sources.
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// const data = side.getSideConditionData('pursuit');
    /// if (!data.sources) data.sources = [];
    /// data.sources.push(pokemon);
    /// ```
    pub fn get_side_condition_data_mut(
        &mut self,
        side_idx: usize,
        condition_id: &ID,
    ) -> Option<&mut std::collections::HashMap<String, serde_json::Value>> {
        if side_idx >= self.sides.len() {
            return None;
        }

        self.sides[side_idx]
            .side_conditions
            .get_mut(condition_id)
            .map(|state| &mut state.data)
    }
}
