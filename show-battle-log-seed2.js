#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = 2;
const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));

const dex = Dex.mod('gen9');

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

// Run 3 turns and show battle log
for (let turnNum = 1; turnNum <= 3; turnNum++) {
    console.log(`\n========== TURN ${turnNum} ==========`);
    battle.makeChoices('default', 'default');

    // Show recent battle log
    const recentLog = battle.log.slice(-20).join('\n');
    console.log(recentLog);
}
