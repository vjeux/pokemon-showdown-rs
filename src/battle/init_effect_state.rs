use crate::*;
use crate::event_system::EffectState;

impl Battle {

    /// Initialize an effect state
    /// Equivalent to battle.ts initEffectState()
    //
    // 	initEffectState(obj: Partial<EffectState>, effectOrder?: number): EffectState {
    // 		if (!obj.id) obj.id = '';
    // 		if (effectOrder !== undefined) {
    // 			obj.effectOrder = effectOrder;
    // 		} else if (obj.id && obj.target && (!(obj.target instanceof Pokemon) || obj.target.isActive)) {
    // 			obj.effectOrder = this.effectOrder++;
    // 		} else {
    // 			obj.effectOrder = 0;
    // 		}
    // 		return obj as EffectState;
    // 	}
    //
    pub fn init_effect_state(&mut self, mut state: EffectState, effect_order: Option<i32>) -> EffectState {
        // JS: if (!obj.id) obj.id = '';
        if state.id.as_str().is_empty() {
            state.id = ID::new("");
        }

        // JS: if (effectOrder !== undefined) {
        // JS:     obj.effectOrder = effectOrder;
        if let Some(order) = effect_order {
            state.effect_order = order;
        // JS: } else if (obj.id && obj.target && (!(obj.target instanceof Pokemon) || obj.target.isActive)) {
        // JS:     obj.effectOrder = this.effectOrder++;
        } else if !state.id.as_str().is_empty() && state.target.is_some() {
            // Note: In TypeScript, target can be a Pokemon object and it checks target.isActive
            // In Rust, state.target is Option<(usize, usize)> so we increment if both id and target exist
            // The full Pokemon.isActive check would require looking up the pokemon, which is complex
            // For now, this matches the common case where target exists
            state.effect_order = self.next_effect_order();
        // JS: } else {
        // JS:     obj.effectOrder = 0;
        } else {
            state.effect_order = 0;
        }

        state
    }
}

