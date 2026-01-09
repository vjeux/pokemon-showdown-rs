use crate::*;

impl Battle {

    /// Get move category, defaulting to "Physical" if move not found
    /// Equivalent to battle.ts getCategory() (battle.ts:2350-2352)
    ///
    //
    // 	getCategory(move: string | Move) {
    // 		return this.dex.moves.get(move).category || 'Physical';
    // 	}
    //
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn get_category(&self, move_id: &ID) -> String {
        if let Some(move_def) = self.dex.moves().get(move_id.as_str()) {
            return move_def.category.clone();
        }
        "Physical".to_string()
    }
}
