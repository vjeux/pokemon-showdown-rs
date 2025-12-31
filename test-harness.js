#!/usr/bin/env node

/**
 * Test Harness - Compare Rust and JavaScript Battle Implementations
 *
 * This script:
 * 1. Generates teams using seed=1 in both Rust and JavaScript
 * 2. Runs battles with those teams
 * 3. Compares the results to ensure they're identical
 */

const fs = require('fs');
const { execSync } = require('child_process');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;
const { PRNG } = require('./../pokemon-showdown-ts/dist/sim/prng');
const { generateRandomTeams } = require('./generate-teams');

// Colors for terminal output
const colors = {
	reset: '\x1b[0m',
	green: '\x1b[32m',
	red: '\x1b[31m',
	yellow: '\x1b[33m',
	blue: '\x1b[34m',
};

function log(message, color = 'reset') {
	console.log(`${colors[color]}${message}${colors.reset}`);
}

/**
 * Generate teams using JavaScript with seed=1
 */
function generateTeamsJS() {
	log('\n=== Generating Teams in JavaScript ===', 'blue');
	const prng = new PRNG([0, 0, 0, 1]);
	const dex = Dex.forFormat('gen9randombattle');

	const teams = generateRandomTeams(prng, dex);

	log(`Generated P1 team: ${teams.p1.map(p => p.species).join(', ')}`, 'green');
	log(`Generated P2 team: ${teams.p2.map(p => p.species).join(', ')}`, 'green');

	// Save teams to JSON file
	fs.writeFileSync('teams-js.json', JSON.stringify(teams, null, 2));
	log('Saved to teams-js.json', 'green');

	return teams;
}

/**
 * Generate teams using Rust with seed=1
 */
function generateTeamsRust() {
	log('\n=== Generating Teams in Rust ===', 'blue');

	try {
		// Run the Rust example that generates and exports teams
		execSync(
			'docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example export_teams 2>&1" | tail -20',
			{ encoding: 'utf8', stdio: 'inherit' }
		);

		// Copy the file from Docker to local
		execSync(
			'docker cp pokemon-rust-dev:/home/builder/workspace/teams-rust.json ./teams-rust.json',
			{ encoding: 'utf8' }
		);

		// Read the exported teams
		const teams = JSON.parse(fs.readFileSync('teams-rust.json', 'utf8'));

		log(`Generated P1 team: ${teams.p1.map(p => p.species).join(', ')}`, 'green');
		log(`Generated P2 team: ${teams.p2.map(p => p.species).join(', ')}`, 'green');
		log('Saved to teams-rust.json', 'green');

		return teams;
	} catch (error) {
		log(`Error running Rust: ${error.message}`, 'red');
		return null;
	}
}

/**
 * Run a battle in JavaScript with the generated teams
 */
function runBattleJS(teams) {
	log('\n=== Running Battle in JavaScript ===', 'blue');

	const BattleStreams = require('./../pokemon-showdown-ts/dist/sim').BattleStreams;
	const streams = BattleStreams.getPlayerStreams(new BattleStreams.BattleStream());

	const spec = {
		formatid: 'gen9randombattle',
		seed: [0, 0, 0, 1],
	};

	const p1spec = {
		name: 'Player 1',
		team: teams.p1,
	};

	const p2spec = {
		name: 'Player 2',
		team: teams.p2,
	};

	const battleLog = [];

	return new Promise((resolve, reject) => {
		streams.omniscient.write(`>start ${JSON.stringify(spec)}`);
		streams.omniscient.write(`>player p1 ${JSON.stringify(p1spec)}`);
		streams.omniscient.write(`>player p2 ${JSON.stringify(p2spec)}`);

		streams.omniscient.on('data', (chunk) => {
			battleLog.push(chunk);

			// Auto-choose moves for testing
			if (chunk.includes('|request|')) {
				const request = JSON.parse(chunk.split('|request|')[1]);
				if (request.active) {
					// Always choose move 1
					const player = chunk.includes('\np1') ? 'p1' : 'p2';
					streams[player].write(`>choose move 1`);
				}
			}

			if (chunk.includes('|win|')) {
				log('Battle completed', 'green');
				fs.writeFileSync('battle-log-js.txt', battleLog.join('\n'));
				resolve(battleLog);
			}
		});

		streams.omniscient.on('error', reject);

		// Timeout after 10 seconds
		setTimeout(() => {
			log('Battle timed out', 'yellow');
			fs.writeFileSync('battle-log-js.txt', battleLog.join('\n'));
			resolve(battleLog);
		}, 10000);
	});
}

