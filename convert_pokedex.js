const fs = require('fs');

// Import the Pokedex from the JavaScript distribution
const {Pokedex} = require('./pokemon-showdown-js/dist/data/pokedex');

// Convert to JSON
const json = JSON.stringify(Pokedex, null, 2);

// Write to data/species.json
fs.writeFileSync('data/species.json', json);

console.log('Converted', Object.keys(Pokedex).length, 'species to data/species.json');
