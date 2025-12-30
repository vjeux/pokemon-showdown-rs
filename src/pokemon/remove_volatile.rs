use crate::*;

impl Pokemon {

    /// Remove a volatile condition
    //
    // 	removeVolatile(status: string | Effect) {
    // 		if (!this.hp) return false;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.volatiles[status.id]) return false;
    // 		const { linkedPokemon, linkedStatus } = this.volatiles[status.id];
    // 		this.battle.singleEvent('End', status, this.volatiles[status.id], this);
    // 		delete this.volatiles[status.id];
    // 		if (linkedPokemon) {
    // 			this.removeLinkedVolatiles(linkedStatus, linkedPokemon);
    // 		}
    // 		return true;
    // 	}
    //
    pub fn remove_volatile(&mut self, id: &ID) -> bool {
        self.volatiles.remove(id).is_some()
    }
}
