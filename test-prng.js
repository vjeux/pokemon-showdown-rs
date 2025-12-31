#!/usr/bin/env node

// Test to understand the PRNG mismatch between Rust and JavaScript

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

console.log('=== Testing JavaScript PRNG ===');
console.log('Initial seed: [0, 0, 0, 1]');

const prng = new PRNG([0, 0, 0, 1]);

console.log('\nFirst 10 random() calls:');
for (let i = 0; i < 10; i++) {
	const val = prng.random(100);
	console.log(`${i}: ${val}`);
}

console.log('\n=== Testing with fresh PRNG ===');
const prng2 = new PRNG([0, 0, 0, 1]);
console.log('First random_range(50, 101):', prng2.random(50, 101));
console.log('Second random_range(50, 101):', prng2.random(50, 101));
