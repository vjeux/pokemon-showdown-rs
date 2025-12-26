const fs = require('fs');

// Read the TypeScript file
const content = fs.readFileSync('pokemon-showdown-js/data/aliases.ts', 'utf8');

// Extract the object content
const match = content.match(/export const Aliases[^=]*=\s*(\{[\s\S]*\});/);
if (!match) {
    console.error('Could not find Aliases object');
    process.exit(1);
}

// Use eval to parse the object (safe since it's our own data)
const aliases = eval('(' + match[1] + ')');

// Write as JSON
fs.writeFileSync('data/aliases.json', JSON.stringify(aliases, null, 2));

console.log('Created data/aliases.json with', Object.keys(aliases).length, 'aliases');
