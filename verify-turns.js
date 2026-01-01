const js = require('./trace-js-seed1.json');
const rust = require('./trace-rust-seed1.json');

console.log('Comparing turns 1-26:');
let match = true;
for (let i = 0; i < 26; i++) {
    const jsCalls = js.prngCallsPerTurn[i];
    const rustCalls = rust.prng_calls_per_turn[i];
    if (jsCalls !== rustCalls) {
        console.log(`  Turn ${i+1}: JS=${jsCalls}, Rust=${rustCalls} ✗ MISMATCH`);
        match = false;
    }
}

if (match) {
    console.log('  ✓ All turns 1-26 match perfectly!');
} else {
    console.log('  ✗ Turns do NOT match!');
}

console.log('\nTurn 27:');
console.log(`  JS: ${js.prngCallsPerTurn[26]} calls`);
console.log(`  Rust: ${rust.prng_calls_per_turn[26]} calls`);
console.log(`  Difference: ${rust.prng_calls_per_turn[26] - js.prngCallsPerTurn[26]} extra calls in Rust`);
