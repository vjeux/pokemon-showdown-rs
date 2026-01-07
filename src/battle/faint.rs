use crate::*;
use crate::Pokemon;
use crate::battle::Effect;

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
        let effect_obj = effect.map(|e| Effect::condition(ID::new(e)));
        Pokemon::faint(self, target, source, effect_obj.as_ref());

        // NOTE: pokemon.fainted is NOT set here!
        // In JavaScript, pokemon.faint() only sets:
        //   - hp = 0
        //   - switchFlag = false
        //   - faintQueued = true
        // The fainted = true flag is set LATER in faintMessages() when the faint queue is processed.
        // Setting it here would cause faintMessages() to skip processing the faint!
    }
}
