const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([1, 1, 1, 1]);

let totalPrngCalls = 0;
let currentTurnPrngCalls = [];

const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    const result = originalNext();
    const stack = new Error().stack.split('\n')[3];
    const match = stack.match(/at (?:\w+\.)?(\w+) /);
    const func = match ? match[1] : 'unknown';
    currentTurnPrngCalls.push({num: totalPrngCalls, func});
    return result;
};

// Load teams from JSON file
const teams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

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

let turnNumber = 0;
while (battle.p1.activeRequest && battle.p2.activeRequest && turnNumber < 50) {
    const prngBefore = totalPrngCalls;
    const logBefore = battle.log.length;
    currentTurnPrngCalls = [];

    battle.choose('p1', 'default');
    battle.choose('p2', 'default');
    turnNumber++;

    const logAfter = battle.log.length;
    const prngAfter = totalPrngCalls;
    const prngThisTurn = prngAfter - prngBefore;

    // Turn 32 has cumulative PRNG 113->117 (4 calls)
    if (prngBefore === 113) {
        console.log(`\n=== JavaScript Turn ${turnNumber}: PRNG ${prngBefore} -> ${prngAfter} (${prngThisTurn} calls) ===`);
        console.log('\nBattle log:');
        for (let i = logBefore; i < logAfter; i++) {
            const line = battle.log[i];
            console.log('  ' + line);
        }

        console.log('\nPRNG calls made:');
        for (const call of currentTurnPrngCalls) {
            console.log(`  [PRNG #${call.num}] ${call.func}`);
        }
        console.log(`\nExpected: 4 calls`);
        console.log(`Actual: ${currentTurnPrngCalls.length} calls`);
        break;
    }
}
