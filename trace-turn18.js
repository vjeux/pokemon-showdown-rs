#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch PRNG to trace calls
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
let prngCount = 0;
let turn18Start = 0;
let tracingTurn18 = false;

battle.prng.rng.next = function() {
    const value = originalNext();
    prngCount++;
    if (tracingTurn18) {
        const stack = new Error().stack.split('\n')[2];
        console.log(`[PRNG #${prngCount}] ${value} - ${stack.trim()}`);
    }
    return value;
};

// Run turns 1-17
for (let i = 1; i < 18; i++) {
    battle.makeChoices('default', 'default');
}

console.log('\n=== Turn 18 ===\n');
turn18Start = prngCount;
tracingTurn18 = true;
battle.makeChoices('default', 'default');
tracingTurn18 = false;

console.log(`\nTotal PRNG calls in turn 18: ${prngCount - turn18Start}`);
console.log(`Total PRNG calls after turn 18: ${prngCount}`);
