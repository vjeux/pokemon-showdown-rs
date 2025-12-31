#!/usr/bin/env node

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');
const prng = new PRNG([0, 0, 0, 1]);

let allSpecies = Object.values(dex.species.all());
allSpecies.sort((a, b) => a.name.localeCompare(b.name));

console.log('Total species:', allSpecies.length);
console.log('\n=== First PRNG call (for species selection) ===');
const rawValue = prng.random();  // Get raw [0,1) value
console.log('Raw random value:', rawValue);

const index = Math.floor(rawValue * allSpecies.length);
console.log('Calculated index:', index);
console.log('Species at that index:', allSpecies[index].name);

console.log('\nSpecies around that index:');
for (let i = Math.max(0, index - 2); i <= Math.min(allSpecies.length - 1, index + 2); i++) {
    console.log(`  [${i}]: ${allSpecies[i].name}${i === index ? ' <-- selected' : ''}`);
}