/**
 * Run a battle in Rust with seed=1
 */
function runBattleRust() {
	log('\n=== Running Battle in Rust ===', 'blue');

	try {
		const output = execSync(
			'docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo test --test battle_state_comparison test_battle_state_comparison -- --nocapture 2>&1"',
			{ encoding: 'utf8', maxBuffer: 10 * 1024 * 1024 }
		);

		log('Rust battle test completed', 'green');

		// The test should have created battle-state-rust.json
		// We would need to read and parse it

		return output;
	} catch (error) {
		log(`Error running Rust battle: ${error.message}`, 'red');
		return null;
	}
}

/**
 * Compare the results
 */
function compareResults(jsTeams, rustTeams, jsLog, rustOutput) {
	log('\n=== Comparing Results ===', 'blue');

	let allPassed = true;

	// Validate JavaScript teams
	if (!jsTeams || !jsTeams.p1 || !jsTeams.p2) {
		log('✗ JavaScript team generation failed', 'red');
		return false;
	}

	// Validate Rust teams
	if (!rustTeams || !rustTeams.p1 || !rustTeams.p2) {
		log('✗ Rust team generation failed', 'red');
		return false;
	}

	// Check that teams have 6 Pokemon each
	if (jsTeams.p1.length !== 6 || jsTeams.p2.length !== 6) {
		log(`✗ JS team size mismatch: P1=${jsTeams.p1.length}, P2=${jsTeams.p2.length}`, 'red');
		allPassed = false;
	} else {
		log(`✓ JS teams have correct size (6 each)`, 'green');
	}

	if (rustTeams.p1.length !== 6 || rustTeams.p2.length !== 6) {
		log(`✗ Rust team size mismatch: P1=${rustTeams.p1.length}, P2=${rustTeams.p2.length}`, 'red');
		allPassed = false;
	} else {
		log(`✓ Rust teams have correct size (6 each)`, 'green');
	}

	// Compare teams between Rust and JavaScript
	log('\n--- Comparing Rust vs JavaScript Teams ---', 'yellow');

	// Compare P1 teams
	for (let i = 0; i < 6; i++) {
		const jsPokemon = jsTeams.p1[i];
		const rustPokemon = rustTeams.p1[i];

		if (jsPokemon.species !== rustPokemon.species) {
			log(`✗ P1[${i}] species mismatch: JS=${jsPokemon.species}, Rust=${rustPokemon.species}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.level !== rustPokemon.level) {
			log(`✗ P1[${i}] level mismatch: JS=${jsPokemon.level}, Rust=${rustPokemon.level}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.ability !== rustPokemon.ability) {
			log(`✗ P1[${i}] ability mismatch: JS=${jsPokemon.ability}, Rust=${rustPokemon.ability}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.item !== rustPokemon.item) {
			log(`✗ P1[${i}] item mismatch: JS=${jsPokemon.item}, Rust=${rustPokemon.item}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.nature !== rustPokemon.nature) {
			log(`✗ P1[${i}] nature mismatch: JS=${jsPokemon.nature}, Rust=${rustPokemon.nature}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.gender !== rustPokemon.gender) {
			log(`✗ P1[${i}] gender mismatch: JS=${jsPokemon.gender}, Rust=${rustPokemon.gender}`, 'red');
			allPassed = false;
		}

		// Compare moves
		if (JSON.stringify(jsPokemon.moves) !== JSON.stringify(rustPokemon.moves)) {
			log(`✗ P1[${i}] moves mismatch:`, 'red');
			log(`  JS:   ${jsPokemon.moves.join(', ')}`, 'red');
			log(`  Rust: ${rustPokemon.moves.join(', ')}`, 'red');
			allPassed = false;
		}

		// Compare EVs
		if (JSON.stringify(jsPokemon.evs) !== JSON.stringify(rustPokemon.evs)) {
			log(`✗ P1[${i}] EVs mismatch:`, 'red');
			log(`  JS:   ${JSON.stringify(jsPokemon.evs)}`, 'red');
			log(`  Rust: ${JSON.stringify(rustPokemon.evs)}`, 'red');
			allPassed = false;
		}

		// Compare IVs
		if (JSON.stringify(jsPokemon.ivs) !== JSON.stringify(rustPokemon.ivs)) {
			log(`✗ P1[${i}] IVs mismatch:`, 'red');
			log(`  JS:   ${JSON.stringify(jsPokemon.ivs)}`, 'red');
			log(`  Rust: ${JSON.stringify(rustPokemon.ivs)}`, 'red');
			allPassed = false;
		}
	}

	// Compare P2 teams
	for (let i = 0; i < 6; i++) {
		const jsPokemon = jsTeams.p2[i];
		const rustPokemon = rustTeams.p2[i];

		if (jsPokemon.species !== rustPokemon.species) {
			log(`✗ P2[${i}] species mismatch: JS=${jsPokemon.species}, Rust=${rustPokemon.species}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.level !== rustPokemon.level) {
			log(`✗ P2[${i}] level mismatch: JS=${jsPokemon.level}, Rust=${rustPokemon.level}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.ability !== rustPokemon.ability) {
			log(`✗ P2[${i}] ability mismatch: JS=${jsPokemon.ability}, Rust=${rustPokemon.ability}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.item !== rustPokemon.item) {
			log(`✗ P2[${i}] item mismatch: JS=${jsPokemon.item}, Rust=${rustPokemon.item}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.nature !== rustPokemon.nature) {
			log(`✗ P2[${i}] nature mismatch: JS=${jsPokemon.nature}, Rust=${rustPokemon.nature}`, 'red');
			allPassed = false;
		}

		if (jsPokemon.gender !== rustPokemon.gender) {
			log(`✗ P2[${i}] gender mismatch: JS=${jsPokemon.gender}, Rust=${rustPokemon.gender}`, 'red');
			allPassed = false;
		}

		// Compare moves
		if (JSON.stringify(jsPokemon.moves) !== JSON.stringify(rustPokemon.moves)) {
			log(`✗ P2[${i}] moves mismatch:`, 'red');
			log(`  JS:   ${jsPokemon.moves.join(', ')}`, 'red');
			log(`  Rust: ${rustPokemon.moves.join(', ')}`, 'red');
			allPassed = false;
		}

		// Compare EVs
		if (JSON.stringify(jsPokemon.evs) !== JSON.stringify(rustPokemon.evs)) {
			log(`✗ P2[${i}] EVs mismatch:`, 'red');
			log(`  JS:   ${JSON.stringify(jsPokemon.evs)}`, 'red');
			log(`  Rust: ${JSON.stringify(rustPokemon.evs)}`, 'red');
			allPassed = false;
		}

		// Compare IVs
		if (JSON.stringify(jsPokemon.ivs) !== JSON.stringify(rustPokemon.ivs)) {
			log(`✗ P2[${i}] IVs mismatch:`, 'red');
			log(`  JS:   ${JSON.stringify(jsPokemon.ivs)}`, 'red');
			log(`  Rust: ${JSON.stringify(rustPokemon.ivs)}`, 'red');
			allPassed = false;
		}
	}

	if (allPassed) {
		log('\n✓ All teams match perfectly between Rust and JavaScript!', 'green');
	}

	// Check for duplicate species in JS
	const jsP1Species = jsTeams.p1.map(p => p.species);
	const jsP2Species = jsTeams.p2.map(p => p.species);

	if (new Set(jsP1Species).size !== jsP1Species.length) {
		log('✗ JS P1 has duplicate species', 'red');
		allPassed = false;
	} else {
		log('✓ JS P1 has no duplicate species', 'green');
	}

	if (new Set(jsP2Species).size !== jsP2Species.length) {
		log('✗ JS P2 has duplicate species', 'red');
		allPassed = false;
	} else {
		log('✓ JS P2 has no duplicate species', 'green');
	}

	// Check for duplicate items in JS (excluding empty)
	const jsP1Items = jsTeams.p1.map(p => p.item).filter(i => i !== '');
	const jsP2Items = jsTeams.p2.map(p => p.item).filter(i => i !== '');

	if (new Set(jsP1Items).size !== jsP1Items.length) {
		log('✗ JS P1 has duplicate items', 'red');
		allPassed = false;
	} else {
		log('✓ JS P1 has no duplicate items', 'green');
	}

	if (new Set(jsP2Items).size !== jsP2Items.length) {
		log('✗ JS P2 has duplicate items', 'red');
		allPassed = false;
	} else {
		log('✓ JS P2 has no duplicate items', 'green');
	}

	// Verify EVs are valid for all teams
	for (const [name, teams] of [['JS', jsTeams], ['Rust', rustTeams]]) {
		for (const [teamName, team] of [['P1', teams.p1], ['P2', teams.p2]]) {
			for (const pokemon of team) {
				const totalEVs = pokemon.evs.hp + pokemon.evs.atk + pokemon.evs.def +
				                pokemon.evs.spa + pokemon.evs.spd + pokemon.evs.spe;

				if (totalEVs > 510) {
					log(`✗ ${name} ${teamName} ${pokemon.species} has too many EVs: ${totalEVs}`, 'red');
					allPassed = false;
				}

				for (const stat of Object.values(pokemon.evs)) {
					if (stat > 252) {
						log(`✗ ${name} ${teamName} ${pokemon.species} has a stat with EVs > 252`, 'red');
						allPassed = false;
					}
				}
			}
		}
	}

	if (allPassed) {
		log('\n✓ All EV validations passed', 'green');
	}

	return allPassed;
}

/**
 * Main test function
 */
async function runTests() {
	log('='.repeat(60), 'blue');
	log('Pokemon Showdown - Rust vs JavaScript Test Harness', 'blue');
	log('='.repeat(60), 'blue');

	try {
		// Step 1: Generate teams
		const jsTeams = generateTeamsJS();
		const rustTeams = generateTeamsRust();

		// Step 2: Run battles (optional for now)
		log('\n--- Skipping Battle Runs (comparing teams only) ---', 'yellow');
		// const jsLog = await runBattleJS(jsTeams);
		// const rustOutput = runBattleRust();

		// Step 3: Compare results
		const success = compareResults(jsTeams, rustTeams, null, null);

		// Summary
		log('\n' + '='.repeat(60), 'blue');
		if (success) {
			log('✓ Test Harness Completed Successfully', 'green');
			log('JavaScript implementation validated', 'green');
			log('Note: Full Rust comparison requires additional implementation', 'yellow');
		} else {
			log('✗ Test Harness Found Issues', 'red');
			process.exit(1);
		}
		log('='.repeat(60), 'blue');

	} catch (error) {
		log(`\nError: ${error.message}`, 'red');
		console.error(error.stack);
		process.exit(1);
	}
}

// Run tests
if (require.main === module) {
	runTests().catch(error => {
		log(`Fatal error: ${error.message}`, 'red');
		console.error(error.stack);
		process.exit(1);
	});
}

module.exports = { runTests, generateTeamsJS, runBattleJS, compareResults };
