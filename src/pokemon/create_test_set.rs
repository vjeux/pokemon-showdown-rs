use crate::*;


    pub fn create_test_set() -> PokemonSet {
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            item: "Light Ball".to_string(),
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string(), "Volt Tackle".to_string()],
            level: 50,
            gender: Gender::Male,
            ..Default::default()
        }
    }
