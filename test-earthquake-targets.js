#!/usr/bin/env node

const {Battle} = require('../pokemon-showdown-ts/dist/sim/battle');

// Create a simple battle
const battle = new Battle({formatid: 'gen9randombattle'});

battle.setPlayer('p1', {
    name: 'Player 1',
    team: [{
        name: 'Pikachu',
        species: 'Pikachu',
        moves: ['earthquake'],
        ability: 'Static',
        evs: {hp: 1, atk: 1, def: 1, spa: 1, spd: 1, spe: 1},
        ivs: {hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31},
        item: '',
        level: 50,
        shiny: false,
        gender: 'M',
    }]
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: [{
        name: 'Charizard',
        species: 'Charizard',
        moves: ['tackle'],
        ability: 'Blaze',
        evs: {hp: 1, atk: 1, def: 1, spa: 1, spd: 1, spe: 1},
        ivs: {hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31},
        item: '',
        level: 50,
        shiny: false,
        gender: 'M',
    }]
});

// Get the active Pokemon
const p1Pokemon = battle.p1.active[0];
const p2Pokemon = battle.p2.active[0];

console.log('P1 Pokemon:', p1Pokemon.name, '(', p1Pokemon.species.name, ')');
console.log('P2 Pokemon:', p2Pokemon.name, '(', p2Pokemon.species.name, ')');
console.log('Battle gameType:', battle.gameType);
console.log('Battle activePerHalf:', battle.activePerHalf);

// Get Earthquake move
const earthquake = battle.dex.getActiveMove('earthquake');
console.log('Earthquake target type:', earthquake.target);

// Get move targets for Earthquake
const {targets, pressureTargets} = p1Pokemon.getMoveTargets(earthquake, p2Pokemon);
console.log('Targets:');
targets.forEach((t, i) => {
    console.log(`  [${i}]`, t.name, '(', t.species.name, ')', 'Side:', t.side.id, 'Position:', t.position);
});
console.log('Pressure Targets:');
pressureTargets.forEach((t, i) => {
    console.log(`  [${i}]`, t.name, '(', t.species.name, ')', 'Side:', t.side.id);
});
