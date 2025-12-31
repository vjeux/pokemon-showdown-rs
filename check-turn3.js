#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

// Get seed from command line, default to 2
const seedValue = parseInt(process.argv[2] || '2', 10);

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

// Run turns 1 and 2
for (let turnNum = 1; turnNum <= 2; turnNum++) {
    const logBefore = battle.log.length;
    battle.makeChoices('default', 'default');
    const turnLog = battle.log.slice(logBefore).join('\n');
    console.log(`\n=== Turn ${turnNum} Log ===`);
    console.log(turnLog);
}

// Now show detailed log for turn 3
console.log('\n=== Before Turn 3 ===');
console.log('Battle turn:', battle.turn);

const logBefore = battle.log.length;

// Make turn 3
console.log('\n=== Making Turn 3 choices ===');
battle.makeChoices('default', 'default');

console.log('\n=== After Turn 3 ===');
console.log('Battle turn:', battle.turn);

// Get only the new log entries for turn 3
const turn3Log = battle.log.slice(logBefore).join('\n');

console.log('\n=== Turn 3 Log ===');
console.log(turn3Log);
