use crate::*;
use crate::Pokemon;

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
        // ✅ NOW USES: Pokemon::faint() which handles both marking as fainted AND adding to faint_queue
        let effect_id = effect.map(ID::new);
        Pokemon::faint(self, target, source, effect_id.as_ref());

        // JS: In JavaScript, pokemon.faint() also sets fainted=true immediately for some reason
        // In Rust, we need to set it here as well for compatibility with JavaScript behavior
        if let Some(pokemon) = self.pokemon_at_mut(target.0, target.1) {
            pokemon.fainted = true;
        }
    }
}
