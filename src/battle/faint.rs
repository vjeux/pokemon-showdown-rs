use crate::*;
use crate::battle::FaintData;

impl Battle {

    /// Faint a Pokemon
    /// Equivalent to battle.ts faint()
    //
    // 	faint(pokemon: Pokemon, source?: Pokemon, effect?: Effect) {
    // 		pokemon.faint(source, effect);
    // 	}
    //
    pub fn faint(
        &mut self,
        target: (usize, usize),
        source: Option<(usize, usize)>,
        effect: Option<&str>,
    ) {
        // JS: pokemon.faint(source, effect)
        // JS: battle.faintQueue.push({target: pokemon, source, effect})

        let (target_side, target_idx) = target;
        if let Some(side) = self.sides.get_mut(target_side) {
            if let Some(pokemon) = side.pokemon.get_mut(target_idx) {
                pokemon.faint();

                // In JavaScript, pokemon.faint() sets fainted=true immediately
                // In Rust, we need to set it here as well for compatibility
                pokemon.fainted = true;

                // JS: this.faintQueue.push({target, source, effect});
                let effect_id = effect.map(ID::new);
                self.faint_queue.push(FaintData {
                    target,
                    source,
                    effect: effect_id,
                });
            }
        }
    }
}
