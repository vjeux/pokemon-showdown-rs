#!/usr/bin/env node

// Export teams to JSON for comparison

const fs = require('fs');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;
const {generateRandomTeams} = require('./generate-teams');

const prng = new PRNG([0, 0, 0, 1]);
const dex = Dex.mod('gen9');

console.log('Generating teams with seed [0, 0, 0, 1]...');
const teams = generateRandomTeams(prng, dex);

fs.writeFileSync('teams-js.json', JSON.stringify(teams, null, 2));
console.log('Teams exported to teams-js.json');
console.log('P1 team:', teams.p1.map(p => p.name).join(', '));
console.log('P2 team:', teams.p2.map(p => p.name).join(', '));
