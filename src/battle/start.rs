use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Start the battle
    /// Source: battle.ts:1859
    /// Deserialized games should use restart()
    //
    // 	start() {
    // 		// Deserialized games should use restart()
    // 		if (this.deserialized) return;
    // 		// need all players to start
    // 		if (!this.sides.every(side => !!side)) throw new Error(`Missing sides: ${this.sides}`);
    //
    // 		if (this.started) throw new Error(`Battle already started`);
    //
    // 		const format = this.format;
    // 		this.started = true;
    // 		if (this.gameType === 'multi') {
    // 			this.sides[1].foe = this.sides[2]!;
    // 			this.sides[0].foe = this.sides[3]!;
    // 			this.sides[2]!.foe = this.sides[1];
    // 			this.sides[3]!.foe = this.sides[0];
    // 			this.sides[1].allySide = this.sides[3]!;
    // 			this.sides[0].allySide = this.sides[2]!;
    // 			this.sides[2]!.allySide = this.sides[0];
    // 			this.sides[3]!.allySide = this.sides[1];
    // 			// sync side conditions
    // 			this.sides[2]!.sideConditions = this.sides[0].sideConditions;
    // 			this.sides[3]!.sideConditions = this.sides[1].sideConditions;
    // 		} else {
    // 			this.sides[1].foe = this.sides[0];
    // 			this.sides[0].foe = this.sides[1];
    // 			if (this.sides.length > 2) { // ffa
    // 				this.sides[2]!.foe = this.sides[3]!;
    // 				this.sides[3]!.foe = this.sides[2]!;
    // 			}
    // 		}
    //
    // 		this.add('gen', this.gen);
    //
    // 		this.add('tier', format.name);
    // 		if (this.rated) {
    // 			if (this.rated === 'Rated battle') this.rated = true;
    // 			this.add('rated', typeof this.rated === 'string' ? this.rated : '');
    // 		}
    //
    // 		format.onBegin?.call(this);
    // 		for (const rule of this.ruleTable.keys()) {
    // 			if ('+*-!'.includes(rule.charAt(0))) continue;
    // 			const subFormat = this.dex.formats.get(rule);
    // 			subFormat.onBegin?.call(this);
    // 		}
    //
    // 		if (this.sides.some(side => !side.pokemon[0])) {
    // 			throw new Error('Battle not started: A player has an empty team.');
    // 		}
    //
    // 		if (this.debugMode) {
    // 			this.checkEVBalance();
    // 		}
    //
    // 		if (format.customRules) {
    // 			const plural = format.customRules.length === 1 ? '' : 's';
    // 			const open = format.customRules.length <= 5 ? ' open' : '';
    // 			this.add(`raw|<div class="infobox"><details class="readmore"${open}><summary><strong>${format.customRules.length} custom rule${plural}:</strong></summary> ${format.customRules.join(', ')}</details></div>`);
    // 		}
    //
    // 		this.runPickTeam();
    // 		this.queue.addChoice({ choice: 'start' });
    // 		this.midTurn = true;
    // 		if (!this.requestState) this.turnLoop();
    // 	}
    //
    pub fn start(&mut self) {
        // JS: if (this.deserialized) return;
        if self.deserialized {
            return;
        }

        // JS: if (!this.sides.every(side => !!side)) throw new Error(`Missing sides`);
        // Rust: Check that all sides exist and are valid
        if self.sides.is_empty() {
            panic!("Missing sides");
        }

        // JS: if (this.started) throw new Error(`Battle already started`);
        if self.started {
            panic!("Battle already started");
        }

        // JS: this.started = true;
        self.started = true;

        // JS: Set up foe and ally references based on game type
        match self.game_type {
            GameType::Multi => {
                // JS: Multi battles (4 sides)
                if self.sides.len() >= 4 {
                    // JS: this.sides[1].foe = this.sides[2];
                    self.sides[1].foe_index = Some(2);
                    // JS: this.sides[0].foe = this.sides[3];
                    self.sides[0].foe_index = Some(3);
                    // JS: this.sides[2].foe = this.sides[1];
                    self.sides[2].foe_index = Some(1);
                    // JS: this.sides[3].foe = this.sides[0];
                    self.sides[3].foe_index = Some(0);

                    // JS: this.sides[1].allySide = this.sides[3];
                    self.sides[1].ally_index = Some(3);
                    // JS: this.sides[0].allySide = this.sides[2];
                    self.sides[0].ally_index = Some(2);
                    // JS: this.sides[2].allySide = this.sides[0];
                    self.sides[2].ally_index = Some(0);
                    // JS: this.sides[3].allySide = this.sides[1];
                    self.sides[3].ally_index = Some(1);

                    // JS: sync side conditions
                    // JS: this.sides[2].sideConditions = this.sides[0].sideConditions;
                    let side_0_conditions = self.sides[0].side_conditions.clone();
                    self.sides[2].side_conditions = side_0_conditions;
                    // JS: this.sides[3].sideConditions = this.sides[1].sideConditions;
                    let side_1_conditions = self.sides[1].side_conditions.clone();
                    self.sides[3].side_conditions = side_1_conditions;
                }
            }
            GameType::FreeForAll => {
                // JS: FFA battles
                if self.sides.len() >= 4 {
                    // JS: this.sides[2].foe = this.sides[3];
                    self.sides[2].foe_index = Some(3);
                    // JS: this.sides[3].foe = this.sides[2];
                    self.sides[3].foe_index = Some(2);
                }
                // Fall through to set up sides 0 and 1
                if self.sides.len() >= 2 {
                    // JS: this.sides[1].foe = this.sides[0];
                    self.sides[1].foe_index = Some(0);
                    // JS: this.sides[0].foe = this.sides[1];
                    self.sides[0].foe_index = Some(1);
                }
            }
            _ => {
                // JS: Singles/Doubles battles (2 sides)
                if self.sides.len() >= 2 {
                    // JS: this.sides[1].foe = this.sides[0];
                    self.sides[1].foe_index = Some(0);
                    // JS: this.sides[0].foe = this.sides[1];
                    self.sides[0].foe_index = Some(1);
                }
            }
        }

        // JS: this.add('gen', this.gen);
        self.add_log("gen", &[&self.gen.to_string()]);

        // JS: this.add('tier', format.name);
        let format_name = self.format_name.clone();
        self.add_log("tier", &[&format_name]);

        // JS: if (this.rated) { ... }
        if let Some(ref rated) = self.rated.clone() {
            // JS: if (this.rated === 'Rated battle') this.rated = true;
            let rated_str = if rated == "Rated battle" { "" } else { rated };
            // JS: this.add('rated', typeof this.rated === 'string' ? this.rated : '');
            self.add_log("rated", &[rated_str]);
        }

        // JS: format.onBegin?.call(this);
        // TODO: Format callbacks require a callback registration system
        // JavaScript formats can have onBegin, onBeforeMove, onAfterMove, etc.
        // These cannot be deserialized from JSON - they must be registered separately
        // Similar to event system: Battle::on_event("FormatBegin", callback)

        // For now, emit an event that format-specific code can hook into
        self.run_event("FormatBegin", None, None, None, None);

        // JS: for (const rule of this.ruleTable.keys()) { subFormat.onBegin?.call(this); }
        if let Some(ref rule_table) = self.rule_table {
            // Collect rule keys to avoid borrow checker issues
            let rule_keys: Vec<String> = rule_table.keys().cloned().collect();

            for rule in rule_keys {
                // JS: if ('+*-!'.includes(rule.charAt(0))) continue;
                if let Some(first_char) = rule.chars().next() {
                    if first_char == '+'
                        || first_char == '*'
                        || first_char == '-'
                        || first_char == '!'
                    {
                        continue;
                    }
                }

                // JS: const subFormat = this.dex.formats.get(rule);
                // JS: subFormat.onBegin?.call(this);
                // TODO: Look up subFormat from Dex and invoke its onBegin callback
                // This requires:
                // 1. Dex::get_format() method to look up formats by ID
                // 2. Format callback registration system
                // For now, emit an event that rule-specific code can hook into
                self.run_event(&format!("RuleBegin:{}", rule), None, None, None, None);
            }
        }

        // JS: if (this.sides.some(side => !side.pokemon[0])) { throw new Error('...'); }
        if self.sides.iter().any(|side| side.pokemon.is_empty()) {
            panic!("Battle not started: A player has an empty team.");
        }

        // JS: if (this.debugMode) { this.checkEVBalance(); }
        if self.debug_mode {
            self.check_ev_balance();
        }

        // JS: if (format.customRules) { this.add(`raw|...`); }
        // Find the format and display custom rules if they exist
        if let Some(format) = self.dex.formats.iter().find(|f| {
            ID::new(&f.name.to_lowercase().replace(" ", "").replace("-", "")) == self.format_id
        }) {
            if let Some(custom_rules) = &format.custom_rules {
                if !custom_rules.is_empty() {
                    // JS: const plural = format.customRules.length === 1 ? '' : 's';
                    let plural = if custom_rules.len() == 1 { "" } else { "s" };
                    // JS: const open = format.customRules.length <= 5 ? ' open' : '';
                    let open = if custom_rules.len() <= 5 { " open" } else { "" };
                    // JS: this.add(`raw|<div class="infobox"><details class="readmore"${open}><summary><strong>${format.customRules.length} custom rule${plural}:</strong></summary> ${format.customRules.join(', ')}</details></div>`);
                    let rules_joined = custom_rules.join(", ");
                    self.add_log("raw", &[&format!("<div class=\"infobox\"><details class=\"readmore\"{open}><summary><strong>{} custom rule{plural}:</strong></summary> {rules_joined}</details></div>", custom_rules.len())]);
                }
            }
        }

        // JS: this.runPickTeam();
        self.run_pick_team();

        eprintln!("=== AFTER RUN_PICK_TEAM, REQUEST_STATE: {:?} ===", self.request_state);

        // JS: this.queue.addChoice({ choice: 'start' });
        use crate::battle_queue::{Action, FieldAction, FieldActionType};
        self.queue.add_choice_raw(Action::Field(FieldAction {
            choice: FieldActionType::Start,
            priority: 0,
        }));

        eprintln!("=== QUEUE LENGTH AFTER ADDING START: {} ===", self.queue.list.len());

        // JS: this.midTurn = true;
        self.mid_turn = true;

        // JS: if (!this.requestState) this.turnLoop();
        eprintln!("=== CHECKING IF SHOULD CALL TURN_LOOP: request_state={:?} ===", self.request_state);
        if self.request_state == BattleRequestState::None {
            eprintln!("=== CALLING TURN_LOOP FROM START() ===");
            self.turn_loop();
        } else {
            eprintln!("=== NOT CALLING TURN_LOOP, REQUEST_STATE IS NOT NONE ===");
        }
    }
}
