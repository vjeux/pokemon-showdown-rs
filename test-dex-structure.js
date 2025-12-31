#!/usr/bin/env node

// Check how the dex methods return data

const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.forFormat('gen9randombattle');

console.log('=== Checking Dex Data Structures ===\n');

// Check species
const allSpecies = dex.species.all();
const speciesKeys = Object.keys(allSpecies);
console.log(`Species: First 10 keys: ${speciesKeys.slice(0, 10).join(', ')}`);
console.log(`Species: Sample entry for key '${speciesKeys[0]}':`,  allSpecies[speciesKeys[0]].name);

// Check moves
const allMoves = dex.moves.all();
const moveKeys = Object.keys(allMoves);
console.log(`\nMoves: First 10 keys: ${moveKeys.slice(0, 10).join(', ')}`);
console.log(`Moves: Sample entry for key '${moveKeys[0]}':`, allMoves[moveKeys[0]].name);

// Check items
const allItems = dex.items.all();
const itemKeys = Object.keys(allItems);
console.log(`\nItems: First 10 keys: ${itemKeys.slice(0, 10).join(', ')}`);
console.log(`Items: Sample entry for key '${itemKeys[0]}':`, allItems[itemKeys[0]].name);

// Check natures
const allNatures = dex.natures.all();
const natureKeys = Object.keys(allNatures);
console.log(`\nNatures: First 10 keys: ${natureKeys.slice(0, 10).join(', ')}`);
console.log(`Natures: Sample entry for key '${natureKeys[0]}':`, allNatures[natureKeys[0]].name);

console.log('\n=== Checking if IDs match names ===');
console.log(`Move '${moveKeys[0]}' has name '${allMoves[moveKeys[0]].name}' and id '${allMoves[moveKeys[0]].id}'`);
console.log(`Item '${itemKeys[0]}' has name '${allItems[itemKeys[0]].name}' and id '${allItems[itemKeys[0]].id}'`);
