use crate::*;
use crate::side::Side;

impl Battle {

    /// Add or update a player in the battle
    ///
    // TypeScript source:
    //
    //
    // 	setPlayer(slot: SideID, options: PlayerOptions) {
    // 		let side;
    // 		let didSomething = true;
    // 		const slotNum = parseInt(slot[1]) - 1;
    // 		if (!this.sides[slotNum]) {
    // 			// create player
    // 			const team = this.getTeam(options);
    // 			side = new Side(options.name || `Player ${slotNum + 1}`, this, slotNum, team);
    // 			if (options.avatar) side.avatar = `${options.avatar}`;
    // 			this.sides[slotNum] = side;
    // 		} else {
    // 			// edit player
    // 			side = this.sides[slotNum];
    // 			didSomething = false;
    // 			if (options.name && side.name !== options.name) {
    // 				side.name = options.name;
    // 				didSomething = true;
    // 			}
    // 			if (options.avatar && side.avatar !== `${options.avatar}`) {
    // 				side.avatar = `${options.avatar}`;
    // 				didSomething = true;
    // 			}
    // 			if (options.team) throw new Error(`Player ${slot} already has a team!`);
    // 		}
    // 		if (options.team && typeof options.team !== 'string') {
    // 			options.team = Teams.pack(options.team);
    // 		}
    // 		if (!didSomething) return;
    // 		this.inputLog.push(`>player ${slot} ` + JSON.stringify(options));
    // 		this.add('player', side.id, side.name, side.avatar, options.rating || '');
    //
    // 		// Start the battle if it's ready to start
    // 		if (this.sides.every(playerSide => !!playerSide) && !this.started) this.start();
    // 	}
    //
    pub fn set_player(&mut self, side_id: SideID, options: PlayerOptions) {
        let slot_num = side_id.index();
        let mut did_something = true;

        // Ensure sides vector is large enough
        while self.sides.len() <= slot_num {
            self.sides.push(Side::new(
                match self.sides.len() {
                    0 => SideID::P1,
                    1 => SideID::P2,
                    2 => SideID::P3,
                    _ => SideID::P4,
                },
                self.sides.len(),
                String::new(),
                Vec::new(),
                self.active_per_half,
            ));
        }

        // Check if this is a new player or editing existing
        let is_new =
            self.sides[slot_num].name.is_empty() && self.sides[slot_num].pokemon.is_empty();

        if is_new {
            // Create player
            let team = options.team.clone(); // For now, assume team is already unpacked
            let name = if !options.name.is_empty() {
                options.name.clone()
            } else {
                format!("Player {}", slot_num + 1)
            };

            let mut side = Side::new(side_id, slot_num, name, team, self.active_per_half);
            if let Some(avatar) = &options.avatar {
                side.avatar = avatar.clone();
            }
            self.sides[slot_num] = side;

            // Initialize Pokemon stats after creating the side
            self.init_pokemon_stats(slot_num);
        } else {
            // Edit player
            did_something = false;

            if !options.name.is_empty() && self.sides[slot_num].name != options.name {
                self.sides[slot_num].name = options.name.clone();
                did_something = true;
            }

            if let Some(avatar) = &options.avatar {
                if self.sides[slot_num].avatar != *avatar {
                    self.sides[slot_num].avatar = avatar.clone();
                    did_something = true;
                }
            }

            if !options.team.is_empty() {
                panic!("Player {} already has a team!", side_id.to_str());
            }
        }

        if !did_something {
            return;
        }

        // Log to inputLog
        // Format options as JSON - simplified version for now
        let mut options_json = format!("{{\"name\":\"{}\"", options.name);
        if let Some(avatar) = &options.avatar {
            options_json.push_str(&format!(",\"avatar\":\"{}\"", avatar));
        }
        if let Some(rating) = &options.rating {
            options_json.push_str(&format!(",\"rating\":\"{}\"", rating));
        }
        options_json.push('}');

        self.input_log
            .push(format!(">player {} {}", side_id.to_str(), options_json));

        // this.add('player', side.id, side.name, side.avatar, options.rating || '');
        // Clone the values to avoid borrow checker issues
        let side_id_str = self.sides[slot_num].id_str().to_string();
        let side_name = self.sides[slot_num].name.clone();
        let side_avatar = self.sides[slot_num].avatar.clone();
        let rating_str = options.rating.as_deref().unwrap_or("").to_string();

        self.add(
            "player",
            &[
                Arg::Str(&side_id_str),
                Arg::Str(&side_name),
                Arg::Str(&side_avatar),
                Arg::Str(&rating_str),
            ],
        );

        // Start the battle if it's ready to start
        // if (this.sides.every(playerSide => !!playerSide) && !this.started)
        // Check that we have the expected number of sides AND all are ready
        // For standard battles, we expect 2 sides (p1 and p2)
        let expected_sides = match self.game_type {
            GameType::Multi | GameType::FreeForAll => 4,
            _ => 2,
        };
        let all_sides_ready = self.sides.len() == expected_sides
            && self
                .sides
                .iter()
                .all(|s| !s.name.is_empty() && !s.pokemon.is_empty());

        if all_sides_ready && !self.started {
            self.start();
        }
    }
}
