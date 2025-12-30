use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Process a single command line
    /// Equivalent to _writeLine() in battle-stream.ts
    pub fn write_line(&mut self, cmd: &str, args: &str) {
        match cmd {
            "start" => {
                // Parse options from JSON and start battle
                // Note: In full implementation, would parse JSON into BattleOptions
                // For now, create default options
                let _ = args; // Would parse args as JSON
                              // Would call: self.start(options);
            }
            "player" => {
                // Set player info
                let parts: Vec<&str> = args.splitn(2, ' ').collect();
                let _slot = parts.first().copied().unwrap_or("");
                let _options_text = parts.get(1).copied().unwrap_or("");
                // Would call battle.set_player(slot, options)
            }
            "p1" | "p2" | "p3" | "p4" => {
                // Player choice
                if args == "undo" {
                    // Undo choice
                    // Would call battle.undo_choice(cmd)
                } else if let Some(ref mut battle) = self.battle {
                    let (p1_choice, p2_choice) = match cmd {
                        "p1" => (args, ""),
                        "p2" => ("", args),
                        _ => ("", ""),
                    };
                    battle.make_choices(p1_choice, p2_choice);
                }
            }
            "forcewin" => {
                if let Some(ref mut battle) = self.battle {
                    battle.ended = true;
                    battle.winner = Some(args.to_string());
                    battle.input_log.push(format!(">forcewin {}", args));
                }
            }
            "forcetie" => {
                if let Some(ref mut battle) = self.battle {
                    battle.ended = true;
                    battle.winner = None;
                    battle.input_log.push(">forcetie".to_string());
                }
            }
            "forcelose" => {
                if let Some(ref mut battle) = self.battle {
                    // Mark the specified side as lost
                    battle.input_log.push(format!(">forcelose {}", args));
                    // Would determine winner based on who didn't lose
                }
            }
            "reseed" => {
                if let Some(ref mut battle) = self.battle {
                    // Reset RNG with new seed
                    battle.input_log.push(format!(">reseed {}", args));
                    // Would call battle.reset_rng(args)
                }
            }
            "tiebreak" => {
                if let Some(ref mut _battle) = self.battle {
                    // Tiebreak logic
                    // Would call battle.tiebreak()
                }
            }
            "chat" => {
                if let Some(ref mut battle) = self.battle {
                    battle.input_log.push(format!(">chat {}", args));
                    // Add chat message to battle log
                }
            }
            "chat-inputlogonly" => {
                if let Some(ref mut battle) = self.battle {
                    battle.input_log.push(format!(">chat {}", args));
                }
            }
            "eval" => {
                // Eval is not supported in Rust for security reasons
                if let Some(ref mut battle) = self.battle {
                    battle.input_log.push(format!(">eval {}", args));
                }
            }
            "requestlog" => {
                if let Some(ref battle) = self.battle {
                    let log = battle.input_log.join("\n");
                    self.output_queue
                        .push_back(format!("requesteddata\n{}", log));
                }
            }
            "requestexport" => {
                if let Some(ref battle) = self.battle {
                    // Would include PRNG seed and input log
                    let log = battle.input_log.join("\n");
                    self.output_queue
                        .push_back(format!("requesteddata\n{}", log));
                }
            }
            "requestteam" => {
                if let Some(ref battle) = self.battle {
                    let slot = args.trim();
                    let side_idx = match slot {
                        "p1" => 0,
                        "p2" => 1,
                        "p3" => 2,
                        "p4" => 3,
                        _ => 0,
                    };
                    if side_idx < battle.sides.len() {
                        // Would pack team
                        self.output_queue.push_back("requesteddata\n".to_string());
                    }
                }
            }
            "show-openteamsheets" => {
                // Show open team sheets
                // Would call battle.show_open_team_sheets()
            }
            "version" | "version-origin" => {
                // Version info - ignored
            }
            _ => {
                // Unknown command
            }
        }
    }
}
