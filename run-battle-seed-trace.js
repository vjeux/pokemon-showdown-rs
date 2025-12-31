#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

// Get seed from command line, default to 1
const seedValue = parseInt(process.argv[2] || '1', 10);

// Load the teams we generated
const teamsFile = seedValue === 1 ? 'teams-js.json' : `teams-js-seed${seedValue}.json`;
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

const dex = Dex.mod('gen9');

// Create battle with seed
const battle = new Battle({
    formatid: 'gen9randombattle',
    seed: [0, 0, 0, seedValue],
});

// Add players with teams
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

// Patch PRNG to track calls with values
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        if (prngCallCount <= 50) {
            console.log(`[JS PRNG #${prngCallCount}] value=${value}`);
        }
        return value;
    };
}

console.log(`JS: Seed ${seedValue} - Battle created`);

// Run 3 turns
for (let turnNum = 1; turnNum <= 3; turnNum++) {
    const prngBefore = prngCallCount;
    battle.makeChoices('default', 'default');
    const prngAfter = prngCallCount;

    console.log(`JS: Seed ${seedValue} Turn ${turnNum} - PRNG calls: ${prngAfter - prngBefore} (total: ${prngAfter})`);
}

console.log(`JS: Seed ${seedValue} - Total PRNG calls: ${prngCallCount}`);
