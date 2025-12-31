#!/usr/bin/env node

// Complete trace of PRNG calls for first Pokemon including EVs/IVs

const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const Dex = require('./../pokemon-showdown-ts/dist/sim/dex').Dex;

const dex = Dex.mod('gen9');
const prng = new PRNG([0, 0, 0, 1]);

let allSpecies = Object.values(dex.species.all());
allSpecies.sort((a, b) => a.name.localeCompare(b.name));

const allMoves = Object.values(dex.moves.all()).map(m => m.id);
const allItems = Object.values(dex.items.all()).map(i => i.id);
const allNatures = Object.values(dex.natures.all()).map(n => n.id);

let callCount = 0;
const callLog = [];

// Log every PRNG call
const logCall = (method, args, result) => {
    callCount++;
    const entry = `[${callCount}] ${method}(${args}) = ${result}`;
    callLog.push(entry);
    console.log(entry);
};

const originalRandom = prng.random.bind(prng);
const originalSample = prng.sample.bind(prng);
const originalRandomChance = prng.randomChance.bind(prng);

prng.random = function(...args) {
    const result = originalRandom(...args);
    logCall('random', args.join(','), result);
    return result;
};

prng.sample = function(arr) {
    const result = originalSample(arr);
    const displayResult = typeof result === 'object' && result.name ? result.name : result;
    logCall('sample', `arr[${arr.length}]`, displayResult);
    return result;
};

prng.randomChance = function(num, denom) {
    const result = originalRandomChance(num, denom);
    logCall('randomChance', `${num},${denom}`, result);
    return result;
};

console.log('=== COMPLETE POKEMON #1 GENERATION ===\n');

// Generate EVs function
function generateRandomEVs(prng) {
    const evs = [0, 0, 0, 0, 0, 0];
    let totalEVs = 0;

    while (totalEVs < 510) {
        const availableStats = [];
        for (let i = 0; i < evs.length; i++) {
            if (evs[i] < 252) {
                availableStats.push(i);
            }
        }

        if (availableStats.length === 0) break;

        const statIdx = prng.sample(availableStats);
        const amount = Math.min(
            prng.random(1, 5),
            252 - evs[statIdx],
            510 - totalEVs
        );
        evs[statIdx] += amount;
        totalEVs += amount;
    }

    return {
        hp: evs[0],
        atk: evs[1],
        def: evs[2],
        spa: evs[3],
        spd: evs[4],
        spe: evs[5],
    };
}

// 1. Species
const species = prng.sample(allSpecies);
console.log(`\n=> Species: ${species.name}\n`);

// 2. Level
const level = prng.random(50, 101);
console.log(`=> Level: ${level}\n`);

// 3. Item
const item = prng.sample(allItems) || '';
console.log(`=> Item: ${item}\n`);

// 4. Nature
const nature = prng.sample(allNatures);
console.log(`=> Nature: ${nature}\n`);

// 5. Gender
let gender = '';
if (species.genderRatio && species.genderRatio.M > 0 && species.genderRatio.F > 0) {
    gender = prng.randomChance(1, 2) ? 'M' : 'F';
}
console.log(`=> Gender: ${gender || 'N'}\n`);

// 6. Moves
console.log('=> Moves:');
const moves = [];
for (let j = 0; j < 4; j++) {
    const move = prng.sample(allMoves);
    if (!moves.includes(move)) {
        moves.push(move);
    }
}
while (moves.length < 4) {
    moves.push('tackle');
}
console.log(`   ${moves.join(', ')}\n`);

// 7. EVs
console.log('=> EVs:');
const evs = generateRandomEVs(prng);
console.log(`   ${JSON.stringify(evs)}\n`);

// 8. IVs
console.log('=> IVs:');
const ivs = {
    hp: prng.random(0, 32),
    atk: prng.random(0, 32),
    def: prng.random(0, 32),
    spa: prng.random(0, 32),
    spd: prng.random(0, 32),
    spe: prng.random(0, 32),
};
console.log(`   ${JSON.stringify(ivs)}\n`);

console.log(`\n=== TOTAL PRNG CALLS: ${callCount} ===\n`);

// Now test if the next call matches what Rust should get for Pokemon #2
console.log('=== NEXT CALL (for Pokemon #2 species selection) ===');
const usedSpecies = [species.name];
const availableSpecies2 = allSpecies.filter(s => !usedSpecies.includes(s.name));
const species2 = prng.sample(availableSpecies2);
console.log(`Pokemon #2 should be: ${species2.name}`);
