use crate::*;

impl Battle {

    /// Send updates to connected players
    /// Equivalent to battle.ts sendUpdates() (battle.ts:3266-3296, 31 lines)
    ///
    // 	sendUpdates() {
    // 		if (this.sentLogPos >= this.log.length) return;
    // 		this.send('update', this.log.slice(this.sentLogPos));
    // 		if (!this.sentRequests) {
    // 			for (const side of this.sides) side.emitRequest();
    // 			this.sentRequests = true;
    // 		}
    // 		this.sentLogPos = this.log.length;
    //
    // 		if (!this.sentEnd && this.ended) {
    // 			const log = {
    // 				winner: this.winner,
    // 				seed: this.prngSeed,
    // 				turns: this.turn,
    // 				p1: this.sides[0].name,
    // 				p2: this.sides[1].name,
    // 				p3: this.sides[2]?.name,
    // 				p4: this.sides[3]?.name,
    // 				p1team: this.sides[0].team,
    // 				p2team: this.sides[1].team,
    // 				p3team: this.sides[2]?.team,
    // 				p4team: this.sides[3]?.team,
    // 				score: [this.sides[0].pokemonLeft, this.sides[1].pokemonLeft],
    // 				inputLog: this.inputLog,
    // 			};
    // 			if (this.sides[2]) {
    // 				log.score.push(this.sides[2].pokemonLeft);
    // 			} else {
    // 				delete log.p3;
    // 				delete log.p3team;
    // 			}
    // 			if (this.sides[3]) {
    // 				log.score.push(this.sides[3].pokemonLeft);
    // 			} else {
    // 				delete log.p4;
    // 				delete log.p4team;
    // 			}
    // 			this.send('end', JSON.stringify(log));
    // 			this.sentEnd = true;
    // 		}
    // 	}
    //
    pub fn send_updates(&mut self) {
        // JS: if (this.sentLogPos >= this.log.length) return;
        if self.sent_log_pos >= self.log.len() {
            return;
        }

        // JS: this.send('update', this.log.slice(this.sentLogPos));
        if let Some(ref send) = self.send {
            let log_slice = self.log[self.sent_log_pos..].join("\n");
            send("update", &log_slice);
        }

        // JS: if (!this.sentRequests) {
        //         for (const side of this.sides) side.emitRequest();
        //         this.sentRequests = true;
        //     }
        if !self.sent_requests {
            // Get requests for all sides
            let requests: Vec<serde_json::Value> = self.sides.iter()
                .map(|side| side.get_request_data())
                .collect();

            // Emit requests
            for (i, request) in requests.iter().enumerate() {
                if let Some(side) = self.sides.get(i) {
                    side.emit_request(request);
                }
            }
            self.sent_requests = true;
        }

        // JS: this.sentLogPos = this.log.length;
        self.sent_log_pos = self.log.len();

        // JS: if (!this.sentEnd && this.ended) { ... }
        if !self.sent_end && self.ended {
            if let Some(ref send) = self.send {
                // Build end log JSON
                let mut log_obj = serde_json::json!({
                    "winner": self.winner,
                    "seed": self.prng_seed.to_string(),
                    "turns": self.turn,
                    "inputLog": self.input_log,
                });

                // Add player names and teams
                let log_map = log_obj.as_object_mut().unwrap();

                // p1, p2 (always present)
                if let Some(side) = self.sides.get(0) {
                    log_map.insert("p1".to_string(), serde_json::json!(side.name));
                    log_map.insert("p1team".to_string(), serde_json::json!(side.team));
                }
                if let Some(side) = self.sides.get(1) {
                    log_map.insert("p2".to_string(), serde_json::json!(side.name));
                    log_map.insert("p2team".to_string(), serde_json::json!(side.team));
                }

                // Score array
                let mut score = Vec::new();
                if let Some(side) = self.sides.get(0) {
                    score.push(side.pokemon_left);
                }
                if let Some(side) = self.sides.get(1) {
                    score.push(side.pokemon_left);
                }

                // p3 (optional)
                if let Some(side) = self.sides.get(2) {
                    log_map.insert("p3".to_string(), serde_json::json!(side.name));
                    log_map.insert("p3team".to_string(), serde_json::json!(side.team));
                    score.push(side.pokemon_left);
                }

                // p4 (optional)
                if let Some(side) = self.sides.get(3) {
                    log_map.insert("p4".to_string(), serde_json::json!(side.name));
                    log_map.insert("p4team".to_string(), serde_json::json!(side.team));
                    score.push(side.pokemon_left);
                }

                log_map.insert("score".to_string(), serde_json::json!(score));

                // JS: this.send('end', JSON.stringify(log));
                let log_json = serde_json::to_string(&log_obj).unwrap_or_default();
                send("end", &log_json);

                // JS: this.sentEnd = true;
                self.sent_end = true;
            }
        }
    }
}
