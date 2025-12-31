#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

const p1 = battle.sides[0].active[0];
console.log('=== Initial State ===');
console.log('Pokemon:', p1.name);
for (const move of p1.moveSlots) {
    console.log(`  ${move.id}: pp=${move.pp}, maxpp=${move.maxpp}`);
}

// Run turns and track PP
for (let turn = 1; turn <= 20; turn++) {
    battle.makeChoices('default', 'default');

    if (turn <= 6 || turn === 17 || turn === 18) {
        console.log(`\n=== After Turn ${turn} ===`);
        for (const move of p1.moveSlots) {
            console.log(`  ${move.id}: pp=${move.pp}, maxpp=${move.maxpp}`);
        }
    }
}
