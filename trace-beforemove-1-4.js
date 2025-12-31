#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch randomChance to catch the 1/4 call
const originalRandomChance = battle.randomChance.bind(battle);
battle.randomChance = function(numerator, denominator) {
    if (battle.turn === 20 && numerator === 1 && denominator === 4) {
        console.log('\n=== FOUND 1/4 randomChance in turn 20 ===');
        console.log('Effect:', battle.effect?.name || battle.effect?.id || 'unknown');
        console.log('EffectHolder:', battle.effectState?.target?.name || 'unknown');
        console.log('Stack:', new Error().stack);
    }
    return originalRandomChance(numerator, denominator);
};

// Run battle
for (let i = 1; i <= 20; i++) {
    battle.makeChoices('default', 'default');
}
