#!/usr/bin/env node

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;
const dex = Dex.forFormat('gen9randombattle');

const allSpecies = Object.values(dex.species.all());
console.log('Total species: ', allSpecies.length);
console.log('First 20 species (before sort):');
allSpecies.slice(0, 20).forEach((s, i) => console.log(`  ${i}: ${s.name}`));

// Note: JavaScript arrays from dex.species.all() are already ordered
console.log('\nAre species already sorted by name?');
const sorted = [...allSpecies].sort((a, b) => a.name.localeCompare(b.name));
console.log('First species in original:', allSpecies[0].name);
console.log('First species if sorted:', sorted[0].name);
console.log('Match?', allSpecies[0].name === sorted[0].name);

console.log('\nFirst 20 species if sorted:');
sorted.slice(0, 20).forEach((s, i) => console.log(`  ${i}: ${s.name}`));
