#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed3.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 3] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch randomChance to see all calls
const originalRandomChance = battle.randomChance.bind(battle);
battle.randomChance = function(numerator, denominator) {
    if (battle.turn === 16) {
        const stack = new Error().stack;
        const lines = stack.split('\n');
        const caller = lines[2]?.trim() || 'unknown';
        console.log(`[T16 randomChance] ${numerator}/${denominator} from ${caller}`);
    }
    return originalRandomChance(numerator, denominator);
};

// Patch random to see all calls
const originalRandom = battle.random.bind(battle);
battle.random = function(n) {
    if (battle.turn === 16) {
        const stack = new Error().stack;
        const lines = stack.split('\n');
        const caller = lines[2]?.trim() || 'unknown';
        console.log(`[T16 random] random(${n}) from ${caller}`);
    }
    return originalRandom(n);
};

// Run turns 1-15
for (let i = 1; i < 16; i++) {
    battle.makeChoices('default', 'default');
}

console.log('\n=== TURN 16 START ===\n');
battle.makeChoices('default', 'default');
console.log('\n=== TURN 16 END ===');
