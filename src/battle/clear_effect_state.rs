use crate::*;

impl Battle {

    /// Clear effect state data
    /// Equivalent to battle.ts clearEffectState()
    //
    // 	clearEffectState(state: EffectState) {
    // 		state.id = '';
    // 		for (const k in state) {
    // 			if (k === 'id' || k === 'target') {
    // 				continue;
    // 			} else if (k === 'effectOrder') {
    // 				state.effectOrder = 0;
    // 			} else {
    // 				delete state[k];
    // 			}
    // 		}
    // 	}
    //
    pub fn clear_effect_state(&self, state: &mut crate::event_system::EffectState) {
        // JS: state.id = '';
        state.id = ID::empty();

        // JS: state.effectOrder = 0;
        state.effect_order = 0;

        // JS: delete state[k] for all other properties
        // Clear the custom data HashMap (TypeScript's dynamic properties)
        state.data.clear();

        // Note: 'id' and 'target' are preserved as per TypeScript logic
        // All other fixed fields (source, duration, etc.) are left as-is
        // since TypeScript would only delete dynamic properties
    }
}
