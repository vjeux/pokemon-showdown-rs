#!/usr/bin/env node

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dexNoFormat = Dex.mod('gen9');
const dexWithFormat = Dex.forFormat('gen9randombattle');

const allSpeciesNoFormat = Object.values(dexNoFormat.species.all());
const allSpeciesWithFormat = Object.values(dexWithFormat.species.all());

console.log('Species count without format:', allSpeciesNoFormat.length);
console.log('Species count with format:', allSpeciesWithFormat.length);
console.log('Difference:', allSpeciesNoFormat.length - allSpeciesWithFormat.length);

// Check first few species in each
console.log('\nFirst 10 species (no format):', allSpeciesNoFormat.slice(0, 10).map(s => s.name).join(', '));
console.log('\nFirst 10 species (with format):', allSpeciesWithFormat.slice(0, 10).map(s => s.name).join(', '));

// Check if Rust might have the no-format count
console.log('\nDoes Rust (1583) match no-format?', allSpeciesNoFormat.length === 1583);
