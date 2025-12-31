#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = parseInt(process.argv[2] || '1', 10);
const targetTurn = parseInt(process.argv[3] || '21', 10);

// Load the teams
const teamsFile = seedValue === 1 ? 'teams-js.json' : `teams-js-seed${seedValue}.json`;
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

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

// Track PRNG calls with stack traces
let prngCallCount = 0;
let prngCalls = [];

if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;

        // Capture stack trace
        const err = new Error();
        const stack = err.stack.split('\n').slice(2, 10).join('\n');

        prngCalls.push({
            call: prngCallCount,
            value: value,
            stack: stack
        });

        return value;
    };
}

console.log(`JS: Running battle to turn ${targetTurn}`);

// Run up to target turn
for (let turnNum = 1; turnNum <= targetTurn; turnNum++) {
    const prngBefore = prngCallCount;

    battle.makeChoices('default', 'default');

    const prngAfter = prngCallCount;
    const callsThisTurn = prngAfter - prngBefore;

    console.log(`JS: Turn ${turnNum} - PRNG calls: ${callsThisTurn} (total: ${prngAfter})`);

    if (turnNum === targetTurn) {
        console.log(`\n=== TURN ${targetTurn} PRNG CALLS ===`);
        const turnCalls = prngCalls.slice(prngBefore, prngAfter);
        turnCalls.forEach((call, idx) => {
            console.log(`\nCall ${idx + 1}/${callsThisTurn} (global #${call.call}):`);
            console.log(`  Value: ${call.value}`);
            console.log(`  Stack:\n${call.stack}`);
        });
    }

    if (battle.ended) {
        console.log(`Battle ended on turn ${turnNum}`);
        break;
    }
}
