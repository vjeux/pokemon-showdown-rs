#!/usr/bin/env node

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

const prng = new PRNG([0, 0, 0, 1]);

console.log('Call 1:', prng.rng.next());
console.log('Call 2:', prng.rng.next());
console.log('Call 3:', prng.rng.next());

// Now test random(583)
const prng2 = new PRNG([0, 0, 0, 1]);
prng2.rng.next(); // skip call 1
prng2.rng.next(); // skip call 2
const index = prng2.random(583);
console.log('\nIndex from random(583):', index);
