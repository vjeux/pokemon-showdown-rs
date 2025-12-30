use crate::*;

impl Pokemon {

    /// Destroy/cleanup Pokemon
    /// Equivalent to destroy in pokemon.ts
    //
    // 	destroy() {
    // 		// deallocate ourself
    // 		// get rid of some possibly-circular references
    // 		(this as any).battle = null!;
    // 		(this as any).side = null!;
    // 	}
    //
    pub fn destroy(&mut self) {
        self.volatiles.clear();
        self.move_slots.clear();
        self.base_move_slots.clear();
    }
}
