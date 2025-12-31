#!/usr/bin/env node

const fs = require('fs');

const jsSpecies = fs.readFileSync('js-species-list.txt', 'utf8').trim().split('\n');
const rustSpecies = fs.readFileSync('rust-species-list.txt', 'utf8').trim().split('\n');

console.log('JavaScript species:', jsSpecies.length);
console.log('Rust species (filtered):', rustSpecies.length);
console.log('Difference:', jsSpecies.length - rustSpecies.length);

const jsSet = new Set(jsSpecies);
const rustSet = new Set(rustSpecies);

const inJsNotRust = jsSpecies.filter(s => !rustSet.has(s));
const inRustNotJs = rustSpecies.filter(s => !jsSet.has(s));

console.log('\n=== In JavaScript but NOT in Rust (' + inJsNotRust.length + ') ===');
inJsNotRust.forEach((s, i) => {
    console.log(`${i + 1}. ${s}`);
});

console.log('\n=== In Rust but NOT in JavaScript (' + inRustNotJs.length + ') ===');
inRustNotJs.forEach((s, i) => {
    console.log(`${i + 1}. ${s}`);
});
