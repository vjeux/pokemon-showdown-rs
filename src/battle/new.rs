use crate::*;
use crate::battle::BattleRequestState;
use crate::battle_queue::BattleQueue;
use crate::data::formats::{get_format, DexFormats, Format};
use crate::field::Field;
use std::collections::HashSet;

impl Battle {
    /// Create a new battle
    /// Equivalent to TypeScript Battle constructor (battle.ts:191)
    pub fn new(options: BattleOptions) -> Self {
        eprintln!("=== BATTLE INIT START ===");
        let seed = options.seed.clone().unwrap_or_else(PRNG::generate_seed);
        eprintln!("=== BATTLE PRNG SEED: {:?} ===", seed);
        let prng = PRNG::new(Some(seed.clone()));

        // Clone format_id before moving it into the struct
        let format_id_str = options.format_id.as_str().to_string();

        // Determine game settings from format
        let game_type = options.game_type.unwrap_or(GameType::Singles);
        let gen = 9; // Default to latest gen
        let active_per_half = match game_type {
            GameType::Triples => 3,
            GameType::Doubles | GameType::Multi | GameType::FreeForAll => 2,
            _ => 1,
        };
        let player_count = match game_type {
            GameType::Multi | GameType::FreeForAll => 4,
            _ => 2,
        };

        let sides = Vec::new(); // Sides will be added via set_player

        // Load dex
        let dex = crate::dex::Dex::load_default().expect("Failed to load Dex");

        // Load rule_table from format
        // JavaScript: this.ruleTable = this.dex.formats.getRuleTable(this.format);
        let rule_table = if let Some(format_def) = get_format(&ID::new(&format_id_str)) {
            // Create a Format from the FormatDef
            let format = Format::from_def(format_def);
            // Create DexFormats and get the rule table
            let dex_formats = DexFormats::new();
            Some(dex_formats.get_rule_table(&format))
        } else {
            None
        };

        let mut battle = Self {
            format_id: options.format_id,
            format_name: options.format_name.unwrap_or_else(|| format_id_str.clone()),
            game_type,
            gen,
            active_per_half,
            dex,
            rule_table,
            field: Field::new(),
            sides,
            queue: BattleQueue::new(),
            prng,
            prng_seed: seed.clone(),
            log: Vec::new(),
            input_log: Vec::new(),
            request_state: BattleRequestState::None,
            sent_requests: false, // JavaScript: sentRequests defaults to false
            turn: 0,
            mid_turn: false,
            started: false,
            ended: false,
            deserialized: false,
            winner: None,
            last_move: None,
            last_successful_move_this_turn: None,
            last_move_line: -1,
            last_damage: 0,
            active_move: None,
            active_pokemon: None,
            active_target: None,
            effect_order: 0,
            event_depth: 0,
            current_event: None,
            current_effect: None,
            current_effect_state: None,
            current_effect_data: None,
            sent_log_pos: 0,
            debug_mode: options.debug,
            rated: options.rated,
            strict_choices: options.strict_choices,
            support_cancel: false, // JavaScript default: supportCancel defaults to false
            force_random_chance: if options.debug {
                options.force_random_chance
            } else {
                None
            },
            hints: HashSet::new(),
            events: std::collections::HashMap::new(),
            faint_queue: Vec::new(),
        };

        // Initialize sides vector
        for _ in 0..player_count {
            // Placeholder - will be filled by set_player
        }

        // Log start
        battle.input_log.push(format!(
            ">start {{\"formatid\":\"{}\",\"seed\":\"{}\"}}",
            format_id_str, seed
        ));
        battle.add_log("gametype", &[&match game_type {
            GameType::Singles => "singles",
            GameType::Doubles => "doubles",
            GameType::Triples => "triples",
            GameType::Rotation => "rotation",
            GameType::Multi => "multi",
            GameType::FreeForAll => "freeforall",
        }]);

        // Add players if provided
        if let Some(p1) = options.p1 {
            battle.set_player(SideID::P1, p1);
        }
        if let Some(p2) = options.p2 {
            battle.set_player(SideID::P2, p2);
        }
        if let Some(p3) = options.p3 {
            battle.set_player(SideID::P3, p3);
        }
        if let Some(p4) = options.p4 {
            battle.set_player(SideID::P4, p4);
        }

        battle
    }
}
