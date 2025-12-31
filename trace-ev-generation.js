#!/usr/bin/env node

// Detailed trace of EV generation to find the 1-call discrepancy

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

const prng = new PRNG([0, 0, 0, 1]);

let nextCallCount = 0;
const originalNext = prng.rng.next.bind(prng.rng);
prng.rng.next = function() {
    nextCallCount++;
    return originalNext();
};

// Skip to call 10 (after moves, before EVs)
// Based on clean-trace.js: species(1), level(2), item(3), nature(4), gender(5), moves(9)
// We'll do this by calling random methods 9 times
for (let i = 0; i < 9; i++) {
    prng.random(0, 100);
}

console.log(`Starting EV generation at call ${nextCallCount + 1}\n`);

function generateRandomEVs(prng) {
    const evs = [0, 0, 0, 0, 0, 0];
    let totalEVs = 0;
    let iteration = 0;

    while (totalEVs < 510) {
        iteration++;
        const callsBefore = nextCallCount;

        const availableStats = [];
        for (let i = 0; i < evs.length; i++) {
            if (evs[i] < 252) {
                availableStats.push(i);
            }
        }

        if (availableStats.length === 0) break;

        const statIdx = prng.sample(availableStats);
        const callAfterSample = nextCallCount;

        const amount = Math.min(
            prng.random(1, 5),
            252 - evs[statIdx],
            510 - totalEVs
        );
        const callAfterRandom = nextCallCount;

        evs[statIdx] += amount;
        totalEVs += amount;

        console.log(`[Iter ${iteration}] calls ${callsBefore + 1}-${callAfterRandom}: available=${availableStats.length}, statIdx=${statIdx}, amount=${amount}, totalEVs=${totalEVs}, evs=[${evs.join(',')}]`);
    }

    console.log(`\nEV generation complete: ${iteration} iterations, ${nextCallCount - 9} total calls`);
    return {
        hp: evs[0],
        atk: evs[1],
        def: evs[2],
        spa: evs[3],
        spd: evs[4],
        spe: evs[5],
    };
}

const evs = generateRandomEVs(prng);
console.log(`\nFinal EVs: ${JSON.stringify(evs)}`);
console.log(`Total calls after EVs: ${nextCallCount}`);
