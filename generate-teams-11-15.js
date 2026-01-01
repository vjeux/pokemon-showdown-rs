const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

// Generate teams for seeds 11-15
for (let seedNum = 11; seedNum <= 15; seedNum++) {
    console.log(`\nGenerating teams for seed ${seedNum}...`);

    const battle = new Battle({formatid: 'gen9randombattle'});
    battle.prng = new PRNG([0, 0, 0, seedNum]);

    battle.setPlayer('p1', {name: 'Player 1', team: null});
    battle.setPlayer('p2', {name: 'Player 2', team: null});

    // Extract teams
    const p1Team = battle.sides[0].pokemon.map(p => ({
        name: p.name,
        species: p.species.name,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.set.nature || p.baseNature || 'serious',
        gender: p.gender === 'M' ? 'M' : p.gender === 'F' ? 'F' : '',
        moves: p.moves,
        evs: p.set.evs,
        ivs: p.set.ivs,
    }));

    const p2Team = battle.sides[1].pokemon.map(p => ({
        name: p.name,
        species: p.species.name,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.set.nature || p.baseNature || 'serious',
        gender: p.gender === 'M' ? 'M' : p.gender === 'F' ? 'F' : '',
        moves: p.moves,
        evs: p.set.evs,
        ivs: p.set.ivs,
    }));

    const teams = {p1: p1Team, p2: p2Team};

    // Write to both js and rust files
    fs.writeFileSync(`teams-seed${seedNum}-js.json`, JSON.stringify(teams, null, 2));
    fs.writeFileSync(`teams-seed${seedNum}-rust.json`, JSON.stringify(teams, null, 2));

    console.log(`Seed ${seedNum}: ${p1Team[0].name} vs ${p2Team[0].name}`);
}

console.log('\nAll teams generated!');
