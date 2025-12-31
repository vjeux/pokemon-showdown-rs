#!/usr/bin/env node

// Detailed test of team generation to find where Rust and JS diverge

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

console.log('=== Testing Team Generation Step by Step ===');
const prng = new PRNG([0, 0, 0, 1]);
const dex = Dex.forFormat('gen9randombattle');

// Get all species
const allSpecies = Object.values(dex.species.all());
console.log(`Total species count: ${allSpecies.length}`);

console.log('\n=== Generating first Pokemon ===');
const usedSpecies = [];
const availableSpecies = allSpecies.filter(s => !usedSpecies.includes(s.name));
console.log(`Available species count: ${availableSpecies.length}`);

// Sample first species
const species = prng.sample(availableSpecies);
console.log(`Selected species: ${species.name}`);
usedSpecies.push(species.name);

// Select level
const level = prng.random(50, 101);
console.log(`Selected level: ${level}`);

// Select ability
const ability = species.abilities['0'] || 'No Ability';
console.log(`Selected ability: ${ability}`);

// Get all items
const allItems = Object.keys(dex.items.all());
console.log(`\nTotal items count: ${allItems.length}`);

// Select item
const usedItems = [];
const availableItems = allItems.filter(i => !usedItems.includes(i) || i === '');
console.log(`Available items count: ${availableItems.length}`);
const item = prng.sample(availableItems) || '';
console.log(`Selected item: ${item}`);
if (item !== '') usedItems.push(item);

// Get all natures
const allNatures = Object.keys(dex.natures.all());
console.log(`\nTotal natures count: ${allNatures.length}`);

const nature = prng.sample(allNatures);
console.log(`Selected nature: ${nature}`);

// Gender
let gender = '';
if (species.genderRatio) {
	if (species.genderRatio.M > 0 && species.genderRatio.F > 0) {
		gender = prng.randomChance(1, 2) ? 'M' : 'F';
	} else if (species.genderRatio.M > 0) {
		gender = 'M';
	} else if (species.genderRatio.F > 0) {
		gender = 'F';
	}
}
console.log(`Selected gender: ${gender || 'N'}`);

// Moves
console.log('\n=== Selecting moves ===');
const allMoves = Object.keys(dex.moves.all());
console.log(`Total moves count: ${allMoves.length}`);

const moves = [];
for (let j = 0; j < 4; j++) {
	const move = prng.sample(allMoves) || 'tackle';
	console.log(`Move ${j+1}: ${move}`);
	if (!moves.includes(move)) {
		moves.push(move);
	}
}
console.log(`Final moves: ${moves.join(', ')}`);
