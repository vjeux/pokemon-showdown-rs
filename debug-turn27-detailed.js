const {Battle} = require('./pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./pokemon-showdown-ts/dist/sim/prng');

const battle = new Battle({formatid: 'gen9randombattle'});

// Track PRNG calls
let prngCallCount = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    const result = originalNext();
    prngCallCount++;
    console.log(`[PRNG CALL #${prngCallCount}] next() -> ${result} (seed: ${this.s0},${this.s1},${this.s2},${this.s3})`);
    console.trace();
    return result;
};

// Use seed 1
battle.prng = new PRNG([1, 1, 1, 1]);
battle.prng.rng.next = function() {
    const result = originalNext.call(this);
    prngCallCount++;
    console.log(`[PRNG CALL #${prngCallCount}] next() -> ${result} (seed: ${this.s0},${this.s1},${this.s2},${this.s3})`);
    console.trace();
    return result;
};

battle.setPlayer('p1', {name: 'Player 1', avatar: '1'});
battle.setPlayer('p2', {name: 'Player 2', avatar: '2'});

const stream = battle.getDebugLog();
stream.on('data', data => {
    if (data.startsWith('|turn|')) {
        const turn = parseInt(data.split('|')[2]);
        console.log(`\n=== TURN ${turn} START ===`);
    }
});

// Play until turn 26
let turnNum = 0;
battle.inputLog = [];

console.log('=== PLAYING UNTIL TURN 27 ===');

// Start battle
battle.start();
turnNum = 1;

// Play turns until turn 26
while (turnNum < 27) {
    const p1Request = battle.p1.activeRequest;
    const p2Request = battle.p2.activeRequest;

    if (!p1Request || !p2Request) {
        console.error('No request at turn', turnNum);
        break;
    }

    // Make random choices
    const p1Choice = 'move 1';
    const p2Choice = 'move 1';

    battle.choose('p1', p1Choice);
    battle.choose('p2', p2Choice);

    turnNum++;
    if (turnNum === 27) {
        console.log('\n=== TURN 27 - DETAILED TRACE ===');
        console.log('About to make choices for turn 27');
        prngCallCount = 0; // Reset counter for turn 27
    }
}

console.log(`\n=== TURN 27 COMPLETE ===`);
console.log(`Total PRNG calls on turn 27: ${prngCallCount}`);
