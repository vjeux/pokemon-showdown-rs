const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, 1]);

// Wrap PRNG to count calls
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    return originalNext();
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

let makeChoicesNum = 0;
while (!battle.ended && makeChoicesNum < 50) {
    const prngBefore = totalPrngCalls;
    const logBefore = battle.log.length;

    battle.makeChoices('default', 'default');
    makeChoicesNum++;

    const prngAfter = totalPrngCalls;
    const logAfter = battle.log.length;

    if (makeChoicesNum >= 38 && makeChoicesNum <= 40) {
        console.log(`\n========== makeChoices #${makeChoicesNum} ==========`);
        console.log(`PRNG: ${prngBefore} -> ${prngAfter} (${prngAfter - prngBefore} calls)`);
        console.log(`Battle turn: ${battle.turn}`);
        console.log(`\nLog:`);
        const newLog = battle.log.slice(logBefore);
        console.log(newLog.join('\n'));
    }

    if (makeChoicesNum >= 40) break;
}
