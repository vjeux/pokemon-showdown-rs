#!/usr/bin/env node

/**
 * Analyze minimized seed files to find patterns in bugs
 *
 * Outputs:
 * - Unique moves with which seeds they appear in
 * - Unique items with which seeds they appear in
 * - Unique abilities with which seeds they appear in
 */

const fs = require('fs');
const path = require('path');

const MINIMIZED_DIR = path.join(__dirname, 'minimized');

// Maps to track occurrences
const moves = new Map();      // move -> [seeds]
const items = new Map();      // item -> [seeds]
const abilities = new Map();  // ability -> [seeds]

// Read all minimized files
const files = fs.readdirSync(MINIMIZED_DIR)
  .filter(f => f.endsWith('.json') && f.startsWith('seed'));

console.log(`Analyzing ${files.length} minimized seed files...\n`);

for (const file of files) {
  const seed = parseInt(file.replace('seed', '').replace('.json', ''));
  const data = JSON.parse(fs.readFileSync(path.join(MINIMIZED_DIR, file), 'utf8'));

  // Process both teams
  for (const team of [data.p1, data.p2]) {
    if (!team) continue;

    for (const pokemon of team) {
      // Track moves
      if (pokemon.moves) {
        for (const move of pokemon.moves) {
          if (!moves.has(move)) moves.set(move, []);
          if (!moves.get(move).includes(seed)) {
            moves.get(move).push(seed);
          }
        }
      }

      // Track items
      if (pokemon.item && pokemon.item !== '') {
        if (!items.has(pokemon.item)) items.set(pokemon.item, []);
        if (!items.get(pokemon.item).includes(seed)) {
          items.get(pokemon.item).push(seed);
        }
      }

      // Track abilities
      if (pokemon.ability) {
        if (!abilities.has(pokemon.ability)) abilities.set(pokemon.ability, []);
        if (!abilities.get(pokemon.ability).includes(seed)) {
          abilities.get(pokemon.ability).push(seed);
        }
      }
    }
  }
}

// Sort by number of occurrences (most common first)
const sortByOccurrences = (map) => {
  return [...map.entries()].sort((a, b) => b[1].length - a[1].length);
};

// Filter out simple/placeholder entries
const SIMPLE_ABILITIES = ['Run Away', 'Honey Gather', 'Illuminate', 'Ball Fetch'];
const SIMPLE_MOVES = ['tackle'];

console.log('='.repeat(60));
console.log('MOVES (excluding tackle)');
console.log('='.repeat(60));
const sortedMoves = sortByOccurrences(moves);
for (const [move, seeds] of sortedMoves) {
  if (SIMPLE_MOVES.includes(move)) continue;
  console.log(`${move}: ${seeds.length} seeds`);
  console.log(`  Seeds: ${seeds.slice(0, 20).join(', ')}${seeds.length > 20 ? '...' : ''}`);
}

console.log('\n' + '='.repeat(60));
console.log('ABILITIES (excluding simple ones)');
console.log('='.repeat(60));
const sortedAbilities = sortByOccurrences(abilities);
for (const [ability, seeds] of sortedAbilities) {
  if (SIMPLE_ABILITIES.includes(ability)) continue;
  console.log(`${ability}: ${seeds.length} seeds`);
  console.log(`  Seeds: ${seeds.slice(0, 20).join(', ')}${seeds.length > 20 ? '...' : ''}`);
}

console.log('\n' + '='.repeat(60));
console.log('ITEMS');
console.log('='.repeat(60));
const sortedItems = sortByOccurrences(items);
if (sortedItems.length === 0) {
  console.log('(no items found in minimized tests)');
} else {
  for (const [item, seeds] of sortedItems) {
    console.log(`${item}: ${seeds.length} seeds`);
    console.log(`  Seeds: ${seeds.slice(0, 20).join(', ')}${seeds.length > 20 ? '...' : ''}`);
  }
}

// Summary stats
console.log('\n' + '='.repeat(60));
console.log('SUMMARY');
console.log('='.repeat(60));
console.log(`Total minimized seeds: ${files.length}`);
console.log(`Unique moves: ${moves.size} (${moves.size - SIMPLE_MOVES.filter(m => moves.has(m)).length} non-simple)`);
console.log(`Unique abilities: ${abilities.size} (${abilities.size - SIMPLE_ABILITIES.filter(a => abilities.has(a)).length} non-simple)`);
console.log(`Unique items: ${items.size}`);

// Most common patterns
console.log('\n--- Most Common Moves (top 10) ---');
sortedMoves.filter(([m]) => !SIMPLE_MOVES.includes(m)).slice(0, 10).forEach(([move, seeds]) => {
  console.log(`  ${move}: ${seeds.length} seeds`);
});

console.log('\n--- Most Common Abilities (top 10) ---');
sortedAbilities.filter(([a]) => !SIMPLE_ABILITIES.includes(a)).slice(0, 10).forEach(([ability, seeds]) => {
  console.log(`  ${ability}: ${seeds.length} seeds`);
});
