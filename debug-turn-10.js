#!/usr/bin/env node
const {Battle} = require('../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const seedNum = 20;
const teamsFile = `/tmp/teams-seed${seedNum}-js.json`;
const teams = JSON.parse(fs.readFileSync(teamsFile, 'utf8'));

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, seedNum]);

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({...p})),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({...p})),
});

// Listen to battle log
const logs = [];
battle.sendUpdates = () => {
    const log = battle.log.join('\n');
    logs.push(log);
    battle.log = [];
};

let iteration = 0;
while (!battle.ended) {
    battle.makeChoices('default', 'default');
    iteration++;

    // Output battle log for turn 10 only
    if (battle.turn === 10) {
        console.log('=== TURN 10 BATTLE LOG ===');
        console.log(logs[logs.length - 1]);
        break;
    }
}
