#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Run turns 1-17
for (let i = 1; i < 18; i++) {
    battle.makeChoices('default', 'default');
}

console.log('\n=== Before Turn 18 ===');
const p1 = battle.sides[0].active[0];
console.log('P1:', p1.name);
for (const move of p1.moveSlots) {
    console.log(`  ${move.id}: disabled=${move.disabled}, pp=${move.pp}/${move.maxpp}`);
}
console.log('Volatiles:', Object.keys(p1.volatiles));

console.log('\n=== Turn 18 ===');
battle.makeChoices('default', 'default');

console.log('\n=== After Turn 18 ===');
for (const move of p1.moveSlots) {
    console.log(`  ${move.id}: disabled=${move.disabled}, pp=${move.pp}/${move.maxpp}`);
}
console.log('Volatiles:', Object.keys(p1.volatiles));
