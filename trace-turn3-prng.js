#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch PRNG to log calls
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
let prngCount = 0;
battle.prng.rng.next = function() {
    const value = originalNext();
    prngCount++;
    if (prngCount >= 7) { // Only log turn 3 calls
        const stack = new Error().stack.split('\n')[2];
        console.log(`[PRNG #${prngCount}] ${value} - ${stack.trim()}`);
    }
    return value;
};

// Run turns
battle.makeChoices('default', 'default'); // turn 1
battle.makeChoices('default', 'default'); // turn 2

console.log('\n=== Turn 3 ===\n');
battle.makeChoices('default', 'default'); // turn 3

console.log(`\nTotal PRNG calls: ${prngCount}`);
