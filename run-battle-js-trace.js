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

// Patch PRNG to log all calls AFTER battle is set up
let prngCallCount = 0;
const prngLog = [];

// Patch the RNG's next() method, not PRNG's
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        const stack = new Error().stack.split('\n').slice(2, 5).join(' | ');
        prngLog.push({count: prngCallCount, value, stack});
        if (prngCallCount <= 50) {
            console.log(`[PRNG #${prngCallCount}] value=${value} turn=${battle.turn}`);
        }
        return value;
    };
    console.log('PRNG patched successfully!');
} else {
    console.log('ERROR: Cannot patch PRNG');
    console.log('battle.prng:', battle.prng);
    console.log('battle.prng.rng:', battle.prng ? battle.prng.rng : 'undefined');
}

console.log('JS: Battle created, turn:', battle.turn);

const p1Active = battle.p1.active[0];
const p2Active = battle.p2.active[0];

if (p1Active) {
    console.log('JS: P1 active:', p1Active.name, p1Active.hp + '/' + p1Active.maxhp);
}
if (p2Active) {
    console.log('JS: P2 active:', p2Active.name, p2Active.hp + '/' + p2Active.maxhp);
}

// Run only up to turn 13
for (let turnNum = 1; turnNum <= 13; turnNum++) {
    console.log(`\n=== JS: Making turn ${turnNum} choices ===`);
    const prngBefore = prngCallCount;

    battle.makeChoices('default', 'default');

    const prngAfter = prngCallCount;
    console.log(`JS: Turn ${turnNum} used ${prngAfter - prngBefore} PRNG calls (total: ${prngAfter})`);
    console.log(`JS: After turn ${turnNum}, battle.turn: ${battle.turn}`);

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
}

console.log(`\nTotal PRNG calls: ${prngCallCount}`);

// Save detailed log
fs.writeFileSync('prng-trace-js.json', JSON.stringify(prngLog, null, 2));
console.log('Saved PRNG trace to prng-trace-js.json');
