//! Battle::add_side_condition - Add side condition with source tracking
//!
//! JavaScript equivalent: Side.addSideCondition(status, source, sourceEffect)
//!
//! This method provides Battle-level access to add side conditions with proper
//! source tracking, which is needed for moves like Pursuit that track multiple sources.

use crate::*;

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
            // TODO: Handle onSideRestart event
            return false;
        }

        // Create effect state with source tracking
        let mut state = dex_data::EffectState::new(condition_id.clone());
        state.source = source_pos;

        // If source exists, add source_slot field
        if let Some(source) = source_pos {
            if let Some(source_pokemon) = self.pokemon_at(source.0, source.1) {
                state.source_slot = Some(source_pokemon.get_slot());
            }
        }

        // Get condition data for duration
        if let Some(condition_data) = self.dex.conditions.get(&condition_id) {
            state.duration = condition_data.duration;
            // TODO: Call durationCallback if exists
        }

        // Add the condition
        self.sides[side_idx].side_conditions.insert(condition_id, state);

        // TODO: Fire SideStart event via battle.single_event()
        // TODO: Fire SideConditionStart via battle.run_event()

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
