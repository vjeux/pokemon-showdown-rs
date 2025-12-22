// Test harness to generate PRNG output for comparison with Rust
// Run with: node test-prng.js

// We need to set up a minimal environment to import the PRNG
const path = require('path');

// Import the PRNG class from pokemon-showdown
const simPath = path.join(__dirname, 'pokemon-showdown-js', 'sim');

// Simple Gen5 PRNG implementation for testing (matching the one in sim/prng.ts)
class Gen5RNG {
    constructor(seed) {
        this.seed = [...seed];
    }

    getSeed() {
        return this.seed.join(',');
    }

    next() {
        this.seed = this.nextFrame(this.seed);
        return (this.seed[0] << 16 >>> 0) + this.seed[1];
    }

    multiplyAdd(a, b, c) {
        const out = [0, 0, 0, 0];
        let carry = 0;

        for (let outIndex = 3; outIndex >= 0; outIndex--) {
            for (let bIndex = outIndex; bIndex < 4; bIndex++) {
                const aIndex = 3 - (bIndex - outIndex);
                carry += a[aIndex] * b[bIndex];
            }
            carry += c[outIndex];
            out[outIndex] = carry & 0xFFFF;
            carry >>>= 16;
        }

        return out;
    }

    nextFrame(seed) {
        const a = [0x5D58, 0x8B65, 0x6C07, 0x8965];
        const c = [0, 0, 0x26, 0x9EC3];
        return this.multiplyAdd(seed, a, c);
    }
}

class PRNG {
    constructor(seed) {
        this.rng = new Gen5RNG(seed);
        this.startingSeed = seed.join(',');
    }

    getSeed() {
        return this.rng.getSeed();
    }

    random(from, to) {
        const result = this.rng.next();

        if (from) from = Math.floor(from);
        if (to) to = Math.floor(to);
        if (from === undefined) {
            return result / 2 ** 32;
        } else if (!to) {
            return Math.floor(result * from / 2 ** 32);
        } else {
            return Math.floor(result * (to - from) / 2 ** 32) + from;
        }
    }
}

// Test cases
function runTests() {
    console.log("=== Gen5 PRNG Test Cases ===\n");

    // Test case 1: Fixed seed, check sequence
    const seed = [0x1234, 0x5678, 0x9ABC, 0xDEF0];
    console.log(`Seed: [${seed.join(', ')}]`);

    const prng = new PRNG(seed);
    console.log("\nFirst 20 random(100) values:");
    const values = [];
    for (let i = 0; i < 20; i++) {
        const val = prng.random(100);
        values.push(val);
    }
    console.log(JSON.stringify(values));

    // Test case 2: Different seed
    const seed2 = [1, 2, 3, 4];
    console.log(`\nSeed: [${seed2.join(', ')}]`);

    const prng2 = new PRNG(seed2);
    console.log("First 20 random(100) values:");
    const values2 = [];
    for (let i = 0; i < 20; i++) {
        const val = prng2.random(100);
        values2.push(val);
    }
    console.log(JSON.stringify(values2));

    // Test case 3: Raw 32-bit values for precise comparison
    const seed3 = [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD];
    console.log(`\nSeed: [${seed3.join(', ')}]`);

    const rng3 = new Gen5RNG(seed3);
    console.log("First 10 raw next() values (32-bit):");
    const raw = [];
    for (let i = 0; i < 10; i++) {
        raw.push(rng3.next());
    }
    console.log(JSON.stringify(raw));
}

runTests();
