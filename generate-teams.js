/**
 * Team Generator - Generates random Pokemon teams from a seed
 *
 * This module provides functionality to generate random 6v6 Pokemon teams
 * deterministically from a PRNG seed.
 *
 * This is a 1-to-1 port of the Rust implementation in src/team_generator.rs
 */

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

/**
 * Generate random EVs with constraints:
 * - Total EVs <= 510
 * - Each stat <= 252
 */
function generateRandomEVs(prng) {
	const evs = [0, 0, 0, 0, 0, 0]; // hp, atk, def, spa, spd, spe
	let totalEVs = 0;

	// Distribute 510 EVs one at a time to random stats
	while (totalEVs < 510) {
		// Find stats that haven't reached the 252 limit
		const availableStats = [];
		for (let i = 0; i < evs.length; i++) {
			if (evs[i] < 252) {
				availableStats.push(i);
			}
		}

		// If no stats available (shouldn't happen), break
		if (availableStats.length === 0) {
			break;
		}

		// Pick a random stat from available ones
		const statIdx = prng.sample(availableStats);

		// Add 1-4 EVs to this stat (for faster generation), but don't exceed limits
		const amount = Math.min(
			prng.random(1, 5),
			252 - evs[statIdx],
			510 - totalEVs
		);
		evs[statIdx] += amount;
		totalEVs += amount;
	}

	return {
		hp: evs[0],
		atk: evs[1],
		def: evs[2],
		spa: evs[3],
		spd: evs[4],
		spe: evs[5],
	};
}

/**
 * Generate a random Pokemon team (6 Pokemon) from a PRNG
 */
function generateRandomTeam(prng, dex) {
	const team = [];

	// Get all available species as an array for sampling (sorted by name for determinism across languages)
	let allSpecies = Object.values(dex.species.all());
	allSpecies.sort((a, b) => a.name.localeCompare(b.name));
	if (allSpecies.length === 0) {
		return team;
	}

	// Get all available moves as an array for sampling (extract IDs)
	const allMoves = Object.values(dex.moves.all()).map(m => m.id);

	// Get all available items as an array for sampling (extract IDs)
	const allItems = Object.values(dex.items.all()).map(i => i.id);

	// Get all available natures as an array for sampling (extract IDs)
	const allNatures = Object.values(dex.natures.all()).map(n => n.id);

	// Track used species and items to avoid duplicates
	const usedSpecies = [];
	const usedItems = [];

	for (let i = 0; i < 6; i++) {
		// Select random species (avoid duplicates)
		let availableSpecies = allSpecies.filter(s => !usedSpecies.includes(s.name));

		// If we've used all species, allow reuse (shouldn't happen with 1000+ species)
		if (availableSpecies.length === 0) {
			availableSpecies = allSpecies.slice();
		}

		const species = prng.sample(availableSpecies);
		usedSpecies.push(species.name);

		// Select random level between 50-100
		const level = prng.random(50, 101);

		// Select random ability from the species' abilities
		const ability = species.abilities['0'] || 'No Ability';

		// Select random item (avoid duplicates, allow empty string for multiple Pokemon)
		let item = '';
		if (allItems.length > 0) {
			let availableItems = allItems.filter(i => !usedItems.includes(i) || i === '');

			// If we've used all items, allow reuse or use empty string
			if (availableItems.length === 0) {
				availableItems = [''];
			}

			const selectedItem = prng.sample(availableItems) || '';
			if (selectedItem !== '') {
				usedItems.push(selectedItem);
			}
			item = selectedItem;
		}

		// Select random nature
		const nature = allNatures.length > 0 ? prng.sample(allNatures) : 'Hardy';

		// Select random gender based on species gender ratio
		let gender = '';
		if (species.genderRatio) {
			if (species.genderRatio.M > 0 && species.genderRatio.F > 0) {
				// Mixed gender
				gender = prng.randomChance(1, 2) ? 'M' : 'F';
			} else if (species.genderRatio.M > 0) {
				gender = 'M';
			} else if (species.genderRatio.F > 0) {
				gender = 'F';
			}
		}

		// Select 4 random moves
		const moves = [];
		for (let j = 0; j < 4; j++) {
			if (allMoves.length > 0) {
				const moveName = prng.sample(allMoves) || 'tackle';
				if (!moves.includes(moveName)) {
					moves.push(moveName);
				}
			}
		}
		// Fill with "tackle" if we don't have enough unique moves
		while (moves.length < 4) {
			moves.push('tackle');
		}

		// Generate random EVs (total max 510, each stat max 252)
		const evs = generateRandomEVs(prng);

		// Generate random IVs (0-31 for each stat)
		const ivs = {
			hp: prng.random(0, 32),
			atk: prng.random(0, 32),
			def: prng.random(0, 32),
			spa: prng.random(0, 32),
			spd: prng.random(0, 32),
			spe: prng.random(0, 32),
		};

		team.push({
			name: species.name,
			species: species.name,
			level: level,
			ability: ability,
			item: item,
			nature: nature,
			gender: gender,
			moves: moves,
			evs: evs,
			ivs: ivs,
		});
	}

	return team;
}

