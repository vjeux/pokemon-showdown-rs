use crate::*;

impl Battle {

    /// Get team based on player options
    /// Equivalent to battle.ts getTeam()
    //
    // 	// players
    //
    // 	getTeam(options: PlayerOptions): PokemonSet[] {
    // 		let team = options.team;
    // 		if (typeof team === 'string') team = Teams.unpack(team);
    // 		if (team) return team;
    //
    // 		if (!options.seed) {
    // 			options.seed = PRNG.generateSeed();
    // 		}
    //
    // 		if (!this.teamGenerator) {
    // 			this.teamGenerator = Teams.getGenerator(this.format, options.seed);
    // 		} else {
    // 			this.teamGenerator.setSeed(options.seed);
    // 		}
    //
    // 		team = this.teamGenerator.getTeam(options);
    // 		return team as PokemonSet[];
    // 	}
    //
    pub fn get_team(&self, side_idx: usize) -> Option<&[crate::pokemon::Pokemon]> {
        self.sides.get(side_idx).map(|s| s.pokemon.as_slice())
    }
}
