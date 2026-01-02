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
    pub fn get_switch_request_data(&self) -> serde_json::Value {
        // JS: const entry: PokemonSwitchRequestData = {
        // JS:     ident: this.fullname,
        // Note: Missing ident field (should be fullname like "p1a: Pikachu")
        // Note: Currently using name instead

        // JS:     details: this.details,
        // Note: Missing details field (should be species details with forme/shiny)
        // Note: Currently using species_id

        // JS:     condition: this.getHealth().secret,
        // Note: Missing condition field (should be "hp/maxhp status" format)
        // Note: Currently using separate hp/maxhp/status fields

        // JS:     active: (this.position < this.side.active.length),
        // Note: Missing active field (whether currently in battle)

        // JS:     stats: {
        // JS:         atk: this.baseStoredStats['atk'],
        // JS:         def: this.baseStoredStats['def'],
        // JS:         spa: this.baseStoredStats['spa'],
        // JS:         spd: this.baseStoredStats['spd'],
        // JS:         spe: this.baseStoredStats['spe'],
        // JS:     },
        // Note: Missing stats object with baseStoredStats

        // JS:     moves: this[forAlly ? 'baseMoves' : 'moves'].map(move => {
        // Note: Missing forAlly parameter to choose baseMoves vs moves

        // JS:         if (move === 'hiddenpower') {
        // JS:             return `${move}${toID(this.hpType)}${this.battle.gen < 6 ? '' : this.hpPower}` as ID;
        // JS:         }
        // Note: Missing Hidden Power formatting with type and power

        // JS:         if (move === 'frustration' || move === 'return') {
        // JS:             const basePowerCallback = this.battle.dex.moves.get(move).basePowerCallback as (pokemon: Pokemon) => number;
        // JS:             return `${move}${basePowerCallback(this)}` as ID;
        // JS:         }
        // Note: Missing Return/Frustration power calculation

        // JS:         return move as ID;
        // JS:     }),
        // Note: Currently calling get_moves() which returns Vec<String>

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
        // Note: Missing Gen 9+ commanding and reviving fields

        // JS: if (this.battle.gen === 9) {
        // JS:     entry.teraType = this.teraType;
        // JS:     entry.terastallized = this.terastallized || '';
        // JS: }
        // Note: Missing Gen 9 teraType and terastallized fields

        // JS: return entry;
        serde_json::json!({
            "name": self.name,
            "species": self.species_id.as_str(),
            "level": self.level,
            "hp": self.hp,
            "maxhp": self.maxhp,
            "status": if self.status.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(self.status.as_str().to_string()) },
            "moves": self.get_moves(),
            "ability": self.ability.as_str(),
            "baseAbility": self.base_ability.as_str(),
            "item": self.item.as_str(),
            "pokeball": self.pokeball.as_str()
        })
    }
}
