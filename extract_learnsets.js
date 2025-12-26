const fs = require('fs');

// Load the compiled learnsets
const Learnsets = require('./pokemon-showdown-js/dist/data/learnsets.js').Learnsets;

// Convert to JSON
const learnsets_json = {};
for (const speciesid in Learnsets) {
    const entry = Learnsets[speciesid];
    const json_entry = {};
    
    if (entry.learnset) {
        json_entry.learnset = entry.learnset;
    }
    
    if (entry.eventData) {
        json_entry.eventData = entry.eventData;
    }
    
    if (entry.eventOnly) {
        json_entry.eventOnly = entry.eventOnly;
    }
    
    if (entry.encounters) {
        json_entry.encounters = entry.encounters;
    }
    
    learnsets_json[speciesid] = json_entry;
}

// Write to JSON file
fs.writeFileSync('data/learnsets.json', JSON.stringify(learnsets_json, null, 2));

const count = Object.keys(learnsets_json).length;
console.log('Extracted ' + count + ' learnsets to data/learnsets.json');
