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

// Run turn 1
battle.makeChoices('default', 'default');
// Run turn 2
battle.makeChoices('default', 'default');

console.log('\n=== Starting Turn 3 ===\n');

// Check what moves are being used
const p1 = battle.sides[0].active[0];
const p2 = battle.sides[1].active[0];

console.log(`P1 (${p1.name}) volatiles:`, Object.keys(p1.volatiles));
console.log(`P1 moves:`, p1.moves.map(m => m));
console.log(`P2 (${p2.name}) volatiles:`, Object.keys(p2.volatiles));

// Run turn 3 with logging
battle.makeChoices('default', 'default');

console.log('\n=== Turn 3 completed ===\n');
