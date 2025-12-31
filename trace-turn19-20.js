#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch to see moves and damage
const originalDamage = battle.actions.getDamage.bind(battle.actions);
battle.actions.getDamage = function(source, target, move, ...rest) {
    const result = originalDamage(source, target, move, ...rest);
    if (battle.turn === 19 || battle.turn === 20) {
        console.log(`[DAMAGE T${battle.turn}] ${source.name} -> ${target.name} with ${move.name}: ${result}`);
    }
    return result;
};

// Run turns 1-18
for (let i = 1; i < 19; i++) {
    battle.makeChoices('default', 'default');
}

console.log('\n=== TURN 19 ===');
battle.makeChoices('default', 'default');

console.log('\n=== TURN 20 ===');
battle.makeChoices('default', 'default');

console.log('\n=== Final State ===');
console.log('P1 Active:', battle.sides[0].active[0]?.name, battle.sides[0].active[0]?.hp);
console.log('P2 Active:', battle.sides[1].active[0]?.name, battle.sides[1].active[0]?.hp);
