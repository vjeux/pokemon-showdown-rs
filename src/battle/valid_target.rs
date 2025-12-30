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
        let target_loc = self.get_loc_of(source, target);
        self.valid_target_loc(target_loc, source, target_type)
    }
}
