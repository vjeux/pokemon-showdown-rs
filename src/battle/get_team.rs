use crate::*;

impl Battle {

    /// Get team based on player options
    /// Equivalent to battle.ts getTeam()
    //
    // TODO: WRONG IMPLEMENTATION - Completely different signature and purpose
    // TypeScript version (battle.ts:3164-3180, 17 lines):
    // - Takes PlayerOptions (with optional team string, seed)
    // - Unpacks team string using Teams.unpack()
    // - Generates random team if no team provided (using TeamGenerator)
    // - Returns PokemonSet[] (team of sets)
    //
    // Rust version:
    // - Takes side_idx (number)
    // - Returns &[Pokemon] (existing Pokemon instances)
    // - No team generation, unpacking, or player options handling
    //
    // This is a fundamental mismatch. The Rust version appears to be a simple getter
    // for a side's Pokemon, while TypeScript handles team creation/generation.
    // The Rust version should either:
    // 1. Be deleted if not needed (no equivalent in JS for this specific functionality)
    // 2. Be renamed to something like get_side_pokemon() to avoid confusion
    // 3. Be reimplemented to match TypeScript's signature and behavior
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
