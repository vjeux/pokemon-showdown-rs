//! Common test utilities
//! Equivalent to pokemon-showdown-js/test/common.js

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, ID, PRNGSeed, Gender};

/// Default PRNG seed for consistent test results
pub const DEFAULT_SEED: PRNGSeed = PRNGSeed::Gen5([1, 2, 3, 4]);

/// Options for creating a test battle
#[derive(Default)]
pub struct CreateBattleOptions {
    pub preview: bool,
    pub game_type: Option<String>,
    pub seed: Option<PRNGSeed>,
}

/// Create a battle from team specs
/// Teams is an array of two arrays of Pokemon specs
pub fn create_battle(
    options: CreateBattleOptions,
    teams: [Vec<PokemonSpec>; 2],
) -> Battle {
    let [team1_specs, team2_specs] = teams;

    let team1: Vec<PokemonSet> = team1_specs.into_iter().map(|spec| spec.into()).collect();
    let team2: Vec<PokemonSet> = team2_specs.into_iter().map(|spec| spec.into()).collect();

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9customgame"),
        seed: options.seed.or(Some(DEFAULT_SEED)),
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

    // If preview is false, automatically submit team choices and start the battle
    if !options.preview {
        use pokemon_showdown::SideID;
        // Submit team choices for both sides (using default order)
        let _ = battle.choose(SideID::P1, "team 1");
        let _ = battle.choose(SideID::P2, "team 1");

        // Actually start the battle (send out Pokemon, transition to Move state)
        battle.start_battle();
    }

    battle
}

/// Simplified Pokemon specification for tests
#[derive(Clone)]
pub struct PokemonSpec {
    pub species: String,
    pub ability: String,
    pub moves: Vec<String>,
    pub level: Option<u8>,
    pub item: Option<String>,
    pub gender: Option<Gender>,
    pub nature: Option<String>,
    pub evs: Option<EVs>,
    pub ivs: Option<IVs>,
}

#[derive(Clone, Default)]
pub struct EVs {
    pub hp: Option<u8>,
    pub atk: Option<u8>,
    pub def: Option<u8>,
    pub spa: Option<u8>,
    pub spd: Option<u8>,
    pub spe: Option<u8>,
}

#[derive(Clone, Default)]
pub struct IVs {
    pub hp: Option<u8>,
    pub atk: Option<u8>,
    pub def: Option<u8>,
    pub spa: Option<u8>,
    pub spd: Option<u8>,
    pub spe: Option<u8>,
}

impl From<PokemonSpec> for PokemonSet {
    fn from(spec: PokemonSpec) -> Self {
        PokemonSet {
            name: spec.species.clone(),
            species: spec.species,
            level: spec.level.unwrap_or(100),
            ability: spec.ability,
            moves: spec.moves,
            item: spec.item.unwrap_or_default(),
            gender: spec.gender.unwrap_or(Gender::None),
            nature: spec.nature.unwrap_or_default(),
            ..Default::default()
        }
    }
}

/// Helper macro to create a PokemonSpec easily
#[macro_export]
macro_rules! pokemon {
    (
        species: $species:expr,
        ability: $ability:expr,
        moves: [$($move:expr),* $(,)?]
        $(, level: $level:expr)?
        $(, item: $item:expr)?
        $(, gender: $gender:expr)?
        $(, nature: $nature:expr)?
    ) => {
        common::PokemonSpec {
            species: $species.to_string(),
            ability: $ability.to_string(),
            moves: vec![$($move.to_string()),*],
            level: None $(.or(Some($level)))?,
            item: None $(.or(Some($item.to_string())))?,
            gender: None $(.or(Some($gender)))?,
            nature: None $(.or(Some($nature.to_string())))?,
            evs: None,
            ivs: None,
        }
    };
}
