#!/usr/bin/env node

// Compare species between JavaScript and Rust to find the 68 missing ones

const fs = require('fs');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');
const jsSpecies = Object.values(dex.species.all()).map(s => s.name);
jsSpecies.sort();

console.log('JavaScript species count:', jsSpecies.length);

// Read Rust species from the data file
const rustSpeciesData = JSON.parse(fs.readFileSync('./data/species.json', 'utf8'));
const rustSpecies = Object.values(rustSpeciesData).map(s => s.name).filter(Boolean);
rustSpecies.sort();

console.log('Rust species count:', rustSpecies.length);
console.log('Difference:', rustSpecies.length - jsSpecies.length);

// Find species in Rust but not in JS
const jsSet = new Set(jsSpecies);
const extraInRust = rustSpecies.filter(s => !jsSet.has(s));

console.log('\n=== Species in Rust but not in JavaScript (' + extraInRust.length + ') ===');
extraInRust.forEach((s, i) => {
    console.log(`${i + 1}. ${s}`);
});

// Find species in JS but not in Rust
const rustSet = new Set(rustSpecies);
const extraInJS = jsSpecies.filter(s => !rustSet.has(s));

if (extraInJS.length > 0) {
    console.log('\n=== Species in JavaScript but not in Rust (' + extraInJS.length + ') ===');
    extraInJS.forEach((s, i) => {
        console.log(`${i + 1}. ${s}`);
    });
}

// Save the JS species list
fs.writeFileSync('js-species-list.txt', jsSpecies.join('\n'));
console.log('\nSaved JavaScript species list to js-species-list.txt');
