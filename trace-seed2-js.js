#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

// Use seed 2 which has largest divergence
const seedValue = 2;
const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));

const dex = Dex.mod('gen9');

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

// Detailed PRNG logging
let prngCallCount = 0;
const prngLog = [];

if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        const stack = new Error().stack.split('\n').slice(2, 5).join(' | ');
        prngLog.push({count: prngCallCount, value, stack, turn: battle.turn});
        if (prngCallCount <= 20) {
            console.log(`[JS PRNG #${prngCallCount}] value=${value} turn=${battle.turn}`);
        }
        return value;
    };
}

console.log('JS: Seed 2 battle created');

// Run only first 3 turns
for (let turnNum = 1; turnNum <= 3; turnNum++) {
    console.log(`\n=== JS: Making turn ${turnNum} choices ===`);
    const prngBefore = prngCallCount;

    battle.makeChoices('default', 'default');

    const prngAfter = prngCallCount;
    console.log(`JS: Turn ${turnNum} used ${prngAfter - prngBefore} PRNG calls (total: ${prngAfter})`);
}

console.log(`\nTotal PRNG calls: ${prngCallCount}`);

// Save detailed log
fs.writeFileSync('prng-trace-seed2-js.json', JSON.stringify(prngLog, null, 2));
console.log('Saved PRNG trace to prng-trace-seed2-js.json');
