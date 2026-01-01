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
    const result = originalNext();
    if (totalPrngCalls >= 133 && totalPrngCalls <= 145) {
        console.log(`  PRNG call #${totalPrngCalls}: ${result}`);
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

// Run 29 makeChoices to get to Turn 30
for (let i = 1; i <= 29; i++) {
    battle.makeChoices('default', 'default');
}

console.log('========== BEFORE Turn 30 ==========');
console.log(`Battle turn: ${battle.turn}`);
console.log(`PRNG calls: ${totalPrngCalls}`);
console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

const logBefore = battle.log.length;
const prngBefore = totalPrngCalls;

console.log('\n========== Turn 30 makeChoices ==========');
battle.makeChoices('default', 'default');

const prngAfter = totalPrngCalls;
console.log(`\nPRNG: ${prngBefore} -> ${prngAfter} (${prngAfter - prngBefore} calls)`);
console.log(`Battle turn: ${battle.turn}`);
console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

console.log('\n========== Turn 30 Log ==========');
const newLog = battle.log.slice(logBefore);
console.log(newLog.join('\n'));
