#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch to see moves
const originalUseMove = battle.actions.useMove.bind(battle.actions);
battle.actions.useMove = function(move, pokemon, target, ...rest) {
    console.log(`Turn ${battle.turn}: ${pokemon.name} uses ${move.name || move}`);
    return originalUseMove(move, pokemon, target, ...rest);
};

// Run turns 1-20
for (let i = 1; i <= 20; i++) {
    battle.makeChoices('default', 'default');
}
