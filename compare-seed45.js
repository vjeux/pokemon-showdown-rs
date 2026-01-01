const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, 45]);

// Load teams from JSON file
const teams = JSON.parse(fs.readFileSync('teams-seed45-js.json', 'utf8'));

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

// Wrap PRNG to count calls
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    return originalNext();
};

console.log('JS: Running battle with seed [0, 0, 0, 45]');

for (let i = 1; i <= 100; i++) {
    const prngBefore = totalPrngCalls;

    battle.makeChoices('default', 'default');

    const prngAfter = totalPrngCalls;

    // Get active Pokemon
    const p1Active = battle.sides[0].active
        .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
        .join(', ');
    const p2Active = battle.sides[1].active
        .map(p => p ? `${p.name}(${p.hp}/${p.maxhp})` : 'none')
        .join(', ');

    console.log(`#${i}: turn=${battle.turn}, prng=${prngBefore}->${prngAfter}, P1=[${p1Active}], P2=[${p2Active}]`);

    if (battle.ended || i >= 100) {
        console.log('\nJS Battle status:');
        console.log(`  Ended: ${battle.ended}`);
        console.log(`  Turn: ${battle.turn}`);
        console.log(`  Total PRNG calls: ${totalPrngCalls}`);
        break;
    }
}
