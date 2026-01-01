const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([1, 1, 1, 1]);

let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    const result = originalNext();
    if (totalPrngCalls >= 114 && totalPrngCalls <= 119) {
        const stack = new Error().stack.split('\n')[3];
        const match = stack.match(/at (\w+\.)?(\w+) /);
        const func = match ? match[2] : 'unknown';
        console.log(`[PRNG #${totalPrngCalls}] ${func}`);
    }
    return result;
};

battle.setPlayer('p1', {name: 'Player 1', avatar: '1'});
battle.setPlayer('p2', {name: 'Player 2', avatar: '2'});

let turnsSinceStart = 0;
while (battle.p1.activeRequest && battle.p2.activeRequest) {
    const prngBefore = totalPrngCalls;
    const logBefore = battle.log.length;

    battle.choose('p1', 'default');
    battle.choose('p2', 'default');
    turnsSinceStart++;

    const logAfter = battle.log.length;
    const prngAfter = totalPrngCalls;

    if (prngBefore <= 113 && prngAfter >= 113) {
        console.log(`\n=== Turn ${turnsSinceStart} crosses PRNG 113 (${prngBefore} -> ${prngAfter}) ===`);
        for (let i = logBefore; i < logAfter; i++) {
            const line = battle.log[i];
            if (line.startsWith('|move|') || line.startsWith('|switch|') ||
                line.startsWith('|-damage|') || line.startsWith('|faint|') ||
                line.startsWith('|turn|')) {
                console.log(line);
            }
        }
    }

    if (prngAfter > 120) break;
}
