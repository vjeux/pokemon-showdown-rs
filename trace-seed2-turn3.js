#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = 2;
const teamsFile = 'teams-js-seed2.json';
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

const battle = new Battle({
    formatid: 'gen9randombattle',
    seed: [0, 0, 0, seedValue],
});

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({
        name: p.name, species: p.species, level: p.level,
        ability: p.ability, item: p.item, nature: p.nature,
        gender: p.gender, moves: p.moves, evs: p.evs, ivs: p.ivs,
    })),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({
        name: p.name, species: p.species, level: p.level,
        ability: p.ability, item: p.item, nature: p.nature,
        gender: p.gender, moves: p.moves, evs: p.evs, ivs: p.ivs,
    })),
});

let prngCallCount = 0;
let turnNum = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        if (turnNum === 3) {
            const stack = new Error().stack.split('\n').slice(2, 6).map(s => s.trim()).join(' <- ');
            console.log(`[JS Turn 3 PRNG #${prngCallCount}] value=${value} stack=${stack}`);
        }
        return value;
    };
}

console.log(`JS: Seed ${seedValue} - Battle created`);

// Run through turns 1-3
for (turnNum = 1; turnNum <= 3; turnNum++) {
    const prngBefore = prngCallCount;
    battle.makeChoices('default', 'default');
    const prngAfter = prngCallCount;
    
    console.log(`JS: Seed ${seedValue} Turn ${turnNum} - PRNG calls: ${prngAfter - prngBefore} (total: ${prngAfter})`);
}
