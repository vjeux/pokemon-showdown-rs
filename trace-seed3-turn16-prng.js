#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed3.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 3] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

let prngCallCount = 0;
let turn15Count = 0;

// Patch PRNG to count calls
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;

        // Calls 48-54 are turn 16 (turn 15 ends at 47)
        if (prngCallCount >= 48 && prngCallCount <= 54) {
            const stack = new Error().stack;
            const lines = stack.split('\n');
            const caller1 = lines[2]?.trim() || '';
            const caller2 = lines[3]?.trim() || '';
            console.log(`[PRNG ${prngCallCount}] value=${value}`);
            console.log(`  ${caller1}`);
            console.log(`  ${caller2}`);
        }
        return value;
    };
}

// Run turns 1-16
for (let i = 1; i <= 16; i++) {
    const before = prngCallCount;
    battle.makeChoices('default', 'default');
    const after = prngCallCount;
    if (i === 15) turn15Count = prngCallCount;
    if (i === 16) {
        console.log(`\nTurn 15 ended at PRNG call ${turn15Count}`);
        console.log(`Turn 16 ended at PRNG call ${after}`);
        console.log(`Turn 16 had ${after - turn15Count} PRNG calls`);
    }
}
