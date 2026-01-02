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
        // TODO: implement the same logic as JavaScript
        
        serde_json::json!({
            "name": self.name,
            "species": self.species_id.as_str(),
            "level": self.level,
            "hp": self.hp,
            "maxhp": self.maxhp,
            "status": if self.status.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(self.status.as_str().to_string()) },
            "moves": self.get_moves(),
            "ability": self.ability.as_str(),
            "item": self.item.as_str()
        })
    }
}
