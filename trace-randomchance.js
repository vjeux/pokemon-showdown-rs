#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch randomChance to see what it's used for
const originalRandomChance = battle.randomChance.bind(battle);
battle.randomChance = function(numerator, denominator) {
    const result = originalRandomChance(numerator, denominator);
    if (battle.turn === 3) {
        const stack = new Error().stack.split('\n').slice(1, 5).join('\n');
        console.log(`\n[randomChance] ${numerator}/${denominator} = ${result}`);
        console.log(stack);
    }
    return result;
};

// Run turns
battle.makeChoices('default', 'default'); // turn 1
battle.makeChoices('default', 'default'); // turn 2

console.log('\n=== Turn 3 ===\n');
battle.makeChoices('default', 'default'); // turn 3
