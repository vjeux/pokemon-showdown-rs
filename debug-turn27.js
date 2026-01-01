#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = 1;
const teams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

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

// Patch PRNG to track calls
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        if (prngCallCount >= 105 && prngCallCount <= 112) {
            console.log(`[JS PRNG] Call #${prngCallCount} -> ${value}`);
            console.trace('Trace');
        }
        return value;
    };
}

// Run to turn 28
for (let turn = 1; turn <= 28; turn++) {
    if (turn >= 26 && turn <= 28) {
        console.log(`\n========== TURN ${turn} ==========`);

        const logBefore = battle.log.length;
        const prngBefore = prngCallCount;
        battle.makeChoices('default', 'default');
        const prngAfter = prngCallCount;

        for (let i = logBefore; i < battle.log.length; i++) {
            console.log(battle.log[i]);
        }
        console.log(`PRNG calls this turn: ${prngAfter - prngBefore} (total: ${prngAfter})`);
    } else {
        battle.makeChoices('default', 'default');
    }

    if (battle.ended) break;
}
