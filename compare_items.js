const fs = require('fs');

// Read both item lists
const jsItems = fs.readFileSync('/tmp/js_items.txt', 'utf8').trim().split('\n').sort();
const rustItems = fs.readFileSync('/tmp/rust_items.txt', 'utf8').trim().split('\n').sort();

const jsSet = new Set(jsItems);
const rustSet = new Set(rustItems);

// Find items in JS but not in Rust (missing)
const missingFromRust = jsItems.filter(item => !rustSet.has(item));

// Find items in Rust but not in JS (extra)
const extraInRust = rustItems.filter(item => !jsSet.has(item));

console.log('=== ITEMS IN JS BUT NOT IN RUST (need to port) ===');
console.log(`Total missing: ${missingFromRust.length}\n`);

// Group by first letter
const byLetter = {};
missingFromRust.forEach(item => {
    const letter = item[0];
    if (!byLetter[letter]) byLetter[letter] = [];
    byLetter[letter].push(item);
});

Object.keys(byLetter).sort().forEach(letter => {
    console.log(`${letter.toUpperCase()} (${byLetter[letter].length}):`);
    byLetter[letter].forEach(item => console.log(`  - ${item}`));
});

console.log('\n=== ITEMS IN RUST BUT NOT IN JS (extra/duplicates) ===');
console.log(`Total extra: ${extraInRust.length}\n`);
extraInRust.forEach(item => console.log(`  - ${item}`));

console.log('\n=== SUMMARY ===');
console.log(`JS items: ${jsItems.length}`);
console.log(`Rust items: ${rustItems.length}`);
console.log(`Missing from Rust: ${missingFromRust.length}`);
console.log(`Extra in Rust: ${extraInRust.length}`);
console.log(`Coverage: ${((rustItems.length - extraInRust.length) / jsItems.length * 100).toFixed(1)}%`);
