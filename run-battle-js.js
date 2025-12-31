#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

// Load the teams we generated
const teams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

const dex = Dex.mod('gen9');

// Create battle with seed 1
const battle = new Battle({
    formatid: 'gen9randombattle',
    seed: [0, 0, 0, 1],
});

// Add players with teams
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

console.log('JS: Battle created');
console.log('JS: Turn:', battle.turn);

// Battle starts automatically, just check status
console.log('JS: Battle started:', battle.started);
console.log('JS: After initialization, turn:', battle.turn);
const p1Active = battle.p1.active[0];
const p2Active = battle.p2.active[0];
if (p1Active) {
    console.log('JS: P1 active:', p1Active.name);
    console.log('JS: P1 HP:', p1Active.hp + '/' + p1Active.maxhp);
}
if (p2Active) {
    console.log('JS: P2 active:', p2Active.name);
    console.log('JS: P2 HP:', p2Active.hp + '/' + p2Active.maxhp);
}

// Run battle until it ends (max 50 turns)
let turnNum = 1;
while (!battle.ended && turnNum <= 50) {
    console.log(`JS: Making turn ${turnNum} choices...`);

    // Check what choices are needed
    const p1Choice = battle.p1.activeRequest ? 'default' : 'move 1';
    const p2Choice = battle.p2.activeRequest ? 'default' : 'move 1';

    console.log(`JS: P1 choice: ${p1Choice}, P2 choice: ${p2Choice}`);
    battle.makeChoices(p1Choice, p2Choice);

    console.log(`JS: After turn ${turnNum}, turn: ${battle.turn}`);

    // Get fresh references to active Pokemon
    const currentP1 = battle.p1.active[0];
    const currentP2 = battle.p2.active[0];

    if (currentP1 && currentP1.hp > 0) {
        console.log('JS: P1 HP:', currentP1.hp + '/' + currentP1.maxhp);
    } else if (currentP1) {
        console.log('JS: P1 fainted');
    }
    if (currentP2 && currentP2.hp > 0) {
        console.log('JS: P2 HP:', currentP2.hp + '/' + currentP2.maxhp);
    } else if (currentP2) {
        console.log('JS: P2 fainted');
    }

    if (battle.ended) {
        console.log('JS: Battle ended!');
        console.log('JS: Winner:', battle.winner);
        break;
    }

    turnNum++;
}

console.log('\nJS LOG:');
console.log(battle.log.join('\n'));

// Save log to file
fs.writeFileSync('battle-js.log', battle.log.join('\n'));
console.log('\nLog saved to battle-js.log');
