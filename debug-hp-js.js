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
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        return value;
    };
}

console.log("Turn-by-turn HP tracking for Cinderace (p2a):\n");

// Run until battle ends or max 1000 turns
let turnNum = 0;
const MAX_TURNS = 1000;

while (!battle.ended && turnNum < MAX_TURNS) {
    turnNum++;
    const prngBefore = prngCallCount;

    battle.makeChoices('default', 'default');

    const prngAfter = prngCallCount;

    // Log Cinderace HP after each turn
    const p2a = battle.sides[1].active[0];
    if (p2a && p2a.name.includes('Cinderace')) {
        console.log(`Turn ${turnNum}: Cinderace HP = ${p2a.hp}/${p2a.maxhp} (PRNG calls: ${prngAfter - prngBefore}, total: ${prngAfter})`);
    } else if (battle.sides[1].pokemon.some(p => p.name.includes('Cinderace'))) {
        // Cinderace is not active but exists in the team
        const cinderace = battle.sides[1].pokemon.find(p => p.name.includes('Cinderace'));
        if (cinderace.fainted) {
            console.log(`Turn ${turnNum}: Cinderace is FAINTED (PRNG calls: ${prngAfter - prngBefore}, total: ${prngAfter})`);
        } else {
            console.log(`Turn ${turnNum}: Cinderace HP = ${cinderace.hp}/${cinderace.maxhp} [not active] (PRNG calls: ${prngAfter - prngBefore}, total: ${prngAfter})`);
        }
    }

    if (battle.ended) {
        console.log(`\nBattle ended on turn ${turnNum}`);
        console.log(`Winner: ${battle.winner || 'none'}`);
        break;
    }
}
