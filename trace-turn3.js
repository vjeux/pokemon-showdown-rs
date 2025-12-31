#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = 2;
const teamsFile = `teams-js-seed${seedValue}.json`;
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

const battle = new Battle({
    formatid: 'gen9randombattle',
    seed: [0, 0, 0, seedValue],
});

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

// Patch PRNG to track calls with stack traces
let prngCallCount = 0;
const prngCalls = [];
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        const stack = new Error().stack.split('\n').slice(2, 8).map(line => line.trim()).join('\n    ');
        prngCalls.push({
            num: prngCallCount,
            value: value,
            stack: stack
        });
        return value;
    };
}

// Run turn 1
battle.makeChoices('default', 'default');
console.log(`\n=== Turn 1 completed - PRNG calls: ${prngCallCount} ===\n`);
const turn1Calls = prngCallCount;

// Run turn 2
battle.makeChoices('default', 'default');
console.log(`\n=== Turn 2 completed - PRNG calls: ${prngCallCount - turn1Calls} ===\n`);
const turn2Calls = prngCallCount;

// Run turn 3
battle.makeChoices('default', 'default');
console.log(`\n=== Turn 3 completed - PRNG calls: ${prngCallCount - turn2Calls} ===\n`);

// Show turn 3 PRNG calls
console.log('Turn 3 PRNG calls:');
for (let i = turn2Calls; i < prngCallCount; i++) {
    const call = prngCalls[i];
    console.log(`\n[JS PRNG #${call.num}] = ${call.value}`);
    console.log('Stack:');
    console.log('    ' + call.stack);
}
