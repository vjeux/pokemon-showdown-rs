// TODO: Implement terastallize from JavaScript
//
// JS Source:
// 
// 	terastallize(pokemon: Pokemon) {
// 		if (pokemon.species.baseSpecies === 'Ogerpon' && !['Fire', 'Grass', 'Rock', 'Water'].includes(pokemon.teraType) &&
// 			(!pokemon.illusion || pokemon.illusion.species.baseSpecies === 'Ogerpon')) {
// 			this.battle.hint("If Ogerpon Terastallizes into a type other than Fire, Grass, Rock, or Water, the game softlocks.");
// 			return;
// 		}
// 
// 		if (pokemon.illusion && ['Ogerpon', 'Terapagos'].includes(pokemon.illusion.species.baseSpecies)) {
// 			this.battle.singleEvent('End', this.dex.abilities.get('Illusion'), pokemon.abilityState, pokemon);
// 		}
// 
// 		const type = pokemon.teraType;
// 		this.battle.add('-terastallize', pokemon, type);
// 		pokemon.terastallized = type;
// 		for (const ally of pokemon.side.pokemon) {
// 			ally.canTerastallize = null;
// 		}
// 		pokemon.addedType = '';
// 		pokemon.knownType = true;
// 		pokemon.apparentType = type;
// 		if (pokemon.species.baseSpecies === 'Ogerpon') {
// 			let ogerponSpecies = toID(pokemon.species.battleOnly || pokemon.species.id);
// 			ogerponSpecies += ogerponSpecies === 'ogerpon' ? 'tealtera' : 'tera';
// 			pokemon.formeChange(ogerponSpecies, null, true);
// 		}
// 		if (pokemon.species.name === 'Terapagos-Terastal') {
// 			pokemon.formeChange('Terapagos-Stellar', null, true);
// 		}
// 		if (pokemon.species.baseSpecies === 'Morpeko' && !pokemon.transformed &&
// 			pokemon.baseSpecies.id !== pokemon.species.id
// 		) {
// 			pokemon.formeRegression = true;
// 			pokemon.baseSpecies = pokemon.species;
// 			pokemon.details = pokemon.getUpdatedDetails();
// 		}
// 		this.battle.runEvent('AfterTerastallization', pokemon);
// 	}

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
