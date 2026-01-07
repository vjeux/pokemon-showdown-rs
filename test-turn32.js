#!/usr/bin/env node

const {Battle} = require('../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const seedNum = 20;
const teamsFile = `/tmp/teams-seed${seedNum}-js.json`;
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, seedNum]);

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({...p})),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({...p})),
});

// Run to turn 31
let iteration = 0;
while (!battle.ended && battle.turn < 32) {
    battle.makeChoices('default', 'default');
    iteration++;
}

console.log('=== Before Turn 32 ===');
console.log('P1 active:', battle.p1.active[0].name, battle.p1.active[0].species.name, `(${battle.p1.active[0].hp}/${battle.p1.active[0].maxhp})`);
console.log('P2 active:', battle.p2.active[0].name, battle.p2.active[0].species.name, `(${battle.p2.active[0].hp}/${battle.p2.active[0].maxhp})`);

// Execute turn 32
console.log('\n=== Executing Turn 32 ===');
const p2HpBefore = battle.p2.active[0].hp;
battle.makeChoices('default', 'default');

console.log('\n=== After Turn 32 ===');
console.log('P1 active:', battle.p1.active[0].name, `(${battle.p1.active[0].hp}/${battle.p1.active[0].maxhp})`);
console.log('P2 active:', battle.p2.active[0].name, `(${battle.p2.active[0].hp}/${battle.p2.active[0].maxhp})`);
console.log('P2 HP change:', p2HpBefore - battle.p2.active[0].hp);
