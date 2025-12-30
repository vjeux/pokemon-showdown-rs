use crate::side::*;

impl Side {

    /// Emit choice error to client
    /// Equivalent to side.ts emitChoiceError()
    //
    // 	emitChoiceError(
    // 		message: string, update?: { pokemon: Pokemon, update: (req: PokemonMoveRequestData) => boolean | void }
    // 	) {
    // 		this.choice.error = message;
    // 		const updated = update ? this.updateRequestForPokemon(update.pokemon, update.update) : null;
    // 		const type = `[${updated ? 'Unavailable' : 'Invalid'} choice]`;
    // 		this.battle.send('sideupdate', `${this.id}\n|error|${type} ${message}`);
    // 		if (updated) this.emitRequest(this.activeRequest!, true);
    // 		if (this.battle.strictChoices) throw new Error(`${type} ${message}`);
    // 		return false;
    // 	}
    //
    pub fn emit_choice_error(&self, message: &str) -> bool {
        // In the full implementation, this would send to client
        // For now, just return false to indicate error was emitted
        let _ = message;
        false
    }
}
