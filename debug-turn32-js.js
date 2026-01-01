const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([1, 1, 1, 1]);

let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    const result = originalNext();

    // Log PRNG calls on turn 32 only
    if (totalPrngCalls >= 108 && totalPrngCalls <= 115) {
        const stack = new Error().stack.split('\n')[3];
        const match = stack.match(/at (\w+\.)?(\w+) /);
        const func = match ? match[2] : 'unknown';
        console.log(`  [PRNG #${totalPrngCalls}] ${func}`);
    }

    return result;
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

let turnsSinceStart = 0;
while (battle.p1.activeRequest && battle.p2.activeRequest) {
    const prngBefore = totalPrngCalls;
    const logBefore = battle.log.length;

    battle.choose('p1', 'default');
    battle.choose('p2', 'default');
    turnsSinceStart++;

    const logAfter = battle.log.length;
    const prngAfter = totalPrngCalls;
    const prngThisTurn = prngAfter - prngBefore;

    // Show turns 30-34
    if (turnsSinceStart >= 30 && turnsSinceStart <= 34) {
        console.log(`\n=== Turn ${turnsSinceStart}: PRNG ${prngBefore} -> ${prngAfter} (${prngThisTurn} calls) ===`);
        for (let i = logBefore; i < logAfter; i++) {
            const line = battle.log[i];
            if (line.startsWith('|move|') || line.startsWith('|switch|') ||
                line.startsWith('|-damage|') || line.startsWith('|faint|') ||
                line.startsWith('|turn|') || line.startsWith('|-heal|') ||
                line.startsWith('|-boost|') || line.startsWith('|-unboost|')) {
                console.log(line);
            }
        }
    }

    if (turnsSinceStart > 35) break;
}
