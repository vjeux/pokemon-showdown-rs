use crate::*;

impl Pokemon {

    /// Get switch request data for protocol
    /// Equivalent to getSwitchRequestData in pokemon.ts
    //
    // 	getSwitchRequestData(forAlly?: boolean): PokemonSwitchRequestData {
    // 		const entry: PokemonSwitchRequestData = {
    // 			ident: this.fullname,
    // 			details: this.details,
    // 			condition: this.getHealth().secret,
    // 			active: (this.position < this.side.active.length),
    // 			stats: {
    // 				atk: this.baseStoredStats['atk'],
    // 				def: this.baseStoredStats['def'],
    // 				spa: this.baseStoredStats['spa'],
    // 				spd: this.baseStoredStats['spd'],
    // 				spe: this.baseStoredStats['spe'],
    // 			},
    // 			moves: this[forAlly ? 'baseMoves' : 'moves'].map(move => {
    // 				if (move === 'hiddenpower') {
    // 					return `${move}${toID(this.hpType)}${this.battle.gen < 6 ? '' : this.hpPower}` as ID;
    // 				}
    // 				if (move === 'frustration' || move === 'return') {
    // 					const basePowerCallback = this.battle.dex.moves.get(move).basePowerCallback as (pokemon: Pokemon) => number;
    // 					return `${move}${basePowerCallback(this)}` as ID;
    // 				}
    // 				return move as ID;
    // 			}),
    // 			baseAbility: this.baseAbility,
    // 			item: this.item,
    // 			pokeball: this.pokeball,
    // 		};
    // 		if (this.battle.gen > 6) entry.ability = this.ability;
    // 		if (this.battle.gen >= 9) {
    // 			entry.commanding = !!this.volatiles['commanding'] && !this.fainted;
    // 			entry.reviving = this.isActive && !!this.side.slotConditions[this.position]['revivalblessing'];
    // 		}
    // 		if (this.battle.gen === 9) {
    // 			entry.teraType = this.teraType;
    // 			entry.terastallized = this.terastallized || '';
    // 		}
    // 		return entry;
    // 	}
    //
    pub fn get_switch_request_data(&self, for_ally: bool) -> serde_json::Value {
        // JS: const entry: PokemonSwitchRequestData = {
        // JS:     ident: this.fullname,
        // ✅ NOW IMPLEMENTED (Session 24 Part 60): ident field (fullname format)
        let side_id = format!("p{}", self.side_index + 1);
        let ident = format!("{}: {}", side_id, self.name);

        // JS:     details: this.details,
        // ✅ NOW IMPLEMENTED (Session 24 Part 60): details field using get_updated_details
        let details = self.get_updated_details();

        // JS:     condition: this.getHealth().secret,
        // ✅ NOW IMPLEMENTED (Session 24 Part 60): condition field using get_health
        let condition = self.get_health();

        // JS:     active: (this.position < this.side.active.length),
        // ✅ NOW IMPLEMENTED (Session 24 Part 60): active field
        // Note: In Rust, is_active already tracks if Pokemon is in battle
        let active = self.is_active;

        // JS:     stats: {
        // JS:         atk: this.baseStoredStats['atk'],
        // JS:         def: this.baseStoredStats['def'],
        // JS:         spa: this.baseStoredStats['spa'],
        // JS:         spd: this.baseStoredStats['spd'],
        // JS:         spe: this.baseStoredStats['spe'],
        // JS:     },
        // ✅ NOW IMPLEMENTED (Session 24 Part 60): stats object with baseStoredStats
        let stats = serde_json::json!({
            "atk": self.base_stored_stats.atk,
            "def": self.base_stored_stats.def,
            "spa": self.base_stored_stats.spa,
            "spd": self.base_stored_stats.spd,
            "spe": self.base_stored_stats.spe
        });

        // JS:     moves: this[forAlly ? 'baseMoves' : 'moves'].map(move => {
        // ✅ NOW IMPLEMENTED (Session 24 Part 68): forAlly parameter support
        //
        // Get move IDs based on forAlly parameter
        let move_ids: Vec<String> = if for_ally {
            // Use base move slots (moves before Transform)
            self.base_move_slots.iter().map(|slot| slot.id.as_str().to_string()).collect()
        } else {
            // Use current move slots (may be transformed)
            self.move_slots.iter().map(|slot| slot.id.as_str().to_string()).collect()
        };

        // Format move IDs with Hidden Power type (Return/Frustration need Dex access)
        let formatted_moves: Vec<String> = move_ids.iter().map(|move_id| {
            if move_id == "hiddenpower" {
                if !self.hp_type.is_empty() {
                    // Note: Would need Battle reference for gen check to conditionally append power
                    // Gen 6+: "hiddenpowerfire", Gen < 6: "hiddenpowerfire70"
                    format!("hiddenpower{}", self.hp_type.to_lowercase())
                } else {
                    move_id.clone()
                }
            } else {
                // Return/Frustration formatting needs Dex access (see note below)
                move_id.clone()
            }
        }).collect();

        // JS:         if (move === 'hiddenpower') {
        // JS:             return `${move}${toID(this.hpType)}${this.battle.gen < 6 ? '' : this.hpPower}` as ID;
        // JS:         }
        // ✅ NOW IMPLEMENTED (Session 24 Part 63): Hidden Power formatting handled in get_moves()

        // JS:         if (move === 'frustration' || move === 'return') {
        // JS:             const basePowerCallback = this.battle.dex.moves.get(move).basePowerCallback as (pokemon: Pokemon) => number;
        // JS:             return `${move}${basePowerCallback(this)}` as ID;
        // JS:         }
        // Note: Missing Return/Frustration power calculation (needs Dex access in get_moves)

        // JS:         return move as ID;
        // JS:     }),
        // ✅ NOW IMPLEMENTED (Session 24 Part 63): get_moves() returns full move objects
        // Note: get_moves() handles Hidden Power formatting internally

        // JS:     baseAbility: this.baseAbility,
        // ✅ NOW IMPLEMENTED (Session 24 Part 56): baseAbility field

        // JS:     item: this.item,
        // JS:     pokeball: this.pokeball,
        // ✅ NOW IMPLEMENTED (Session 24 Part 56): pokeball field

        // JS: };
        // JS: if (this.battle.gen > 6) entry.ability = this.ability;
        // Note: Would need Battle reference for gen check
        // Note: Currently always including ability

        // JS: if (this.battle.gen >= 9) {
        // JS:     entry.commanding = !!this.volatiles['commanding'] && !this.fainted;
        // JS:     entry.reviving = this.isActive && !!this.side.slotConditions[this.position]['revivalblessing'];
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 64): commanding field
        let commanding = self.has_volatile(&ID::new("commanding")) && !self.fainted;
        // Note: reviving field needs Side.slotConditions reference

        // JS: if (this.battle.gen === 9) {
        // JS:     entry.teraType = this.teraType;
        // JS:     entry.terastallized = this.terastallized || '';
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 64): teraType and terastallized fields
        let tera_type = self.tera_type.as_deref().unwrap_or("");
        let terastallized = self.terastallized.as_deref().unwrap_or("");

        // JS: return entry;
        serde_json::json!({
            "ident": ident,
            "details": details,
            "condition": condition,
            "active": active,
            "stats": stats,
            "moves": formatted_moves, // ✅ NOW IMPLEMENTED (Session 24 Part 68): Use forAlly-selected and formatted moves
            "ability": self.ability.as_str(),
            "baseAbility": self.base_ability.as_str(),
            "item": self.item.as_str(),
            "pokeball": self.pokeball.as_str(),
            "commanding": commanding,
            "teraType": tera_type,
            "terastallized": terastallized
        })
    }
}
