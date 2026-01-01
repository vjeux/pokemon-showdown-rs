const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, 1]);

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

// Wrap PRNG AFTER setPlayer (matching run-battle-full-trace.js)
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    return originalNext();
};

let prngPerTurn = [];
let turnNumber = 0;
while (!battle.ended && turnNumber < 50) {
    turnNumber++;
    const prngBefore = totalPrngCalls;
    battle.makeChoices('default', 'default');
    const prngAfter = totalPrngCalls;
    const prngThisTurn = prngAfter - prngBefore;
    prngPerTurn.push(prngThisTurn);

    if ((turnNumber >= 1 && turnNumber <= 15) || (turnNumber >= 29 && turnNumber <= 35)) {
        console.log(`Turn ${turnNumber}: PRNG ${prngBefore} -> ${prngAfter} (${prngThisTurn} calls)`);
    }

    if (battle.ended) break;
}

console.log(`\nTotal turns: ${turnNumber}`);
console.log(`Total PRNG calls: ${totalPrngCalls}`);
