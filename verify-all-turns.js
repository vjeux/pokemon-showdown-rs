const js = require('./trace-js-seed1.json');
const rust = require('./trace-rust-seed1.json');

console.log('Comparing all turns:\n');

const maxTurns = Math.max(js.prngCallsPerTurn.length, rust.prng_calls_per_turn.length);

for (let i = 0; i < maxTurns; i++) {
    const jsCalls = js.prngCallsPerTurn[i] || 'N/A';
    const rustCalls = rust.prng_calls_per_turn[i] || 'N/A';

    if (jsCalls === rustCalls) {
        console.log(`Turn ${i+1}: ✓ ${jsCalls} calls`);
    } else {
        console.log(`Turn ${i+1}: ✗ JS=${jsCalls}, Rust=${rustCalls} (diff: ${rustCalls - jsCalls})`);
    }
}

// Summary
console.log('\n' + '='.repeat(60));
let firstDivergence = null;
for (let i = 0; i < maxTurns; i++) {
    const jsCalls = js.prngCallsPerTurn[i];
    const rustCalls = rust.prng_calls_per_turn[i];
    if (jsCalls !== rustCalls) {
        firstDivergence = i + 1;
        break;
    }
}

if (firstDivergence) {
    console.log(`First divergence at turn ${firstDivergence}`);
    console.log(`Turns 1-${firstDivergence - 1} match perfectly!`);
} else {
    console.log('All turns match perfectly!');
}
