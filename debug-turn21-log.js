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

// Patch PRNG to track calls (without stack traces)
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        if (prngCallCount >= 69 && prngCallCount <= 77) {
            console.log(`[JS PRNG] Call #${prngCallCount} -> ${value}`);
        }
        return value;
    };
}

// Run until battle ends or max 1000 turns
let turnNum = 0;
const MAX_TURNS = 1000;

while (!battle.ended && turnNum < MAX_TURNS) {
    turnNum++;
    const prngBefore = prngCallCount;

    // Capture battle log for turns 19-22
    if (turnNum >= 19 && turnNum <= 22) {
        console.log(`\n========== TURN ${turnNum} ==========`);

        // Get log before this turn
        const logBefore = battle.log.length;

        battle.makeChoices('default', 'default');

        // Print new log entries
        for (let i = logBefore; i < battle.log.length; i++) {
            console.log(battle.log[i]);
        }

        const prngAfter = prngCallCount;
        console.log(`[PRNG calls this turn: ${prngAfter - prngBefore}]`);
    } else {
        battle.makeChoices('default', 'default');
    }

    if (battle.ended) {
        console.log(`\nBattle ended on turn ${turnNum}`);
        console.log(`Winner: ${battle.winner || 'none'}`);
        break;
    }
}
