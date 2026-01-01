const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([1, 1, 1, 1]);
battle.setPlayer('p1', {name: 'Player 1', avatar: '1'});
battle.setPlayer('p2', {name: 'Player 2', avatar: '2'});

// Play until just before turn 32
let turn = 1;
while (turn < 32) {
    if (!battle.p1.activeRequest || !battle.p2.activeRequest) break;
    battle.choose('p1', 'default');
    battle.choose('p2', 'default');
    turn++;
}

if (!battle.p1.activeRequest || !battle.p2.activeRequest) {
    console.log('Battle ended before Turn 32');
    process.exit(0);
}

// Execute turn 32
console.log(`\n=== TURN 32 START (actual turn ${battle.turn}) ===`);
const logStart = battle.log.length;
battle.choose('p1', 'default');
battle.choose('p2', 'default');
const logEnd = battle.log.length;

console.log('\nTurn 32 battle log:');
for (let i = logStart; i < logEnd; i++) {
    console.log(battle.log[i]);
}
