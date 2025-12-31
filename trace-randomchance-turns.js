#!/usr/bin/env node

const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const teams = JSON.parse(fs.readFileSync('teams-js-seed2.json', 'utf8'));
const battle = new Battle({ formatid: 'gen9randombattle', seed: [0, 0, 0, 2] });

battle.setPlayer('p1', { name: 'Player 1', team: teams.p1 });
battle.setPlayer('p2', { name: 'Player 2', team: teams.p2 });

// Patch randomChance to see what it's checking
let inRunEvent = false;
let currentEvent = '';

const originalRunEvent = battle.runEvent.bind(battle);
battle.runEvent = function(eventid, ...args) {
    inRunEvent = true;
    currentEvent = eventid;
    const result = originalRunEvent(eventid, ...args);
    inRunEvent = false;
    return result;
};

const originalRandomChance = battle.randomChance.bind(battle);
battle.randomChance = function(numerator, denominator) {
    const stack = new Error().stack;
    const lines = stack.split('\n');
    const caller = lines[3]?.trim() || 'unknown';

    if (battle.turn === 19 || battle.turn === 20) {
        const eventInfo = inRunEvent ? ` [during ${currentEvent}]` : '';
        console.log(`[T${battle.turn} randomChance] ${numerator}/${denominator}${eventInfo} from ${caller}`);
    }

    return originalRandomChance(numerator, denominator);
};

// Run battle
for (let i = 1; i <= 20; i++) {
    battle.makeChoices('default', 'default');
}
