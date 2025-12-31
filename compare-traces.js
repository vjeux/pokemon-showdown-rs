#!/usr/bin/env node

const fs = require('fs');

// Load both traces
const jsTrace = JSON.parse(fs.readFileSync('trace-js-seed1.json', 'utf8'));
const rustTrace = JSON.parse(fs.readFileSync('trace-rust-seed1.json', 'utf8'));

console.log('=== Battle Comparison ===\n');
console.log(`JS: ${jsTrace.turns} turns, ${jsTrace.totalPrngCalls} PRNG calls`);
console.log(`Rust: ${rustTrace.summary.turns} turns, ${rustTrace.prng_calls_per_turn.reduce((a, b) => a + b, 0)} PRNG calls`);
console.log();

// Compare turn by turn
const maxTurns = Math.max(jsTrace.prngCallsPerTurn.length, rustTrace.prng_calls_per_turn.length);

let firstDivergence = null;
for (let i = 0; i < maxTurns; i++) {
    const jsCalls = jsTrace.prngCallsPerTurn[i] || 0;
    const rustCalls = rustTrace.prng_calls_per_turn[i] || 0;
    const match = jsCalls === rustCalls;

    if (!match && firstDivergence === null) {
        firstDivergence = i + 1;
        console.log(`\n*** FIRST DIVERGENCE ON TURN ${i + 1} ***\n`);
    }

    if (!match || (firstDivergence && i < firstDivergence + 5)) {
        const marker = match ? '✓' : '✗';
        console.log(`Turn ${i + 1}: JS=${jsCalls}, Rust=${rustCalls} ${marker}`);
    }
}

if (firstDivergence) {
    console.log(`\n==> First divergence on turn ${firstDivergence}`);
    console.log(`    JS had ${jsTrace.prngCallsPerTurn[firstDivergence - 1]} PRNG calls`);
    console.log(`    Rust had ${rustTrace.prng_calls_per_turn[firstDivergence - 1]} PRNG calls`);
    console.log(`    Difference: ${rustTrace.prng_calls_per_turn[firstDivergence - 1] - jsTrace.prngCallsPerTurn[firstDivergence - 1]}`);
} else {
    console.log('\n✓✓✓ PERFECT MATCH! ✓✓✓');
}

console.log('\n=== Summary ===');
console.log(`JS Winner: ${jsTrace.winner}`);
console.log(`Rust Winner: ${rustTrace.summary.winner}`);
console.log(`Winners match: ${jsTrace.winner === rustTrace.summary.winner ? 'YES ✓' : 'NO ✗'}`);
