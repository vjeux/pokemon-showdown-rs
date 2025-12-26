const fs = require('fs');

// Load the compiled formats data
const FormatsData = require('./pokemon-showdown-js/dist/data/formats-data.js').FormatsData;

// Write to JSON file
fs.writeFileSync('data/formats-data.json', JSON.stringify(FormatsData, null, 2));

const count = Object.keys(FormatsData).length;
console.log('Extracted ' + count + ' species formats data entries to data/formats-data.json');
