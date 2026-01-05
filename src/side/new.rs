// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {
    /// Create a new side
    pub fn new(
        id: SideID,
        n: usize,
        name: String,
        team: Vec<PokemonSet>,
        active_count: usize,
        dex: &crate::dex::Dex,
    ) -> Self {
        let pokemon: Vec<Pokemon> = team
            .iter()
            .enumerate()
            .map(|(i, set)| Pokemon::new(set, n, i, dex))
            .collect();

        let pokemon_left = pokemon.len();
        let slot_conditions = (0..active_count).map(|_| HashMap::new()).collect();

        Self {
            id,
            n,
            name,
            avatar: String::new(),
            team,
            pokemon,
            active: vec![None; active_count],
            pokemon_left,
            z_move_used: false,
            dynamax_used: false,
            fainted_last_turn: None,
            fainted_this_turn: None,
            total_fainted: 0,
            last_selected_move: ID::empty(),
            last_move: None,
            side_conditions: HashMap::new(),
            slot_conditions,
            request_state: RequestState::None,
            active_request: None,
            choice: Choice::new(),
            foe_index: None,
            ally_index: None,
        }
    }
}
