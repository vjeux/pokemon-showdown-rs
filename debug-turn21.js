#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = 1;
const teams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

const dex = Dex.mod('gen9');

// Create battle
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

// Track PRNG calls
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        if (prngCallCount >= 69 && prngCallCount <= 80) {
            console.log(`[JS PRNG #${prngCallCount}] value=${value}`);
        }
        return value;
    };
}

console.log('Running turns 1-22...');

// Run to turn 22
for (let turnNum = 1; turnNum <= 22; turnNum++) {
    const prngBefore = prngCallCount;

    battle.makeChoices('default', 'default');

    const prngAfter = prngCallCount;
    const callsThisTurn = prngAfter - prngBefore;

    console.log(`JS: Turn ${turnNum} - PRNG calls: ${callsThisTurn} (total: ${prngAfter})`);

    if (battle.ended) {
        console.log(`Battle ended on turn ${turnNum}`);
        break;
    }
}

console.log(`\nFinal PRNG call count: ${prngCallCount}`);
