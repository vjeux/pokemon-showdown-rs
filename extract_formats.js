const fs = require('fs');

// Load the compiled formats
const formats = require('./pokemon-showdown-js/dist/config/formats.js').Formats;

// Convert to a simpler format for JSON
const formatsArray = [];
for (const item of formats) {
    if (item.section) {
        // Section header - skip for now, we'll handle sections separately
        continue;
    }
    
    // Extract only the fields we need for validation
    const format = {
        name: item.name
    };
    
    // Optional string fields
    if (item.mod) format.mod = item.mod;
    if (item.team) format.team = item.team;
    if (item.gameType) format.gameType = item.gameType;
    if (item.desc) format.desc = item.desc;
    
    // Boolean fields
    if (item.debug !== undefined) format.debug = item.debug;
    if (item.rated !== undefined) format.rated = item.rated;
    if (item.searchShow !== undefined) format.searchShow = item.searchShow;
    if (item.challengeShow !== undefined) format.challengeShow = item.challengeShow;
    if (item.tournamentShow !== undefined) format.tournamentShow = item.tournamentShow;
    if (item.bestOfDefault !== undefined) format.bestOfDefault = item.bestOfDefault;
    
    // Array fields
    if (item.ruleset) format.ruleset = item.ruleset;
    if (item.banlist) format.banlist = item.banlist;
    if (item.restricted) format.restricted = item.restricted;
    if (item.unbanlist) format.unbanlist = item.unbanlist;
    
    formatsArray.push(format);
}

// Write to JSON file
fs.writeFileSync('data/formats.json', JSON.stringify(formatsArray, null, 2));
console.log('Extracted ' + formatsArray.length + ' formats to data/formats.json');
