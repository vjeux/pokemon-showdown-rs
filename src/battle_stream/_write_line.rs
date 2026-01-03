// TODO: Implement _writeLine from JavaScript
//
// JS Source:
// 
// 	_writeLine(type: string, message: string) {
// 		switch (type) {
// 		case 'start':
// 			const options = JSON.parse(message);
// 			options.send = (t: string, data: any) => {
// 				if (Array.isArray(data)) data = data.join("\n");
// 				this.pushMessage(t, data);
// 				if (t === 'end' && !this.keepAlive) this.pushEnd();
// 			};
// 			if (this.debug) options.debug = true;
// 			this.battle = new Battle(options);
// 			break;
// 		case 'player':
// 			const [slot, optionsText] = splitFirst(message, ' ');
// 			this.battle!.setPlayer(slot as SideID, JSON.parse(optionsText));
// 			break;
// 		case 'p1':
// 		case 'p2':
// 		case 'p3':
// 		case 'p4':
// 			if (message === 'undo') {
// 				this.battle!.undoChoice(type);
// 			} else {
// 				this.battle!.choose(type, message);
// 			}
// 			break;
// 		case 'forcewin':
// 		case 'forcetie':
// 			this.battle!.win(type === 'forcewin' ? message as SideID : null);
// 			if (message) {
// 				this.battle!.inputLog.push(`>forcewin ${message}`);
// 			} else {
// 				this.battle!.inputLog.push(`>forcetie`);
// 			}
// 			break;
// 		case 'forcelose':
// 			this.battle!.lose(message as SideID);
// 			this.battle!.inputLog.push(`>forcelose ${message}`);
// 			break;
// 		case 'reseed':
// 			this.battle!.resetRNG(message as PRNGSeed);
// 			// could go inside resetRNG, but this makes using it in `eval` slightly less buggy
// 			this.battle!.inputLog.push(`>reseed ${this.battle!.prng.getSeed()}`);
// 			break;
// 		case 'tiebreak':
// 			this.battle!.tiebreak();
// 			break;
// 		case 'chat-inputlogonly':
// 			this.battle!.inputLog.push(`>chat ${message}`);
// 			break;
// 		case 'chat':
// 			this.battle!.inputLog.push(`>chat ${message}`);
// 			this.battle!.add('chat', `${message}`);
// 			break;
// 		case 'eval':
// 			const battle = this.battle!;
// 
// 			// n.b. this will usually but not always work - if you eval code that also affects the inputLog,
// 			// replaying the inputlog would double-play the change.
// 			battle.inputLog.push(`>${type} ${message}`);
// 
// 			message = message.replace(/\f/g, '\n');
// 			battle.add('', '>>> ' + message.replace(/\n/g, '\n||'));
// 			try {
// 				/* eslint-disable no-eval, @typescript-eslint/no-unused-vars */
// 				const p1 = battle.sides[0];
// 				const p2 = battle.sides[1];
// 				const p3 = battle.sides[2];
// 				const p4 = battle.sides[3];
// 				const p1active = p1?.active[0];
// 				const p2active = p2?.active[0];
// 				const p3active = p3?.active[0];
// 				const p4active = p4?.active[0];
// 				const toID = battle.toID;
// 				const player = (input: string) => {
// 					input = toID(input);
// 					if (/^p[1-9]$/.test(input)) return battle.sides[parseInt(input.slice(1)) - 1];
// 					if (/^[1-9]$/.test(input)) return battle.sides[parseInt(input) - 1];
// 					for (const side of battle.sides) {
// 						if (toID(side.name) === input) return side;
// 					}
// 					return null;
// 				};
// 				const pokemon = (side: string | Side, input: string) => {
// 					if (typeof side === 'string') side = player(side)!;
// 
// 					input = toID(input);
// 					if (/^[1-9]$/.test(input)) return side.pokemon[parseInt(input) - 1];
// 					return side.pokemon.find(p => p.baseSpecies.id === input || p.species.id === input);
// 				};
// 				let result = eval(message);
// 				/* eslint-enable no-eval, @typescript-eslint/no-unused-vars */
// 
// 				if (result?.then) {
// 					result.then((unwrappedResult: any) => {
// 						unwrappedResult = Utils.visualize(unwrappedResult);
// 						battle.add('', 'Promise -> ' + unwrappedResult);
// 						battle.sendUpdates();
// 					}, (error: Error) => {
// 						battle.add('', '<<< error: ' + error.message);
// 						battle.sendUpdates();
// 					});
// 				} else {
// 					result = Utils.visualize(result);
// 					result = result.replace(/\n/g, '\n||');
// 					battle.add('', '<<< ' + result);
// 				}
// 			} catch (e: any) {
// 				battle.add('', '<<< error: ' + e.message);
// 			}
// 			break;
// 		case 'requestlog':
// 			this.push(`requesteddata\n${this.battle!.inputLog.join('\n')}`);
// 			break;
// 		case 'requestexport':
// 			this.push(`requesteddata\n${this.battle!.prngSeed}\n${this.battle!.inputLog.join('\n')}`);
// 			break;
// 		case 'requestteam':
// 			message = message.trim();
// 			const slotNum = parseInt(message.slice(1)) - 1;
// 			if (isNaN(slotNum) || slotNum < 0) {
// 				throw new Error(`Team requested for slot ${message}, but that slot does not exist.`);
// 			}
// 			const side = this.battle!.sides[slotNum];
// 			const team = Teams.pack(side.team);
// 			this.push(`requesteddata\n${team}`);
// 			break;
// 		case 'show-openteamsheets':
// 			this.battle!.showOpenTeamSheets();
// 			break;
// 		case 'version':
// 		case 'version-origin':
// 			break;
// 		default:
// 			throw new Error(`Unrecognized command ">${type} ${message}"`);
// 		}
// 	}

