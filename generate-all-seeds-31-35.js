const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

// Unified generator for battle test infrastructure
// Creates teams, Rust runners, and JavaScript runners
const START_SEED = 31;
const END_SEED = 35;

console.log(`Generating complete test infrastructure for seeds ${START_SEED}-${END_SEED}...\n`);

for (let seedNum = START_SEED; seedNum <= END_SEED; seedNum++) {
    console.log(`\n=== Seed ${seedNum} ===`);

    // Generate teams
    const battle = new Battle({formatid: 'gen9randombattle'});
    battle.prng = new PRNG([0, 0, 0, seedNum]);

    battle.setPlayer('p1', {name: 'Player 1', team: null});
    battle.setPlayer('p2', {name: 'Player 2', team: null});

    const p1Team = battle.sides[0].pokemon.map(p => ({
        name: p.name,
        species: p.species.name,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.set.nature || p.baseNature || 'serious',
        gender: p.gender === 'M' ? 'M' : p.gender === 'F' ? 'F' : '',
        moves: p.moves,
        evs: p.set.evs,
        ivs: p.set.ivs,
    }));

    const p2Team = battle.sides[1].pokemon.map(p => ({
        name: p.name,
        species: p.species.name,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.set.nature || p.baseNature || 'serious',
        gender: p.gender === 'M' ? 'M' : p.gender === 'F' ? 'F' : '',
        moves: p.moves,
        evs: p.set.evs,
        ivs: p.set.ivs,
    }));

    const teams = {p1: p1Team, p2: p2Team};

    // Write team files
    fs.writeFileSync(`teams-seed${seedNum}-js.json`, JSON.stringify(teams, null, 2));
    fs.writeFileSync(`teams-seed${seedNum}-rust.json`, JSON.stringify(teams, null, 2));
    console.log(`  Teams: ${p1Team[0].name} vs ${p2Team[0].name}`);

    // Generate Rust test runner
    const rustCode = `/// Compare battle with seed ${seedNum} - Rust version
use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;

fn main() {
    let teams_json = fs::read_to_string("teams-seed${seedNum}-rust.json").unwrap();

    #[derive(serde::Deserialize)]
    struct Teams {
        p1: Vec<TestPokemon>,
        p2: Vec<TestPokemon>,
    }

    #[derive(serde::Deserialize)]
    struct TestPokemon {
        name: String,
        species: String,
        level: u8,
        ability: String,
        item: String,
        nature: String,
        gender: String,
        moves: Vec<String>,
        evs: TestStats,
        ivs: TestStats,
    }

    #[derive(serde::Deserialize)]
    struct TestStats {
        hp: i32,
        atk: i32,
        def: i32,
        spa: i32,
        spd: i32,
        spe: i32,
    }

    let teams: Teams = serde_json::from_str(&teams_json).unwrap();

    let team1: Vec<_> = teams.p1.iter().map(|set| PokemonSet {
        name: set.name.clone(),
        species: set.species.clone(),
        level: set.level,
        ability: set.ability.clone(),
        item: set.item.clone(),
        nature: set.nature.clone(),
        gender: match set.gender.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        },
        moves: set.moves.clone(),
        evs: StatsTable::new(set.evs.hp, set.evs.atk, set.evs.def, set.evs.spa, set.evs.spd, set.evs.spe),
        ivs: StatsTable::new(set.ivs.hp, set.ivs.atk, set.ivs.def, set.ivs.spa, set.ivs.spd, set.ivs.spe),
        ..Default::default()
    }).collect();

    let team2: Vec<_> = teams.p2.iter().map(|set| PokemonSet {
        name: set.name.clone(),
        species: set.species.clone(),
        level: set.level,
        ability: set.ability.clone(),
        item: set.item.clone(),
        nature: set.nature.clone(),
        gender: match set.gender.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        },
        moves: set.moves.clone(),
        evs: StatsTable::new(set.evs.hp, set.evs.atk, set.evs.def, set.evs.spa, set.evs.spd, set.evs.spe),
        ivs: StatsTable::new(set.ivs.hp, set.ivs.atk, set.ivs.def, set.ivs.spa, set.ivs.spd, set.ivs.spe),
        ..Default::default()
    }).collect();

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, 0, ${seedNum}])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
            rating: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
            rating: None,
        }),
        ..Default::default()
    });

    eprintln!("RUST: Running battle with seed [0, 0, 0, ${seedNum}]");

    for make_choices_num in 1..=100 {
        let prng_before = battle.prng.call_count;

        battle.make_choices(&["default", "default"]);

        let prng_after = battle.prng.call_count;

        // Print HP of all active Pokemon
        let mut p1_active = Vec::new();
        let mut p2_active = Vec::new();

        for active_idx in &battle.sides[0].active {
            if let Some(poke_idx) = active_idx {
                if let Some(pokemon) = battle.sides[0].pokemon.get(*poke_idx) {
                    p1_active.push(format!("{}({}/{})", pokemon.name, pokemon.hp, pokemon.maxhp));
                }
            }
        }

        for active_idx in &battle.sides[1].active {
            if let Some(poke_idx) = active_idx {
                if let Some(pokemon) = battle.sides[1].pokemon.get(*poke_idx) {
                    p2_active.push(format!("{}({}/{})", pokemon.name, pokemon.hp, pokemon.maxhp));
                }
            }
        }

        eprintln!("#{}: turn={}, prng={}->{}, P1=[{}], P2=[{}]",
            make_choices_num, battle.turn, prng_before, prng_after,
            p1_active.join(", "), p2_active.join(", "));

        if battle.ended || make_choices_num >= 100 {
            eprintln!("\\nRUST Battle status:");
            eprintln!("  Ended: {}", battle.ended);
            eprintln!("  Turn: {}", battle.turn);
            eprintln!("  Total PRNG calls: {}", battle.prng.call_count);
            break;
        }
    }
}
`;

    fs.writeFileSync(`examples/compare_seed${seedNum}.rs`, rustCode);
    console.log(`  Created: examples/compare_seed${seedNum}.rs`);

    // Generate JavaScript test runner
    const jsCode = `const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, ${seedNum}]);

// Load teams from JSON file
const teams = JSON.parse(fs.readFileSync('teams-seed${seedNum}-js.json', 'utf8'));

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

// Wrap PRNG to count calls
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    return originalNext();
};

console.log('JS: Running battle with seed [0, 0, 0, ${seedNum}]');

for (let i = 1; i <= 100; i++) {
    const prngBefore = totalPrngCalls;

    battle.makeChoices('default', 'default');

    const prngAfter = totalPrngCalls;

    // Get active Pokemon
    const p1Active = battle.sides[0].active
        .map(p => p ? \`\${p.name}(\${p.hp}/\${p.maxhp})\` : 'none')
        .join(', ');
    const p2Active = battle.sides[1].active
        .map(p => p ? \`\${p.name}(\${p.hp}/\${p.maxhp})\` : 'none')
        .join(', ');

    console.log(\`#\${i}: turn=\${battle.turn}, prng=\${prngBefore}->\${prngAfter}, P1=[\${p1Active}], P2=[\${p2Active}]\`);

    if (battle.ended || i >= 100) {
        console.log('\\nJS Battle status:');
        console.log(\`  Ended: \${battle.ended}\`);
        console.log(\`  Turn: \${battle.turn}\`);
        console.log(\`  Total PRNG calls: \${totalPrngCalls}\`);
        break;
    }
}
`;

    fs.writeFileSync(`compare-seed${seedNum}.js`, jsCode);
    console.log(`  Created: compare-seed${seedNum}.js`);
}

console.log(`\n✅ All test infrastructure generated for seeds ${START_SEED}-${END_SEED}!`);
