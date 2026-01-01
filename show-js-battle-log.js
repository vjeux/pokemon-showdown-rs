const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([1, 1, 1, 1]);

let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    return originalNext();
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
    const prngThisTurn = prngAfter - prngBefore;

    // Show all turns with their logs
    console.log(`\nTurn ${turnsSinceStart} (PRNG: ${prngBefore} -> ${prngAfter}, ${prngThisTurn} calls):`);
    for (let i = logBefore; i < logAfter; i++) {
        const line = battle.log[i];
        if (line.startsWith('|move|') || line.startsWith('|switch|') ||
            line.startsWith('|-damage|') || line.startsWith('|faint|') ||
            line.startsWith('|turn|') || line.startsWith('|-heal|')) {
            console.log('  ' + line);
        }
    }

    if (turnsSinceStart > 35) break;
}
