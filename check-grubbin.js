#!/usr/bin/env node

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');

let allSpecies = Object.values(dex.species.all());
allSpecies.sort((a, b) => a.name.localeCompare(b.name));

const grubbin = allSpecies.find(s => s.name === 'Grubbin');

console.log('Grubbin data:');
console.log('- Name:', grubbin.name);
console.log('- Gender ratio:', JSON.stringify(grubbin.genderRatio, null, 2));
console.log('- Has both genders?', grubbin.genderRatio && grubbin.genderRatio.M > 0 && grubbin.genderRatio.F > 0);
