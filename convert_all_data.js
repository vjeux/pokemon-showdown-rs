const fs = require('fs');

// Import all data from JavaScript distribution
const {Pokedex} = require('./pokemon-showdown-js/dist/data/pokedex');
const {Moves} = require('./pokemon-showdown-js/dist/data/moves');
const {Abilities} = require('./pokemon-showdown-js/dist/data/abilities');
const {Items} = require('./pokemon-showdown-js/dist/data/items');

// Write each to JSON
fs.writeFileSync('data/species.json', JSON.stringify(Pokedex, null, 2));
console.log('Converted', Object.keys(Pokedex).length, 'species');

fs.writeFileSync('data/moves.json', JSON.stringify(Moves, null, 2));
console.log('Converted', Object.keys(Moves).length, 'moves');

fs.writeFileSync('data/abilities.json', JSON.stringify(Abilities, null, 2));
console.log('Converted', Object.keys(Abilities).length, 'abilities');

fs.writeFileSync('data/items.json', JSON.stringify(Items, null, 2));
console.log('Converted', Object.keys(Items).length, 'items');

console.log('All data converted successfully!');
