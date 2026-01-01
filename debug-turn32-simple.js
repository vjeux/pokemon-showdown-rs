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
let turn32Calls = [];
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    const result = originalNext();

    // Capture Turn 32 calls (cumulative 114-117 based on trace)
    if (totalPrngCalls >= 114 && totalPrngCalls <= 120) {
        const stack = new Error().stack.split('\n');
        let caller = 'unknown';

        // Look for the first meaningful function name in the stack
        for (let i = 3; i < Math.min(stack.length, 10); i++) {
            const line = stack[i];
            const match = line.match(/at (?:(\w+)\.)?(\w+) /);
            if (match && match[2] !== 'next' && match[2] !== 'random' && match[2] !== 'randomChance') {
                caller = match[1] ? `${match[1]}.${match[2]}` : match[2];
                break;
            }
        }

        turn32Calls.push({
            num: totalPrngCalls,
            caller: caller,
            value: result
        });
    }

    return result;
};

let turnNumber = 0;
while (!battle.ended && turnNumber < 50) {
    turnNumber++;
    const prngBefore = totalPrngCalls;
    battle.makeChoices('default', 'default');
    const prngAfter = totalPrngCalls;

    if (turnNumber === 32) {
        console.log(`\n=== JavaScript Turn ${turnNumber}: PRNG ${prngBefore} -> ${prngAfter} (${prngAfter - prngBefore} calls) ===\n`);

        console.log('PRNG calls made:');
        for (const call of turn32Calls) {
            console.log(`  [PRNG #${call.num}] ${call.caller}`);
        }

        console.log(`\nExpected: 4 calls (cumulative 113->117)`);
        console.log(`Actual: ${prngAfter - prngBefore} calls`);
        break;
    }
}
