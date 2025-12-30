use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Clear ability (set to empty)
    //
    // 	clearAbility() {
    // 		return this.setAbility('');
    // 	}
    //
    pub fn clear_ability(&mut self) -> ID {
        let old = self.ability.clone();
        self.ability = ID::empty();
        self.ability_state = EffectState::new(ID::empty());
        old
    }
}
