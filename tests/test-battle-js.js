#!/usr/bin/env node

/**
 * JavaScript Battle Test Runner
 *
 * Runs a random battle with a specific seed and outputs:
 * - Turn number
 * - PRNG call count before/after each turn
 * - HP of all active Pokemon
 *
 * Output format: #<iteration>: turn=<turn>, prng=<before>-><after>, P1=[...], P2=[...]
 *
 * Usage: node tests/test-battle-js.js [seed_number]
 */

const {Battle} = require('./../../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const seedNum = parseInt(process.argv[2]) || 1;
const teamsFile = `/tmp/teams-seed${seedNum}-js.json`;

if (!fs.existsSync(teamsFile)) {
    console.error(`ERROR: Team file not found: ${teamsFile}`);
    console.error('Run: node tests/generate-test-teams.js [seed_number] first');
    process.exit(1);
}

// Load teams from JSON file
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

// Create battle with specific seed
const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, seedNum]);

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

// Parse TRACE_PRNG environment variable
// Examples: TRACE_PRNG=1-5 (trace calls 1 through 5)
//           TRACE_PRNG=10 (trace call 10)
//           TRACE_PRNG=1,5,10 (trace calls 1, 5, and 10)
const tracePrng = process.env.TRACE_PRNG || '';
const traceCalls = new Set();
if (tracePrng) {
    tracePrng.split(',').forEach(part => {
        if (part.includes('-')) {
            const [start, end] = part.split('-').map(Number);
            for (let i = start; i <= end; i++) {
                traceCalls.add(i);
            }
        } else {
            traceCalls.add(Number(part));
        }
    });
}

// Wrap PRNG to count calls and optionally trace
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;

    // Trace specific calls if requested
    if (traceCalls.has(totalPrngCalls)) {
        console.error(`\n[JS PRNG #${totalPrngCalls}] Stack trace:`);
        const stack = new Error().stack.split('\n').slice(2, 10);
        stack.forEach(line => console.error('  ' + line.trim()));
        console.error('');
    }

    return originalNext();
};

// Patch speedSort to log what's being shuffled
const originalSpeedSort = battle.speedSort.bind(battle);
battle.speedSort = function(list, comparator) {
    const prngBefore = totalPrngCalls;
    const result = originalSpeedSort(list, comparator);
    const prngAfter = totalPrngCalls;

    if (prngAfter > prngBefore) {
        console.error(`\n[SPEED_SORT SHUFFLED] Made ${prngAfter - prngBefore} PRNG calls sorting ${list.length} items:`);
        list.forEach((item, i) => {
            console.error(`  [${i}] effect=${item.effect?.id || item.id || 'unknown'}, speed=${item.speed || 0}, priority=${item.priority || 0}, subOrder=${item.subOrder || 0}, order=${item.order || 'none'}`);
        });
    }
    return result;
};

console.log(`# JavaScript Battle Test - Seed ${seedNum}`);
console.log(`# P1: ${teams.p1[0].name} vs P2: ${teams.p2[0].name}`);

// Run battle for up to 100 turns
for (let i = 1; i <= 100; i++) {
    const prngBefore = totalPrngCalls;

    battle.makeChoices('default', 'default');

    const prngAfter = totalPrngCalls;

    // Get active Pokemon HP
    const p1Active = battle.sides[0].active
        .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
        .join(', ');
    const p2Active = battle.sides[1].active
        .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
        .join(', ');

    console.log(`#${i}: turn=${battle.turn}, prng=${prngBefore}->${prngAfter}, P1=[${p1Active}], P2=[${p2Active}]`);

    if (battle.ended || i >= 100) {
        console.log(`# Battle ended: ${battle.ended}, Turn: ${battle.turn}, Total PRNG: ${totalPrngCalls}`);
        break;
    }
}
