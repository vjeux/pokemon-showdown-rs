#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed3.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 3] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

let prngCallCount = 0;

// Patch Pokemon.addVolatile to trace calls
const {Pokemon} = require('./../pokemon-showdown-ts/dist/sim/pokemon');
const originalAddVolatile = Pokemon.prototype.addVolatile;
Pokemon.prototype.addVolatile = function(status, source, sourceEffect, linkedStatus) {
    if (battle.turn === 16) {
        console.log(`[T16 addVolatile] status=${status}, pokemon=${this.name}, source=${source?.name}, sourceEffect=${sourceEffect?.name || sourceEffect?.id}`);
    }
    return originalAddVolatile.call(this, status, source, sourceEffect, linkedStatus);
};

// Patch PRNG to count calls
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        return value;
    };
}

// Run turns 1-16
for (let i = 1; i <= 16; i++) {
    battle.makeChoices('default', 'default');
}
console.log(`\nTotal PRNG calls after turn 16: ${prngCallCount}`);
