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

console.log('P1 Pokemon:', p1Pokemon.name);
console.log('P2 Pokemon:', p2Pokemon.name);
console.log('Battle gameType:', battle.gameType);
console.log('Battle activePerHalf:', battle.activePerHalf);
console.log('');

// Test isAdjacent
console.log('p1Pokemon.isAdjacent(p1Pokemon):', p1Pokemon.isAdjacent(p1Pokemon));
console.log('p1Pokemon.isAdjacent(p2Pokemon):', p1Pokemon.isAdjacent(p2Pokemon));
console.log('');

// Test side.allies()
console.log('p1Pokemon.side.allies():');
const allies = p1Pokemon.side.allies();
allies.forEach((ally, i) => {
    console.log(`  [${i}]`, ally.name, 'same as p1Pokemon?', ally === p1Pokemon);
});
console.log('');

// Test adjacentAllies()
console.log('p1Pokemon.adjacentAllies():');
const adjacentAllies = p1Pokemon.adjacentAllies();
adjacentAllies.forEach((ally, i) => {
    console.log(`  [${i}]`, ally.name, 'same as p1Pokemon?', ally === p1Pokemon);
});
console.log('');

// Test adjacentFoes()
console.log('p1Pokemon.adjacentFoes():');
const adjacentFoes = p1Pokemon.adjacentFoes();
adjacentFoes.forEach((foe, i) => {
    console.log(`  [${i}]`, foe.name);
});
