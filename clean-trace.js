#!/usr/bin/env node

// Clean trace - count actual PRNG next() calls only

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;
const {generateRandomEVs} = require('./generate-teams');

const dex = Dex.mod('gen9');
const prng = new PRNG([0, 0, 0, 1]);

let allSpecies = Object.values(dex.species.all());
allSpecies.sort((a, b) => a.name.localeCompare(b.name));

const allMoves = Object.values(dex.moves.all()).map(m => m.id);
const allItems = Object.values(dex.items.all()).map(i => i.id);
const allNatures = Object.values(dex.natures.all()).map(n => n.id);

// Count actual next() calls
let nextCallCount = 0;
const originalNext = prng.rng.next.bind(prng.rng);
prng.rng.next = function() {
    nextCallCount++;
    return originalNext();
};

// 1. Species
const species = prng.sample(allSpecies);
console.log(`After species: ${nextCallCount} calls => ${species.name}`);

// 2. Level
const level = prng.random(50, 101);
console.log(`After level: ${nextCallCount} calls => ${level}`);

// 3. Item
const item = prng.sample(allItems) || '';
console.log(`After item: ${nextCallCount} calls => ${item}`);

// 4. Nature
const nature = prng.sample(allNatures);
console.log(`After nature: ${nextCallCount} calls => ${nature}`);

// 5. Gender
let gender = '';
if (species.genderRatio && species.genderRatio.M > 0 && species.genderRatio.F > 0) {
    gender = prng.randomChance(1, 2) ? 'M' : 'F';
}
console.log(`After gender: ${nextCallCount} calls => ${gender || 'N'}`);

// 6. Moves
const moves = [];
for (let j = 0; j < 4; j++) {
    const move = prng.sample(allMoves);
    if (!moves.includes(move)) {
        moves.push(move);
    }
}
while (moves.length < 4) {
    moves.push('tackle');
}
console.log(`After moves: ${nextCallCount} calls => ${moves.join(', ')}`);

// 7. EVs
const evs = generateRandomEVs(prng);
console.log(`After EVs: ${nextCallCount} calls => ${JSON.stringify(evs)}`);

// 8. IVs
const ivs = {
    hp: prng.random(0, 32),
    atk: prng.random(0, 32),
    def: prng.random(0, 32),
    spa: prng.random(0, 32),
    spd: prng.random(0, 32),
    spe: prng.random(0, 32),
};
console.log(`After IVs: ${nextCallCount} calls => ${JSON.stringify(ivs)}`);

console.log(`\n=== TOTAL: ${nextCallCount} next() calls ===`);

// Next Pokemon
const usedSpecies = [species.name];
const availableSpecies2 = allSpecies.filter(s => !usedSpecies.includes(s.name));
const species2 = prng.sample(availableSpecies2);
console.log(`\nPokemon #2 (call ${nextCallCount}): ${species2.name}`);
