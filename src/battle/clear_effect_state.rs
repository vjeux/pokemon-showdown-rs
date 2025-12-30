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
    pub fn clear_effect_state(&mut self, target: (usize, usize), effect_id: &ID) {
        if let Some(side) = self.sides.get_mut(target.0) {
            if let Some(pokemon) = side.pokemon.get_mut(target.1) {
                pokemon.volatiles.remove(effect_id);
            }
        }
    }
}
