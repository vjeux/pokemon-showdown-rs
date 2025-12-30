use crate::side::*;
use crate::*;

impl Side {

    /// Get picked team size for team preview
    // TypeScript source:
    // /**
    // 	 * The number of pokemon you must choose in Team Preview.
    // 	 *
    // 	 * Note that PS doesn't support choosing fewer than this number of pokemon.
    // 	 * In the games, it is sometimes possible to bring fewer than this, but
    // 	 * since that's nearly always a mistake, we haven't gotten around to
    // 	 * supporting it.
    // 	 */
    // 	pickedTeamSize() {
    // 		return Math.min(this.pokemon.length, this.battle.ruleTable.pickedTeamSize || Infinity);
    // 	}
    //
    pub fn picked_team_size(&self, rule_table_size: Option<usize>) -> usize {
        rule_table_size
            .unwrap_or(self.pokemon.len())
            .min(self.pokemon.len())
    }
}
