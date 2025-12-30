use crate::*;

impl Battle {

    /// Set the currently active move
    /// Equivalent to battle.ts setActiveMove()
    //
    // 	setActiveMove(move?: ActiveMove | null, pokemon?: Pokemon | null, target?: Pokemon | null) {
    // 		this.activeMove = move || null;
    // 		this.activePokemon = pokemon || null;
    // 		this.activeTarget = target || pokemon || null;
    // 	}
    //
    pub fn set_active_move(
        &mut self,
        move_id: Option<ID>,
        pokemon: Option<(usize, usize)>,
        target: Option<(usize, usize)>,
    ) {
        self.active_move = move_id.and_then(|id| self.dex.get_active_move(&id.to_string()));
        self.active_pokemon = pokemon;
        self.active_target = target.or(pokemon);
    }
}
