const fs = require('fs');

// Import all data from JavaScript distribution
const {Pokedex} = require('./pokemon-showdown-js/dist/data/pokedex');
const {Moves} = require('./pokemon-showdown-js/dist/data/moves');
const {Abilities} = require('./pokemon-showdown-js/dist/data/abilities');
const {Items} = require('./pokemon-showdown-js/dist/data/items');

// Helper to convert an object by manually copying all properties
function deepCopy(obj) {
    if (obj === null || typeof obj !== 'object') return obj;
    if (Array.isArray(obj)) return obj.map(deepCopy);
    
    const result = {};
    for (const key in obj) {
        // Get the property value directly, even if non-enumerable
        const value = obj[key];
        if (value !== undefined) {
            result[key] = deepCopy(value);
        }
    }
    // Also get all own properties including non-enumerable
    Object.getOwnPropertyNames(obj).forEach(prop => {
        if (result[prop] === undefined && obj[prop] !== undefined) {
            result[prop] = deepCopy(obj[prop]);
        }
    });
    return result;
}

function convertData(data) {
    const result = {};
    for (const key in data) {
        result[key] = deepCopy(data[key]);
    }
    return result;
}

// Write each to JSON
fs.writeFileSync('data/species.json', JSON.stringify(convertData(Pokedex), null, 2));
console.log('Converted', Object.keys(Pokedex).length, 'species');

fs.writeFileSync('data/moves.json', JSON.stringify(convertData(Moves), null, 2));
console.log('Converted', Object.keys(Moves).length, 'moves');

fs.writeFileSync('data/abilities.json', JSON.stringify(convertData(Abilities), null, 2));
console.log('Converted', Object.keys(Abilities).length, 'abilities');

fs.writeFileSync('data/items.json', JSON.stringify(convertData(Items), null, 2));
console.log('Converted', Object.keys(Items).length, 'items');

console.log('All data converted successfully!');
