use crate::*;

impl Pokemon {

    /// Remove linked volatiles
    /// Equivalent to removeLinkedVolatiles in pokemon.ts
    //
    // 	removeLinkedVolatiles(linkedStatus: string | Effect, linkedPokemon: Pokemon[]) {
    // 		linkedStatus = linkedStatus.toString();
    // 		for (const linkedPoke of linkedPokemon) {
    // 			const volatileData = linkedPoke.volatiles[linkedStatus];
    // 			if (!volatileData) continue;
    // 			volatileData.linkedPokemon.splice(volatileData.linkedPokemon.indexOf(this), 1);
    // 			if (volatileData.linkedPokemon.length === 0) {
    // 				linkedPoke.removeVolatile(linkedStatus);
    // 			}
    // 		}
    // 	}
    //
    pub fn remove_linked_volatiles(&mut self, linked_status: &ID) {
        // Remove volatiles that are linked to this one
        // For example, Leech Seed removes when source switches
        let to_remove: Vec<ID> = self
            .volatiles
            .keys()
            .filter(|k| k.as_str().starts_with(linked_status.as_str()))
            .cloned()
            .collect();

        for id in to_remove {
            self.volatiles.remove(&id);
        }
    }
}
