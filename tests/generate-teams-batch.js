#!/usr/bin/env node

/**
 * Batch Team Generator (JavaScript)
 *
 * Generates teams for multiple seeds in a single process.
 * Uses the same custom generateRandomTeam as generate-test-teams.js
 *
 * Usage: node tests/generate-teams-batch.js <start_seed> <end_seed>
 */

const {Dex} = require('./../../pokemon-showdown-ts/dist/sim/dex');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const startSeed = parseInt(process.argv[2]) || 1;
const endSeed = parseInt(process.argv[3]) || startSeed;

// Pre-load data once for efficiency
const allSpecies = Object.values(Dex.data.Pokedex).sort((a, b) => a.name.localeCompare(b.name));
const allMoves = Object.keys(Dex.data.Moves).sort();
const allItems = Object.keys(Dex.data.Items).sort();
const allNatures = Object.keys(Dex.data.Natures).sort();

for (let seedNum = startSeed; seedNum <= endSeed; seedNum++) {
    try {
        const prng = new PRNG([0, 0, 0, seedNum]);

        const team1 = generateRandomTeam(prng);
        const team2 = generateRandomTeam(prng);

        const teams = {
            p1: team1,
            p2: team2,
        };

        const outputFile = `/tmp/teams-seed${seedNum}-js.json`;
        fs.writeFileSync(outputFile, JSON.stringify(teams, null, 2));
        console.log(`SEED ${seedNum}: OK`);
    } catch (e) {
        console.log(`SEED ${seedNum}: ERROR - ${e.message}`);
    }
}

/**
 * Generate a random 6-Pokemon team
 * This mirrors the Rust implementation in src/team_generator.rs exactly
 */
function generateRandomTeam(prng) {
    const team = [];

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
