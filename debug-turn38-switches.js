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

// Wrap runAction to log switch requests
const originalRunAction = battle.runAction.bind(battle);
battle.runAction = function(action) {
    const result = originalRunAction(action);
    if (result === true) {
        console.log(`  [SWITCH REQUEST TRIGGERED] runAction returned true, requestState=${battle.requestState}`);
    }
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

let makeChoicesNum = 0;
while (!battle.ended && makeChoicesNum < 50) {
    const prngBefore = totalPrngCalls;
    const logBefore = battle.log.length;
    const turnBefore = battle.turn;

    if (makeChoicesNum + 1 >= 38 && makeChoicesNum + 1 <= 39) {
        console.log(`\n========== BEFORE makeChoices #${makeChoicesNum + 1} ==========`);
        console.log(`Battle turn: ${battle.turn}`);
        console.log(`Request state: ${battle.requestState}`);
        console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (HP: ${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
        console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (HP: ${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
    }

    battle.makeChoices('default', 'default');
    makeChoicesNum++;

    const prngAfter = totalPrngCalls;
    const logAfter = battle.log.length;
    const turnAfter = battle.turn;

    if (makeChoicesNum >= 38 && makeChoicesNum <= 39) {
        console.log(`\n========== makeChoices #${makeChoicesNum} ==========`);
        console.log(`PRNG: ${prngBefore} -> ${prngAfter} (${prngAfter - prngBefore} calls)`);
        console.log(`Battle turn: ${turnBefore} -> ${turnAfter}`);
        console.log(`Request state: ${battle.requestState}`);
        console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (HP: ${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
        console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (HP: ${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

        console.log(`\nLog:`);
        const newLog = battle.log.slice(logBefore);
        console.log(newLog.join('\n'));
    }

    if (makeChoicesNum >= 39) break;
}
