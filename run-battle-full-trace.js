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

// Patch PRNG to track calls
let prngCallCount = 0;
let turnPrngCalls = [];
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        return value;
    };
}

console.log(`JS: Seed ${seedValue} - Battle created`);

// Run until battle ends or max 1000 turns
let turnNum = 0;
const MAX_TURNS = 1000;

while (!battle.ended && turnNum < MAX_TURNS) {
    turnNum++;
    const prngBefore = prngCallCount;

    try {
        battle.makeChoices('default', 'default');
    } catch (e) {
        console.log(`JS: Seed ${seedValue} Turn ${turnNum} - Error: ${e.message}`);
        break;
    }

    const prngAfter = prngCallCount;
    const callsThisTurn = prngAfter - prngBefore;

    turnPrngCalls.push(callsThisTurn);
    console.log(`JS: Turn ${turnNum} - PRNG calls: ${callsThisTurn} (total: ${prngAfter})`);

    if (battle.ended) {
        console.log(`JS: Battle ended on turn ${turnNum}`);
        console.log(`JS: Winner: ${battle.winner || 'none'}`);
        break;
    }
}

console.log(`JS: Total turns: ${turnNum}`);
console.log(`JS: Total PRNG calls: ${prngCallCount}`);
console.log(`JS: PRNG calls per turn: ${JSON.stringify(turnPrngCalls)}`);

// Save trace to file
const trace = {
    seed: seedValue,
    turns: turnNum,
    winner: battle.winner || null,
    totalPrngCalls: prngCallCount,
    prngCallsPerTurn: turnPrngCalls,
};

fs.writeFileSync(`trace-js-seed${seedValue}.json`, JSON.stringify(trace, null, 2));
console.log(`JS: Trace saved to trace-js-seed${seedValue}.json`);
