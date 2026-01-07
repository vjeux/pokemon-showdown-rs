#!/usr/bin/env node

/**
 * JavaScript Battle Benchmark
 * Runs battles for multiple seeds and measures performance
 *
 * Usage: node tests/benchmark-battle-js.js [start_seed] [end_seed]
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const startSeed = parseInt(process.argv[2]) || 1;
const endSeed = parseInt(process.argv[3]) || 100;

let totalTurns = 0;
let totalBattles = 0;

const startTime = Date.now();

for (let seedNum = startSeed; seedNum <= endSeed; seedNum++) {
    const teamsFile = `/tmp/teams-seed${seedNum}-js.json`;

    if (!fs.existsSync(teamsFile)) {
        continue;
    }

    try {
        const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

        const battle = new Battle({formatid: 'gen9randombattle'});
        battle.prng = new PRNG([0, 0, 0, seedNum]);

        battle.setPlayer('p1', {team: teams.p1});
        battle.setPlayer('p2', {team: teams.p2});
        battle.start();

        let iteration = 0;
        const MAX_TURNS = 100;

        while (!battle.ended && battle.turn < MAX_TURNS) {
            iteration++;

            for (const side of battle.sides) {
                const request = side.getChoice();
                if (request && request.active) {
                    const pokemon = side.active[0];
                    if (pokemon && !pokemon.fainted) {
                        const moveIndex = (iteration % pokemon.moveSlots.length);
                        const choice = `move ${moveIndex + 1}`;
                        side.choose(choice);
                    }
                }
            }

            battle.makeChoices();
        }

        totalTurns += battle.turn;
        totalBattles++;

    } catch (error) {
        // Skip failed battles
    }
}

const endTime = Date.now();
const elapsedMs = endTime - startTime;
const elapsedSec = elapsedMs / 1000;

console.log(`JavaScript Benchmark Results:`);
console.log(`  Seeds: ${startSeed}-${endSeed}`);
console.log(`  Battles completed: ${totalBattles}`);
console.log(`  Total turns: ${totalTurns}`);
console.log(`  Time: ${elapsedSec.toFixed(2)}s`);
console.log(`  Battles/sec: ${(totalBattles / elapsedSec).toFixed(2)}`);
console.log(`  Turns/sec: ${(totalTurns / elapsedSec).toFixed(2)}`);
console.log(`  Avg ms/battle: ${(elapsedMs / totalBattles).toFixed(2)}`);
