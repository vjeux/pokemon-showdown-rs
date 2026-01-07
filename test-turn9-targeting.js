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

// Run to turn 9
let iteration = 0;
while (!battle.ended && battle.turn < 9) {
    battle.makeChoices('default', 'default');
    iteration++;
}

// Now at turn 9, check the targeting
const p1Pokemon = battle.p1.active[0];
const p2Pokemon = battle.p2.active[0];

console.log('=== Turn 9 State ===');
console.log('P1 Pokemon:', p1Pokemon.name, p1Pokemon.species.name);
console.log('P2 Pokemon:', p2Pokemon.name, p2Pokemon.species.name);
console.log('');

// Test adjacentAllies for both
console.log('P1 adjacentAllies():');
const p1Allies = p1Pokemon.adjacentAllies();
p1Allies.forEach((ally, i) => {
    console.log(`  [${i}]`, ally.name, ally.species.name, 'same as p1?', ally === p1Pokemon);
});
console.log('  Total:', p1Allies.length);
console.log('');

console.log('P2 adjacentAllies():');
const p2Allies = p2Pokemon.adjacentAllies();
p2Allies.forEach((ally, i) => {
    console.log(`  [${i}]`, ally.name, ally.species.name, 'same as p2?', ally === p2Pokemon);
});
console.log('  Total:', p2Allies.length);
console.log('');

// Test getMoveTargets for Earthquake
const earthquake = battle.dex.getActiveMove('earthquake');
console.log('P2 (Exeggutor) using Earthquake:');
const {targets: p2Targets} = p2Pokemon.getMoveTargets(earthquake, p1Pokemon);
console.log('Targets:');
p2Targets.forEach((t, i) => {
    console.log(`  [${i}]`, t.name, t.species.name, 'Side:', t.side.id);
});
console.log('  Total:', p2Targets.length);
console.log('');

console.log('P1 (Floragato) using Earthquake via Mirror Move (explicit target=p2):');
const {targets: p1Targets} = p1Pokemon.getMoveTargets(earthquake, p2Pokemon);
console.log('Targets:');
p1Targets.forEach((t, i) => {
    console.log(`  [${i}]`, t.name, t.species.name, 'Side:', t.side.id);
});
console.log('  Total:', p1Targets.length);
