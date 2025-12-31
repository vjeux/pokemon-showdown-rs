#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch PRNG to see when it's called
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        const stack = new Error().stack;
        const callerLine = stack.split('\n')[2]; // Get the caller
        console.log(`[PRNG ${prngCallCount}] ${value} from ${callerLine.trim()}`);
        return value;
    };
}

// Run turns 1-19
for (let i = 1; i < 20; i++) {
    battle.makeChoices('default', 'default');
}

console.log(`\n=== TURN 20 START (PRNG count: ${prngCallCount}) ===\n`);

// Run turn 20
battle.makeChoices('default', 'default');

console.log(`\n=== TURN 20 END (PRNG count: ${prngCallCount}) ===`);