/**
 * Generate two random teams (P1 and P2) from a seed
 */
function generateRandomTeams(prng, dex) {
	const team1 = generateRandomTeam(prng, dex);
	const team2 = generateRandomTeam(prng, dex);
	return {p1: team1, p2: team2};
}

module.exports = {
	generateRandomTeam,
	generateRandomTeams,
	generateRandomEVs,
};

// Example usage (if run directly):
if (require.main === module) {
	const prng = new PRNG([1, 2, 3, 4]);
	const dex = Dex.forFormat('gen9randombattle');

	console.log('Generating random teams with seed [1, 2, 3, 4]...');
	const teams = generateRandomTeams(prng, dex);

	console.log('\nP1 Team:');
	teams.p1.forEach((pokemon, i) => {
		console.log(`${i + 1}. ${pokemon.name} @ ${pokemon.item || '(no item)'}`);
		console.log(`   Level: ${pokemon.level}, Nature: ${pokemon.nature}, Gender: ${pokemon.gender || 'N'}`);
		console.log(`   Ability: ${pokemon.ability}`);
		console.log(`   Moves: ${pokemon.moves.join(', ')}`);
		console.log(`   EVs: ${pokemon.evs.hp} HP / ${pokemon.evs.atk} Atk / ${pokemon.evs.def} Def / ${pokemon.evs.spa} SpA / ${pokemon.evs.spd} SpD / ${pokemon.evs.spe} Spe`);
		console.log(`   IVs: ${pokemon.ivs.hp} HP / ${pokemon.ivs.atk} Atk / ${pokemon.ivs.def} Def / ${pokemon.ivs.spa} SpA / ${pokemon.ivs.spd} SpD / ${pokemon.ivs.spe} Spe`);
		console.log('');
	});

	console.log('\nP2 Team:');
	teams.p2.forEach((pokemon, i) => {
		console.log(`${i + 1}. ${pokemon.name} @ ${pokemon.item || '(no item)'}`);
		console.log(`   Level: ${pokemon.level}, Nature: ${pokemon.nature}, Gender: ${pokemon.gender || 'N'}`);
		console.log(`   Ability: ${pokemon.ability}`);
		console.log(`   Moves: ${pokemon.moves.join(', ')}`);
		console.log(`   EVs: ${pokemon.evs.hp} HP / ${pokemon.evs.atk} Atk / ${pokemon.evs.def} Def / ${pokemon.evs.spa} SpA / ${pokemon.evs.spd} SpD / ${pokemon.evs.spe} Spe`);
		console.log(`   IVs: ${pokemon.ivs.hp} HP / ${pokemon.ivs.atk} Atk / ${pokemon.ivs.def} Def / ${pokemon.ivs.spa} SpA / ${pokemon.ivs.spd} SpD / ${pokemon.ivs.spe} Spe`);
		console.log('');
	});
}
