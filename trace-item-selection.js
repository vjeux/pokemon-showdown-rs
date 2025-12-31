#!/usr/bin/env node

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');
const prng = new PRNG([0, 0, 0, 1]);

let allSpecies = Object.values(dex.species.all());
allSpecies.sort((a, b) => a.name.localeCompare(b.name));

const allItems = Object.values(dex.items.all()).map(i => i.id);
allItems.sort();

// Call 1: species
prng.sample(allSpecies);

// Call 2: level
prng.random(50, 101);

// Call 3: item
console.log('All items length:', allItems.length);
const item = prng.sample(allItems);
console.log('Selected item:', item);
console.log('Index of selected item:', allItems.indexOf(item));