use crate::*;
use crate::battle_stream::BattleStream;
use crate::battle::Battle;
use crate::dex_data::SideID;

impl BattleStream {
    /// Process a single protocol command line
    /// Equivalent to _writeLine() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   _writeLine(type: string, message: string) {
    ///     switch (type) { ... }
    ///   }
    pub fn _write_line(&mut self, cmd_type: &str, message: &str) -> Result<(), String> {
        match cmd_type {
            "start" => {
                // Parse battle options from JSON
                let options: serde_json::Value = serde_json::from_str(message)
                    .map_err(|e| format!("Failed to parse start options: {}", e))?;

                // Create new battle
                // JavaScript: this.battle = new Battle(options);
                // For now, create a default battle - full option parsing TODO
                let battle = Battle::new(
                    options.get("formatid").and_then(|v| v.as_str()).unwrap_or("gen9ou"),
                    options.get("seed").and_then(|v| v.as_array()).map(|arr| {
                        arr.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect()
                    })
                );

                self.battle = Some(battle);
                Ok(())
            }

            "player" => {
                // JavaScript: const [slot, optionsText] = splitFirst(message, ' ');
                // JavaScript: this.battle!.setPlayer(slot as SideID, JSON.parse(optionsText));
                let (slot_str, options_text) = crate::battle_stream::split_first(message, ' ');

                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                let slot = match slot_str {
                    "p1" => SideID::P1,
                    "p2" => SideID::P2,
                    "p3" => SideID::P3,
                    "p4" => SideID::P4,
                    _ => return Err(format!("Invalid slot: {}", slot_str)),
                };

                let player_options: serde_json::Value = serde_json::from_str(options_text)
                    .map_err(|e| format!("Failed to parse player options: {}", e))?;

                // Convert JSON to PlayerOptions
                let options = crate::battle::PlayerOptions {
                    name: player_options.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    avatar: player_options.get("avatar")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    team: if let Some(team_str) = player_options.get("team").and_then(|v| v.as_str()) {
                        crate::battle::TeamFormat::Packed(team_str.to_string())
                    } else {
                        crate::battle::TeamFormat::Empty
                    },
                    rating: player_options.get("rating")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                };

                battle.set_player(slot, options);
                Ok(())
            }

            "p1" | "p2" | "p3" | "p4" => {
                // JavaScript: if (message === 'undo') { ... } else { this.battle!.choose(type, message); }
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                if message == "undo" {
                    let slot = match cmd_type {
                        "p1" => SideID::P1,
                        "p2" => SideID::P2,
                        "p3" => SideID::P3,
                        "p4" => SideID::P4,
                        _ => unreachable!(),
                    };
                    battle.undo_choice(slot);
                } else {
                    let slot = match cmd_type {
                        "p1" => SideID::P1,
                        "p2" => SideID::P2,
                        "p3" => SideID::P3,
                        "p4" => SideID::P4,
                        _ => unreachable!(),
                    };
                    battle.choose(slot, message);
                }
                Ok(())
            }

            "forcewin" | "forcetie" => {
                // JavaScript: this.battle!.win(type === 'forcewin' ? message as SideID : null);
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                if cmd_type == "forcewin" {
                    let winner = if !message.is_empty() {
                        Some(match message {
                            "p1" => SideID::P1,
                            "p2" => SideID::P2,
                            "p3" => SideID::P3,
                            "p4" => SideID::P4,
                            _ => return Err(format!("Invalid winner: {}", message)),
                        })
                    } else {
                        None
                    };
                    battle.win(winner);
                    if !message.is_empty() {
                        battle.input_log.push(format!(">forcewin {}", message));
                    }
                } else {
                    battle.win(None);
                    battle.input_log.push(">forcetie".to_string());
                }
                Ok(())
            }

            "forcelose" => {
                // JavaScript: this.battle!.lose(message as SideID);
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                let loser = match message {
                    "p1" => SideID::P1,
                    "p2" => SideID::P2,
                    "p3" => SideID::P3,
                    "p4" => SideID::P4,
                    _ => return Err(format!("Invalid loser: {}", message)),
                };

                battle.lose(loser);
                battle.input_log.push(format!(">forcelose {}", message));
                Ok(())
            }

            "reseed" => {
                // JavaScript: this.battle!.resetRNG(message as PRNGSeed);
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                // Parse seed from message (format: "[u32, u32, u32, u32]")
                let seed: Vec<u32> = serde_json::from_str(message)
                    .map_err(|e| format!("Failed to parse seed: {}", e))?;

                battle.reset_rng(seed);
                battle.input_log.push(format!(">reseed {:?}", battle.prng.get_seed()));
                Ok(())
            }

            "tiebreak" => {
                // JavaScript: this.battle!.tiebreak();
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                battle.tiebreak();
                Ok(())
            }

            "chat-inputlogonly" => {
                // JavaScript: this.battle!.inputLog.push(`>chat ${message}`);
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                battle.input_log.push(format!(">chat {}", message));
                Ok(())
            }

            "chat" => {
                // JavaScript: this.battle!.inputLog.push(`>chat ${message}`);
                // JavaScript: this.battle!.add('chat', `${message}`);
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                battle.input_log.push(format!(">chat {}", message));
                battle.add("chat", &[crate::battle::Arg::Str(message)]);
                Ok(())
            }

            "eval" => {
                // JavaScript has full eval implementation with promise support
                // Rust cannot support eval. Log warning and skip.
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                battle.input_log.push(format!(">eval {}", message));
                battle.add("", &[crate::battle::Arg::Str(">>> eval not supported in Rust")]);
                battle.add("", &[crate::battle::Arg::Str("<<< error: eval not available")]);
                Ok(())
            }

            "requestlog" => {
                // JavaScript: this.push(`requesteddata\n${this.battle!.inputLog.join('\n')}`);
                let battle = self.battle.as_ref()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                let log_data = format!("requesteddata\n{}", battle.input_log.join("\n"));
                self.output_queue.push_back(log_data);
                Ok(())
            }

            "requestexport" => {
                // JavaScript: this.push(`requesteddata\n${this.battle!.prngSeed}\n${this.battle!.inputLog.join('\n')}`);
                let battle = self.battle.as_ref()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                let seed_str = format!("{:?}", battle.prng.get_seed());
                let export_data = format!("requesteddata\n{}\n{}", seed_str, battle.input_log.join("\n"));
                self.output_queue.push_back(export_data);
                Ok(())
            }

            "requestteam" => {
                // JavaScript: const side = this.battle!.sides[slotNum];
                // JavaScript: const team = Teams.pack(side.team);
                // JavaScript: this.push(`requesteddata\n${team}`);
                let battle = self.battle.as_ref()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                let message_trimmed = message.trim();
                let slot_num: usize = message_trimmed[1..]
                    .parse()
                    .map_err(|_| format!("Invalid slot number: {}", message_trimmed))?;

                if slot_num == 0 || slot_num > battle.sides.len() {
                    return Err(format!("Team requested for slot {}, but that slot does not exist.", message_trimmed));
                }

                let side = &battle.sides[slot_num - 1];
                let team = crate::teams::Teams::pack(&side.team);
                let team_data = format!("requesteddata\n{}", team);
                self.output_queue.push_back(team_data);
                Ok(())
            }

            "show-openteamsheets" => {
                // JavaScript: this.battle!.showOpenTeamSheets();
                let battle = self.battle.as_mut()
                    .ok_or_else(|| "No battle initialized".to_string())?;

                battle.show_open_team_sheets();
                Ok(())
            }

            "version" | "version-origin" => {
                // JavaScript: break; (no-op)
                Ok(())
            }

            _ => {
                Err(format!("Unrecognized command \">{} {}\"", cmd_type, message))
            }
        }
    }
}
