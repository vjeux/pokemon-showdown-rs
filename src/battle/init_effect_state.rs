use crate::*;
use crate::dex_data::EffectState;

impl Battle {

    /// Initialize an effect state
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
    pub fn init_effect_state(&mut self, id: ID) -> EffectState {
        let mut state = EffectState::new(id);
        state.effect_order = self.next_effect_order();
        state
    }
}
