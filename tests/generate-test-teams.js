#!/usr/bin/env node

/**
 * Generate test teams for battle comparison
 *
 * This script generates random battle teams with a specific seed
 * and saves them to JSON files for both JavaScript and Rust tests.
 *
 * Usage: node tests/generate-test-teams.js [seed_number]
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');
const path = require('path');

const seedNum = parseInt(process.argv[2]) || 1;

console.log(`Generating test teams for seed ${seedNum}...`);

// Generate battle with specific seed
const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, seedNum]);

battle.setPlayer('p1', {name: 'Player 1', team: null});
battle.setPlayer('p2', {name: 'Player 2', team: null});

// Extract team data
const p1Team = battle.sides[0].pokemon.map(p => ({
    name: p.name,
    species: p.species.name,
    level: p.level,
    ability: p.ability,
    item: p.item,
    nature: p.set.nature || p.baseNature || 'serious',
    gender: p.gender === 'M' ? 'M' : p.gender === 'F' ? 'F' : '',
    moves: p.moves,
    evs: p.set.evs,
    ivs: p.set.ivs,
}));

const p2Team = battle.sides[1].pokemon.map(p => ({
    name: p.name,
    species: p.species.name,
    level: p.level,
    ability: p.ability,
    item: p.item,
    nature: p.set.nature || p.baseNature || 'serious',
    gender: p.gender === 'M' ? 'M' : p.gender === 'F' ? 'F' : '',
    moves: p.moves,
    evs: p.set.evs,
    ivs: p.set.ivs,
}));

const teams = {p1: p1Team, p2: p2Team};

// Write team files
const testDir = path.join(__dirname);
if (!fs.existsSync(testDir)) {
    fs.mkdirSync(testDir, { recursive: true });
}

fs.writeFileSync(path.join(testDir, `teams-seed${seedNum}-js.json`), JSON.stringify(teams, null, 2));

console.log(`✓ Generated teams for seed ${seedNum}`);
console.log(`  P1: ${p1Team[0].name} (${p1Team[0].species})`);
console.log(`  P2: ${p2Team[0].name} (${p2Team[0].species})`);
console.log(`  File: tests/teams-seed${seedNum}-js.json`);
