#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch to see when moves succeed/fail
const originalUseMove = battle.actions.useMove.bind(battle.actions);
battle.actions.useMove = function(move, pokemon, target, ...rest) {
    console.log(`[USE_MOVE] ${pokemon.name} uses ${move.name || move}`);
    const result = originalUseMove(move, pokemon, target, ...rest);
    console.log(`[USE_MOVE] Result: ${result}`);
    return result;
};

// Run turns
battle.makeChoices('default', 'default'); // turn 1
battle.makeChoices('default', 'default'); // turn 2

const p1 = battle.sides[0].active[0];
console.log('\n=== Before Turn 3 ===');
console.log('P1 volatiles:', Object.keys(p1.volatiles));
for (const id in p1.volatiles) {
    const vol = p1.volatiles[id];
    console.log(`  ${id}: duration=${vol.duration}, counter=${vol.counter}`);
}

battle.makeChoices('default', 'default'); // turn 3

console.log('\n=== After Turn 3 ===');
console.log('P1 volatiles:', Object.keys(p1.volatiles));
for (const id in p1.volatiles) {
    const vol = p1.volatiles[id];
    console.log(`  ${id}: duration=${vol.duration}, counter=${vol.counter}`);
}
