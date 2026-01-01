const jsTrace = require('./trace-js-seed1.json');
const rustTrace = require('./trace-rust-seed1.json');

console.log('=== Turn 26 State ===');
console.log('\nJavaScript Turn 26:');
const jsTurn26 = jsTrace.states.find(s => s.turn === 26);
if (jsTurn26) {
    jsTurn26.state.sides.forEach((side, idx) => {
        console.log(`${side.name}:`);
        side.pokemon.forEach(p => {
            if (p.active) {
                console.log(`  ${p.name}: ${p.hp}/${p.maxhp} HP (active)`);
            }
        });
    });
}

console.log('\nRust Turn 26:');
const rustTurn26 = rustTrace.states.find(s => s.turn === 26);
if (rustTurn26) {
    rustTurn26.state.sides.forEach((side, idx) => {
        console.log(`${side.name}:`);
        side.pokemon.forEach(p => {
            if (p.active) {
                console.log(`  ${p.name}: ${p.hp}/${p.maxhp} HP (active)`);
            }
        });
    });
}

console.log('\n=== Turn 27 State (after turn) ===');
console.log('\nJavaScript Turn 27:');
const jsTurn27 = jsTrace.states.find(s => s.turn === 27);
if (jsTurn27) {
    jsTurn27.state.sides.forEach((side, idx) => {
        console.log(`${side.name}:`);
        side.pokemon.forEach(p => {
            if (p.active) {
                console.log(`  ${p.name}: ${p.hp}/${p.maxhp} HP (active, fainted=${p.fainted})`);
            }
        });
    });
}

console.log('\nRust Turn 27:');
const rustTurn27 = rustTrace.states.find(s => s.turn === 27);
if (rustTurn27) {
    rustTurn27.state.sides.forEach((side, idx) => {
        console.log(`${side.name}:`);
        side.pokemon.forEach(p => {
            if (p.active) {
                console.log(`  ${p.name}: ${p.hp}/${p.maxhp} HP (active, fainted=${p.fainted})`);
            }
        });
    });
}
