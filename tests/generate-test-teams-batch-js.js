#!/usr/bin/env node

/**
 * Batch JavaScript Team Generator
 *
 * Generates random battle teams for multiple seeds
 *
 * Usage: node tests/generate-test-teams-batch-js.js [start_seed] [end_seed]
 * Example: node tests/generate-test-teams-batch-js.js 1 100
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const {Teams} = require('./../../pokemon-showdown-ts/dist/sim/teams');
const fs = require('fs');

const startSeed = parseInt(process.argv[2]) || 1;
const endSeed = parseInt(process.argv[3]) || 100;

console.error(`Generating JS teams for seeds ${startSeed}-${endSeed}`);

for (let seedNum = startSeed; seedNum <= endSeed; seedNum++) {
    try {
        const prng = new PRNG([0, 0, 0, seedNum]);
        const format = 'gen9randombattle';

        const p1Team = Teams.generate(format, {prng});
        const p2Team = Teams.generate(format, {prng});

        const teamData = {
            p1: p1Team.map(set => ({
                name: set.name,
                species: set.species,
                level: set.level,
                ability: set.ability,
                item: set.item,
                nature: set.nature,
                gender: set.gender || '',
                moves: set.moves,
                evs: set.evs,
                ivs: set.ivs,
            })),
            p2: p2Team.map(set => ({
                name: set.name,
                species: set.species,
                level: set.level,
                ability: set.ability,
                item: set.item,
                nature: set.nature,
                gender: set.gender || '',
                moves: set.moves,
                evs: set.evs,
                ivs: set.ivs,
            })),
        };

        const outputFile = `/tmp/teams-seed${seedNum}-js.json`;
        fs.writeFileSync(outputFile, JSON.stringify(teamData, null, 2));

    } catch (error) {
        console.error(`Seed ${seedNum}: ERROR - ${error.message}`);
    }
}

console.error('JS team generation complete');
