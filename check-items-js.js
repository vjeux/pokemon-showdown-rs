#!/usr/bin/env node

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');

const allItems = Object.values(dex.items.all()).map(i => i.id);
allItems.sort();

console.log('Total items:', allItems.length);
console.log('First 20 items:', allItems.slice(0, 20));
console.log('');

// Find specific items
console.log('Index of firiumz:', allItems.indexOf('firiumz'));
console.log('Index of fightiniumz:', allItems.indexOf('fightiniumz'));
