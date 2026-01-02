// TODO: Implement canMegaEvo from JavaScript
//
// JS Source:
// 
// 	canMegaEvo(pokemon: Pokemon) {
// 		const species = pokemon.baseSpecies;
// 		const altForme = species.otherFormes && this.dex.species.get(species.otherFormes[0]);
// 		const item = pokemon.getItem();
// 		// Mega Rayquaza
// 		if ((this.battle.gen <= 7 || this.battle.ruleTable.has('+pokemontag:past') ||
// 			this.battle.ruleTable.has('+pokemontag:future')) &&
// 			altForme?.isMega && altForme?.requiredMove &&
// 			pokemon.baseMoves.includes(toID(altForme.requiredMove)) && !item.zMove) {
// 			return altForme.name;
// 		}
// 		// Temporary hardcode until generation shift
// 		if ((species.baseSpecies === "Floette" || species.baseSpecies === "Zygarde") && item.megaEvolves === species.name) {
// 			return item.megaStone as string;
// 		}
// 		// a hacked-in Megazard X can mega evolve into Megazard Y, but not into Megazard X
// 		if (Array.isArray(item.megaStone)) {
// 			// FIXME: Change to species.name when champions comes
// 			const index = (item.megaEvolves as string[]).indexOf(species.baseSpecies);
// 			if (index < 0) return null;
// 			return item.megaStone[index];
// 			// FIXME: Change to species.name when champions comes
// 		} else if (item.megaEvolves === species.baseSpecies && item.megaStone !== species.name) {
// 			return item.megaStone;
// 		}
// 		return null;
// 	}

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
