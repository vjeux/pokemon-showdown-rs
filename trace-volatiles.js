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

function logVolatiles(label) {
    const p1 = battle.sides[0].active[0];
    console.log(`\n${label}`);
    console.log(`P1 (${p1.name}) volatiles:`, Object.keys(p1.volatiles));
    for (const id in p1.volatiles) {
        const vol = p1.volatiles[id];
        console.log(`  ${id}: duration=${vol.duration}, counter=${vol.counter}, data=`, vol);
    }
}

logVolatiles('=== Initial State ===');

// Run turn 1
battle.makeChoices('default', 'default');
logVolatiles('=== After Turn 1 ===');

// Run turn 2
battle.makeChoices('default', 'default');
logVolatiles('=== After Turn 2 ===');

// Run turn 3
battle.makeChoices('default', 'default');
logVolatiles('=== After Turn 3 ===');
