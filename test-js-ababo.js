#!/usr/bin/env node

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;
const dex = Dex.mod('gen9');

const allSpecies = Object.values(dex.species.all());
const sorted = [...allSpecies].sort((a, b) => a.name.localeCompare(b.name));

console.log('Total species:', sorted.length);
console.log('First 20 sorted species:');
sorted.slice(0, 20).forEach((s, i) => console.log(`  ${i}: ${s.name}`));

// Check if Ababo exists
const ababo = dex.species.get('Ababo');
console.log('\nAbabo exists in JS?', !!ababo);
if (ababo) {
    console.log('Ababo data:', ababo);
}
