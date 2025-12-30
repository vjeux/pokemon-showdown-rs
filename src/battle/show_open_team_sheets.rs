use crate::*;

impl Battle {

    /// Show open team sheets to players
    /// Equivalent to battle.ts showOpenTeamSheets()
    //
    // TODO: EXTREMELY INCOMPLETE IMPLEMENTATION - ~2% of TypeScript logic
    // Missing from TypeScript version (battle.ts:3183-3221, 39 lines):
    // 1. Check turn === 0 (only show at start)
    // 2. For each side, create modified PokemonSet array with:
    //    - Empty name and nature (hidden)
    //    - Null EVs/IVs (hidden)
    //    - Revealed: species, item, ability, moves, gender, level
    // 3. Gen 8: add gigantamax property
    // 4. Gen 9: add teraType property
    // 5. Hidden Power: add hpType only if Pokemon has Hidden Power move
    // 6. Zacian/Zamazenta special handling:
    //    - If has Rusted Sword/Shield, change species to Crowned forme
    //    - Replace Iron Head with Behemoth Blade/Bash in move list
    // 7. Pack team using Teams.pack()
    // 8. Send via this.add('showteam', side.id, Teams.pack(team))
    // Current implementation just shows a generic message, doesn't reveal actual teams
    //
    // Also: TypeScript takes no parameters, Rust has unused _side_idx parameter (signature mismatch)
    //
    // 	showOpenTeamSheets() {
    // 		if (this.turn !== 0) return;
    // 		for (const side of this.sides) {
    // 			const team = side.pokemon.map(pokemon => {
    // 				const set = pokemon.set;
    // 				const newSet: PokemonSet = {
    // 					name: '',
    // 					species: set.species,
    // 					item: set.item,
    // 					ability: set.ability,
    // 					moves: set.moves,
    // 					nature: '',
    // 					gender: pokemon.gender,
    // 					evs: null!,
    // 					ivs: null!,
    // 					level: set.level,
    // 				};
    // 				if (this.gen === 8) newSet.gigantamax = set.gigantamax;
    // 				if (this.gen === 9) newSet.teraType = set.teraType;
    // 				// Only display Hidden Power type if the Pokemon has Hidden Power
    // 				// This is based on how team sheets were written in past VGC formats
    // 				if (set.moves.some(m => this.dex.moves.get(m).id === 'hiddenpower')) newSet.hpType = set.hpType;
    // 				// This is done so the client doesn't flag Zacian/Zamazenta as illusions
    // 				// when they use their signature move
    // 				if ((toID(set.species) === 'zacian' && toID(set.item) === 'rustedsword') ||
    // 					(toID(set.species) === 'zamazenta' && toID(set.item) === 'rustedshield')) {
    // 					newSet.species = Dex.species.get(set.species + 'crowned').name;
    // 					const crowned: { [k: string]: string } = {
    // 						'Zacian-Crowned': 'behemothblade', 'Zamazenta-Crowned': 'behemothbash',
    // 					};
    // 					const ironHeadIndex = set.moves.map(toID).indexOf('ironhead' as ID);
    // 					if (ironHeadIndex >= 0) {
    // 						newSet.moves[ironHeadIndex] = crowned[newSet.species];
    // 					}
    // 				}
    // 				return newSet;
    // 			});
    //
    // 			this.add('showteam', side.id, Teams.pack(team));
    // 		}
    // 	}
    //
    pub fn show_open_team_sheets(&mut self, _side_idx: Option<usize>) {
        // STUB - just shows generic message instead of actual team data
        self.add("-message", &["Team sheets revealed".into()]);
    }
}
