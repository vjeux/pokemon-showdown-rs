use crate::side::*;

impl Side {

    /// Destroy side (cleanup)
    //
    // 	destroy() {
    // 		// deallocate ourself
    //
    // 		// deallocate children and get rid of references to them
    // 		for (const pokemon of this.pokemon) {
    // 			if (pokemon) pokemon.destroy();
    // 		}
    //
    // 		for (const action of this.choice.actions) {
    // 			delete action.side;
    // 			delete action.pokemon;
    // 			delete action.target;
    // 		}
    // 		this.choice.actions = [];
    //
    // 		// get rid of some possibly-circular references
    // 		this.pokemon = [];
    // 		this.active = [];
    // 		this.foe = null!;
    // 		(this as any).battle = null!;
    // 	}
    //
    pub fn destroy(&mut self) {
        self.pokemon.clear();
        self.active.clear();
        self.choice.actions.clear();
        self.side_conditions.clear();
        for slot_cond in &mut self.slot_conditions {
            slot_cond.clear();
        }
    }
}
