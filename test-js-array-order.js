#!/usr/bin/env node

// Check the order of species/moves/items/natures in JavaScript

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.forFormat('gen9randombattle');

const allSpecies = Object.values(dex.species.all());
const allMoves = Object.values(dex.moves.all()).map(m => m.id);
const allItems = Object.values(dex.items.all()).map(i => i.id);
const allNatures = Object.values(dex.natures.all()).map(n => n.id);

console.log('=== JavaScript Array Orders ===\n');
console.log(`Total species: ${allSpecies.length}`);
console.log(`First 10 species: ${allSpecies.slice(0, 10).map(s => s.name).join(', ')}\n`);

console.log(`Total moves: ${allMoves.length}`);
console.log(`First 10 moves: ${allMoves.slice(0, 10).join(', ')}\n`);

console.log(`Total items: ${allItems.length}`);
console.log(`First 10 items: ${allItems.slice(0, 10).join(', ')}\n`);

console.log(`Total natures: ${allNatures.length}`);
console.log(`All natures: ${allNatures.join(', ')}`);
