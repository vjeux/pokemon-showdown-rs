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

let turnNumber = 0;
while (!battle.ended && turnNumber < 50) {
    turnNumber++;

    if (turnNumber === 36 || turnNumber === 37 || turnNumber === 38) {
        console.log(`\n=== BEFORE Turn ${turnNumber} - Choosing moves ===`);
        console.log(`Request for p1:`, JSON.stringify(battle.sides[0].getRequestData(), null, 2));
        console.log(`Request for p2:`, JSON.stringify(battle.sides[1].getRequestData(), null, 2));
    }

    battle.makeChoices('default', 'default');

    if (turnNumber === 36 || turnNumber === 37 || turnNumber === 38) {
        console.log(`\n=== AFTER Turn ${turnNumber} ===`);
        for (const side of battle.sides) {
            console.log(`\n${side.name}:`);
            for (const pokemon of side.pokemon) {
                console.log(`  ${pokemon.name}: HP=${pokemon.hp}/${pokemon.maxhp} fainted=${pokemon.fainted} active=${pokemon.isActive} position=${pokemon.position}`);
            }
        }
    }

    if (turnNumber === 38) {
        break;
    }
}
