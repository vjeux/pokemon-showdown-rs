use crate::*;
use crate::event::EventResult;
use crate::battle::BattleRequestState;

impl Battle {

    /// Handle team preview phase
    /// Equivalent to battle.ts runPickTeam()
    ///
    //
    // 	runPickTeam() {
    // 		// onTeamPreview handlers are expected to show full teams to all active sides,
    // 		// and send a 'teampreview' request for players to pick their leads / team order.
    // 		this.format.onTeamPreview?.call(this);
    // 		for (const rule of this.ruleTable.keys()) {
    // 			if ('+*-!'.includes(rule.charAt(0))) continue;
    // 			const subFormat = this.dex.formats.get(rule);
    // 			subFormat.onTeamPreview?.call(this);
    // 		}
    //
    // 		if (this.requestState === 'teampreview') {
    // 			return;
    // 		}
    //
    // 		if (this.ruleTable.pickedTeamSize) {
    // 			// There was no onTeamPreview handler (e.g. Team Preview rule missing).
    // 			// Players must still pick their own Pokémon, so we show them privately.
    // 			this.add('clearpoke');
    // 			for (const pokemon of this.getAllPokemon()) {
    // 				// Still need to hide these formes since they change on battle start
    // 				const details = pokemon.details.replace(', shiny', '')
    // 					.replace(/(Zacian|Zamazenta)(?!-Crowned)/g, '$1-*')
    // 					.replace(/(Xerneas)(-[a-zA-Z?-]+)?/g, '$1-*');
    // 				this.addSplit(pokemon.side.id, ['poke', pokemon.side.id, details, '']);
    // 			}
    // 			this.makeRequest('teampreview');
    // 		}
    // 	}
    //
    pub fn run_pick_team(&mut self) {
        // JS: this.format.onTeamPreview?.call(this);
        // TODO: Implement format.onTeamPreview callback

        // JS: for (const rule of this.ruleTable.keys()) { ... subFormat.onTeamPreview?.call(this); }
        // TODO: Implement ruleTable iteration and subFormat.onTeamPreview callbacks

        // JS: if (this.requestState === 'teampreview') { return; }
        if matches!(self.request_state, BattleRequestState::TeamPreview) {
            return;
        }

        // JS: if (this.ruleTable.pickedTeamSize) { ... }
        // If pickedTeamSize is set, show Pokemon privately (no onTeamPreview handler ran)
        if let Some(ref rule_table) = self.rule_table {
            if rule_table.picked_team_size.is_some() {
                // There was no onTeamPreview handler (e.g. Team Preview rule missing).
                // Players must still pick their own Pokémon, so we show them privately.
                // JS: this.add('clearpoke');
                self.add("clearpoke", &[]);

                // JS: for (const pokemon of this.getAllPokemon()) { ... }
                // Collect Pokemon data first to avoid borrow checker issues
                let all_pokemon = self.get_all_pokemon();
                let pokemon_data: Vec<(String, usize)> = all_pokemon
                    .iter()
                    .map(|pokemon| {
                        // Still need to hide these formes since they change on battle start
                        // JS: const details = pokemon.details.replace(', shiny', '')
                        //         .replace(/(Zacian|Zamazenta)(?!-Crowned)/g, '$1-*')
                        //         .replace(/(Xerneas)(-[a-zA-Z?-]+)?/g, '$1-*');
                        let mut details = pokemon.details().clone();
                        details = details.replace(", shiny", "");

                        // Hide Zacian/Zamazenta forme (becomes Crowned on battle start)
                        if details.contains("Zacian") && !details.contains("Zacian-Crowned") {
                            details = details.replace("Zacian", "Zacian-*");
                        }
                        if details.contains("Zamazenta") && !details.contains("Zamazenta-Crowned") {
                            details = details.replace("Zamazenta", "Zamazenta-*");
                        }

                        // Hide Xerneas forme - replace "Xerneas" or "Xerneas-<forme>" with "Xerneas-*"
                        // JS regex: /(Xerneas)(-[a-zA-Z?-]+)?/g -> $1-*
                        if details.contains("Xerneas") {
                            if let Some(pos) = details.find("Xerneas") {
                                let before = &details[..pos];
                                let after = &details[pos + 7..]; // "Xerneas" is 7 chars

                                // Find where the forme name ends (at comma or end of string)
                                let after_cleaned = if after.starts_with('-') {
                                    // Has a forme suffix - find the next comma or use empty string
                                    if let Some(comma_pos) = after.find(',') {
                                        &after[comma_pos..]
                                    } else {
                                        ""
                                    }
                                } else {
                                    // No forme suffix, keep the rest as-is
                                    after
                                };

                                details = format!("{}Xerneas-*{}", before, after_cleaned);
                            }
                        }

                        (details, pokemon.side_index)
                    })
                    .collect();

                // Drop the immutable borrow
                drop(all_pokemon);

                // Now we can call methods that need mutable borrows
                for (details, side_index) in pokemon_data {
                    // JS: this.addSplit(pokemon.side.id, ['poke', pokemon.side.id, details, '']);
                    let side_id = if side_index == 0 { "p1" } else { "p2" };
                    self.add_split(side_id, &["poke", side_id, &details, ""], None);
                }

                // JS: this.makeRequest('teampreview');
                self.make_request(Some(BattleRequestState::TeamPreview));
                return;
            }
        }

        // JS: If we reach here, there's no team preview - don't call makeRequest
        // JavaScript's runPickTeam() returns without calling makeRequest if:
        // - requestState isn't already 'teampreview'
        // - ruleTable.pickedTeamSize doesn't exist
        // - format.onTeamPreview callback doesn't exist
        //
        // This allows turnLoop() to run immediately in start(), processing the 'start' action
        // which switches in initial Pokemon before team preview.
        //
        // For formats without team preview (like gen9randombattle in our test),
        // requestState remains None, turnLoop runs, and Pokemon are switched in during battle.start().
    }
}
