#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch PRNG to see exact calls
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        const stack = new Error().stack;
        const lines = stack.split('\n');
        // Show multiple stack frames for context
        const caller1 = lines[2]?.trim() || '';
        const caller2 = lines[3]?.trim() || '';
        const caller3 = lines[4]?.trim() || '';
        if (prngCallCount >= 42) {  // Show turn 19 and 20
            console.log(`[PRNG ${prngCallCount}]`);
            console.log(`  ${caller1}`);
            console.log(`  ${caller2}`);
            console.log(`  ${caller3}`);
        }
        return value;
    };
}

// Run turns 1-19
for (let i = 1; i < 20; i++) {
    battle.makeChoices('default', 'default');
}

console.log(`\n=== TURN 20 START (count: ${prngCallCount}) ===\n`);
battle.makeChoices('default', 'default');
console.log(`\n=== TURN 20 END (count: ${prngCallCount}) ===`);
