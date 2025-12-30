use crate::*;

impl Battle {

    /// Check if a target is valid for a move
    /// Equivalent to battle.ts validTarget()
    //
    // 	validTarget(target: Pokemon, source: Pokemon, targetType: string) {
    // 		return this.validTargetLoc(source.getLocOf(target), source, targetType);
    // 	}
    //
    pub fn valid_target(
        &self,
        target: (usize, usize),
        source: (usize, usize),
        target_type: &str,
    ) -> bool {
        // JS: return this.validTargetLoc(source.getLocOf(target), source, targetType);
        let target_loc = if let Some(source_pokemon) = self.sides.get(source.0).and_then(|s| s.pokemon.get(source.1)) {
            source_pokemon.get_loc_of(target.0, target.1, self.active_per_half) as i32
        } else {
            0
        };
        self.valid_target_loc(target_loc, source, target_type)
    }
}
