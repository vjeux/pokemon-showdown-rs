use crate::*;
use crate::dex_data::BoostsTable;
use crate::event_system::EffectState;
use crate::dex_data::StatsTable;
use crate::pokemon::MoveSlot;
use std::collections::HashMap;

impl Pokemon {
    /// Create a new Pokemon from a PokemonSet
    pub fn new(set: &PokemonSet, side_index: usize, position: usize) -> Self {
        let species_id = ID::new(&set.species);
        let ability_id = ID::new(&set.ability);
        let item_id = ID::new(&set.item);

        // Convert moves to move slots
        let move_slots: Vec<MoveSlot> = set
            .moves
            .iter()
            .map(|m| {
                let id = ID::new(m);
                MoveSlot::new(id, m.clone(), 5, 5) // Default PP, will be set by Dex
            })
            .collect();

        Self {
            name: if set.name.is_empty() {
                set.species.clone()
            } else {
                set.name.clone()
            },
            species_id: species_id.clone(),
            base_species: species_id.clone(),
            level: set.level,
            gender: set.gender,
            nature: set.nature.clone(),
            happiness: set.happiness,
            pokeball: ID::new(&set.pokeball),
            dynamax_level: set.dynamax_level,
            gigantamax: set.gigantamax,

            position,
            side_index,
            is_active: false,

            base_stored_stats: StatsTable::default(),
            stored_stats: StatsTable::default(),
            boosts: BoostsTable::new(),
            // JS Pokemon constructor doesn't set maxhp/hp directly
            // They're calculated later in clearVolatile() -> setSpecies() -> spreadModify()
            // Initialize to 0 so setSpecies() knows to calculate them
            maxhp: 0,
            base_maxhp: 0,
            max_hp_undynamaxed: 0,
            hp: 0,

            base_ability: ability_id.clone(),
            ability: ability_id,
            ability_state: EffectState::new(ID::empty()),

            item: item_id,
            item_state: EffectState::new(ID::empty()),
            last_item: ID::empty(),
            used_item_this_turn: false,
            ate_berry: false,
            item_knocked_off: false,

            locked_move: None,

            types: Vec::new(),
            added_type: None,
            base_types: Vec::new(),
            known_type: None, // Initially null, set when type becomes known (e.g., Illusion breaks)
            apparent_type: None, // Initially null, set when type is revealed to players

            tera_type: set.tera_type.clone(),
            terastallized: None,
            can_terastallize: set.tera_type.clone(),

            base_move_slots: move_slots.clone(),
            move_slots,

            hp_type: set.hptype.clone(),

            status: ID::empty(),
            status_state: EffectState::new(ID::empty()),
            volatiles: HashMap::new(),

            fainted: false,
            faint_queued: false,
            transformed: false,
            illusion: None,

            trapped: false,
            maybe_trapped: false,
            maybe_disabled: false,
            maybe_locked: false,
            switch_flag: false,
            force_switch_flag: false,
            newly_switched: false,
            being_called_back: false,
            dragged_in: None,
            skip_before_switch_out_event_flag: false,
            stats_raised_this_turn: false,
            stats_lowered_this_turn: false,
            sub_fainted: false,

            sword_boost: false,

            last_move: None,
            last_move_used: None,
            last_move_target_loc: None,
            move_this_turn: None,
            move_this_turn_result: None,
            move_last_turn_result: None,
            hurt_this_turn: None,
            last_damage: 0,
            times_attacked: 0,

            active_turns: 0,
            active_move_actions: 0,
            previously_switched_in: 0,
            is_started: false,
            during_move: false,

            attacked_by: Vec::new(),

            weight_hg: 0,
            speed: 0,

            can_mega_evo: None,
            can_ultra_burst: None,
            can_gigantamax: if set.gigantamax {
                Some(set.species.clone())
            } else {
                None
            },

            stellar_boosted_types: Vec::new(),

            staleness: None,
            pending_staleness: None,
            volatile_staleness: None,
        }
    }
}
