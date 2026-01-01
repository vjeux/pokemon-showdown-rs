const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

const battle = new Battle({formatid: 'gen9randombattle'});

// Use seed 1
battle.prng = new PRNG([1, 1, 1, 1]);

battle.setPlayer('p1', {name: 'Player 1', avatar: '1'});
battle.setPlayer('p2', {name: 'Player 2', avatar: '2'});

// Start battle
battle.start();

// Play turns until turn 27
for (let turn = 1; turn < 27; turn++) {
    const p1Request = battle.p1.activeRequest;
    const p2Request = battle.p2.activeRequest;

    if (!p1Request || !p2Request) {
        console.error('No request at turn', turn);
        break;
    }

    battle.choose('p1', 'default');
    battle.choose('p2', 'default');
}

// Play turn 27 with logging
console.log('=== TURN 27 START ===');
const prngBefore = battle.prng.rng.startingSeed.map((s, i) => [i, s]);
const callsBefore = battle.prng.callCount || 0;

const logBefore = battle.log.length;
battle.choose('p1', 'default');
battle.choose('p2', 'default');
const logAfter = battle.log.length;

console.log('\nTurn 27 log:');
for (let i = logBefore; i < logAfter; i++) {
    console.log(battle.log[i]);
}

const callsAfter = battle.prng.callCount || 0;
console.log(`\nTotal PRNG calls on turn 27: ${callsAfter - callsBefore}`);
