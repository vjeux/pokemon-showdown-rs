const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([1, 1, 1, 1]);

// Track total PRNG calls
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    return originalNext();
};

// Load teams from JSON file (same as Rust now uses)
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

let prngPerTurn = [];
while (battle.p1.activeRequest && battle.p2.activeRequest && prngPerTurn.length < 40) {
    const prngBefore = totalPrngCalls;

    battle.choose('p1', 'default');
    battle.choose('p2', 'default');

    const prngAfter = totalPrngCalls;
    prngPerTurn.push(prngAfter - prngBefore);
}

// Compare with expected
const expected = [3,3,3,3,3,3,3,3,0,6,6,0,6,6,3,0,7,7,4,0,3,0,7,7,7,7,4,0,5,4,0,4,6,6,4,0,4,4,3,0,4];

console.log('\nComparison:');
for (let i = 0; i < Math.min(prngPerTurn.length, expected.length); i++) {
    const match = prngPerTurn[i] === expected[i] ? '✓' : '✗';
    console.log(`Turn ${i+1}: ${match} JS=${prngPerTurn[i]}, Expected=${expected[i]}`);
}

console.log(`\nTotal PRNG calls: ${totalPrngCalls}`);
console.log(`Cumulative after turn 31: ${prngPerTurn.slice(0, 31).reduce((a,b) => a+b, 0)}`);
console.log(`Turn 32 PRNG calls: ${prngPerTurn[31]}`);
