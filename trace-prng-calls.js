#!/usr/bin/env node

// Trace PRNG calls during team generation

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');
const prng = new PRNG([0, 0, 0, 1]);

let allSpecies = Object.values(dex.species.all());
allSpecies.sort((a, b) => a.name.localeCompare(b.name));

const allMoves = Object.values(dex.moves.all()).map(m => m.id);
const allItems = Object.values(dex.items.all()).map(i => i.id);
const allNatures = Object.values(dex.natures.all()).map(n => n.id);

console.log('Total species:', allSpecies.length);
console.log('First species:', allSpecies[0].name);

let callCount = 0;

// Wrap PRNG methods to log calls
const originalRandom = prng.random.bind(prng);
const originalSample = prng.sample.bind(prng);
const originalRandomChance = prng.randomChance.bind(prng);

prng.random = function(...args) {
    const result = originalRandom(...args);
    callCount++;
    console.log(`[${callCount}] random(${args.join(',')}) = ${result}`);
    return result;
};

prng.sample = function(arr) {
    const result = originalSample(arr);
    callCount++;
    console.log(`[${callCount}] sample(array[${arr.length}]) = ${result.name || result}`);
    return result;
};

prng.randomChance = function(num, denom) {
    const result = originalRandomChance(num, denom);
    callCount++;
    console.log(`[${callCount}] randomChance(${num},${denom}) = ${result}`);
    return result;
};

console.log('\n=== Generating first Pokemon ===\n');

// Select species
const species = prng.sample(allSpecies);
console.log(`Selected species: ${species.name}`);

// Select level
const level = prng.random(50, 101);
console.log(`Selected level: ${level}`);

// Select ability
const ability = species.abilities['0'] || 'No Ability';
console.log(`Selected ability: ${ability} (no PRNG call)`);

// Select item
const item = prng.sample(allItems) || '';
console.log(`Selected item: ${item}`);

// Select nature
const nature = prng.sample(allNatures);
console.log(`Selected nature: ${nature}`);

// Gender
let gender = '';
if (species.genderRatio) {
    if (species.genderRatio.M > 0 && species.genderRatio.F > 0) {
        gender = prng.randomChance(1, 2) ? 'M' : 'F';
    }
}
console.log(`Selected gender: ${gender || 'N'}`);

// Moves
console.log('\nSelecting 4 moves:');
const moves = [];
for (let j = 0; j < 4; j++) {
    const move = prng.sample(allMoves);
    if (!moves.includes(move)) {
        moves.push(move);
    }
}
console.log(`Moves: ${moves.join(', ')}`);

console.log(`\nTotal PRNG calls for first Pokemon: ${callCount}`);
