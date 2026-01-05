#!/usr/bin/env node

/**
 * Generate test teams for battle comparison
 *
 * This script generates random battle teams with a specific seed
 * using the same logic as the Rust team generator (NOT gen9randombattle format).
 *
 * Usage: node tests/generate-test-teams.js [seed_number]
 */

const {Dex} = require('./../../pokemon-showdown-ts/dist/sim/dex');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const seedNum = parseInt(process.argv[2]) || 1;

console.log(`Generating test teams for seed ${seedNum}...`);

// Create PRNG with seed (matching Rust: PRNGSeed::Gen5([0, 0, 0, seed_num]))
const prng = new PRNG([0, 0, 0, seedNum]);

// Generate two teams
const p1Team = generateRandomTeam(prng);
const p2Team = generateRandomTeam(prng);

const teams = {p1: p1Team, p2: p2Team};

// Write to /tmp
const filename = `/tmp/teams-seed${seedNum}-js.json`;
fs.writeFileSync(filename, JSON.stringify(teams, null, 2));

console.log(`✓ Generated teams for seed ${seedNum}`);
console.log(`  P1: ${p1Team[0].name} (${p1Team[0].species})`);
console.log(`  P2: ${p2Team[0].name} (${p2Team[0].species})`);
console.log(`  File: ${filename}`);

/**
 * Generate a random 6-Pokemon team
 * This mirrors the Rust implementation in src/team_generator.rs exactly
 */
function generateRandomTeam(prng) {
    const team = [];

    // Get all available species (sorted by name for determinism)
    // Use raw Pokedex data to match Rust behavior
    const allSpecies = Object.values(Dex.data.Pokedex).sort((a, b) => a.name.localeCompare(b.name));
    if (allSpecies.length === 0) return team;

    // Get all available moves (sorted for determinism)
    // Use raw Moves keys to match Rust (includes hiddenpowerpsychic etc)
    const allMoves = Object.keys(Dex.data.Moves).sort();

    // Get all available items (sorted for determinism)
    const allItems = Object.keys(Dex.data.Items).sort();

    // Get all available natures (sorted for determinism)
    const allNatures = Object.keys(Dex.data.Natures).sort();

    // Track used species and items to avoid duplicates
    const usedSpecies = [];
    const usedItems = [];

    for (let i = 0; i < 6; i++) {
        // Select random species (avoid duplicates)
        let availableSpecies = allSpecies.filter(s => !usedSpecies.includes(s.name));
        if (availableSpecies.length === 0) {
            availableSpecies = allSpecies;
        }
        const species = prng.sample(availableSpecies);
        usedSpecies.push(species.name);

        // Select random level between 50-100
        const level = Math.floor(prng.random() * (101 - 50)) + 50;

        // Select random ability from species' abilities
        // For cosmetic formes, inherit from baseSpecies
        let ability = '';
        if (species.abilities && species.abilities['0']) {
            ability = species.abilities['0'];
        } else if (species.baseSpecies) {
            // Cosmetic forme - get ability from base species
            const baseSpecies = Dex.data.Pokedex[species.baseSpecies.toLowerCase().replace(/[^a-z0-9]/g, '')];
            if (baseSpecies && baseSpecies.abilities && baseSpecies.abilities['0']) {
                ability = baseSpecies.abilities['0'];
            }
        }

        // Select random item (avoid duplicates, allow empty string)
        let item = '';
        if (allItems.length > 0) {
            let availableItems = allItems.filter(i => !usedItems.includes(i) || i === '');
            if (availableItems.length === 0) {
                availableItems = [''];
            }
            item = prng.sample(availableItems) || '';
            if (item !== '') {
                usedItems.push(item);
            }
        }

        // Select random nature
        const nature = allNatures.length > 0 ? prng.sample(allNatures) : 'hardy';

        // Select random gender based on species gender ratio
        let gender = '';
        if (species.genderRatio) {
            if (species.genderRatio.M > 0 && species.genderRatio.F > 0) {
                // Mixed gender - 50/50 chance
                gender = prng.randomChance(1, 2) ? 'M' : 'F';
            } else if (species.genderRatio.M > 0) {
                gender = 'M';
            } else if (species.genderRatio.F > 0) {
                gender = 'F';
            } else {
                gender = '';
            }
        } else if (species.gender) {
            // No genderRatio, but gender field is set
            if (species.gender === 'M') {
                gender = 'M';
            } else if (species.gender === 'F') {
                gender = 'F';
            } else if (species.gender === 'N') {
                gender = '';
            } else {
                // Default to 50/50
                gender = prng.randomChance(1, 2) ? 'M' : 'F';
            }
        } else {
            // No genderRatio and no gender field - default to 50/50
            gender = prng.randomChance(1, 2) ? 'M' : 'F';
        }

        // Select 4 random moves
        const moves = [];
        for (let j = 0; j < 4; j++) {
            if (allMoves.length > 0) {
                const move = prng.sample(allMoves);
                if (!moves.includes(move)) {
                    moves.push(move);
                }
            }
        }
        // Fill with "tackle" if not enough unique moves
        while (moves.length < 4) {
            moves.push('tackle');
        }

        // Generate random EVs (total max 510, each stat max 252)
        const evs = generateRandomEVs(prng);

        // Generate random IVs (0-31 for each stat)
        const ivs = {
            hp: Math.floor(prng.random() * 32),
            atk: Math.floor(prng.random() * 32),
            def: Math.floor(prng.random() * 32),
            spa: Math.floor(prng.random() * 32),
            spd: Math.floor(prng.random() * 32),
            spe: Math.floor(prng.random() * 32),
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
 * Generate random EVs with constraints:
 * - Total EVs <= 510
 * - Each stat <= 252
 * This mirrors the Rust implementation exactly
 */
function generateRandomEVs(prng) {
    const evs = [0, 0, 0, 0, 0, 0]; // hp, atk, def, spa, spd, spe
    let totalEvs = 0;

    // Distribute 510 EVs one at a time to random stats
    while (totalEvs < 510) {
        // Find stats that haven't reached the 252 limit
        const availableStats = [];
        for (let i = 0; i < 6; i++) {
            if (evs[i] < 252) {
                availableStats.push(i);
            }
        }

        // If no stats available, break
        if (availableStats.length === 0) break;

        // Pick a random stat from available ones
        const statIdx = prng.sample(availableStats);

        // Add 1-4 EVs to this stat (for faster generation), but don't exceed limits
        const amount = Math.min(
            Math.floor(prng.random() * 4) + 1,
            252 - evs[statIdx],
            510 - totalEvs
        );
        evs[statIdx] += amount;
        totalEvs += amount;
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
