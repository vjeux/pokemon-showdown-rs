#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed3.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 3] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

let prngCallCount = 0;

// Patch PRNG to count calls
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;

        // Show detailed info around PRNG 54
        if (prngCallCount >= 51 && prngCallCount <= 57) {
            const stack = new Error().stack;
            const lines = stack.split('\n');
            const caller1 = lines[2]?.trim() || '';
            const caller2 = lines[3]?.trim() || '';
            const caller3 = lines[4]?.trim() || '';
            const caller4 = lines[5]?.trim() || '';
            console.log(`\n[PRNG ${prngCallCount}] value=${value}`);
            console.log(`  ${caller1}`);
            console.log(`  ${caller2}`);
            console.log(`  ${caller3}`);
            console.log(`  ${caller4}`);
        }
        return value;
    };
}

// Run turns 1-16
for (let i = 1; i <= 16; i++) {
    battle.makeChoices('default', 'default');
}
console.log(`\nTotal PRNG calls after turn 16: ${prngCallCount}`);
