// JS Source:
// 
// 	getRequestData(forAlly?: boolean): SideRequestData {
// 		const data: SideRequestData = {
// 			name: this.name,
// 			id: this.id,
// 			pokemon: [] as PokemonSwitchRequestData[],
// 		};
// 		for (const pokemon of this.pokemon) {
// 			data.pokemon.push(pokemon.getSwitchRequestData(forAlly));
// 		}
// 		return data;
// 	}


use crate::side::*;

impl Side {

    /// Get request data for protocol
    /// Equivalent to side.ts getRequestData()
    //
    // 	getRequestData(forAlly?: boolean): SideRequestData {
    // 		const data: SideRequestData = {
    // 			name: this.name,
    // 			id: this.id,
    // 			pokemon: [] as PokemonSwitchRequestData[],
    // 		};
    // 		for (const pokemon of this.pokemon) {
    // 			data.pokemon.push(pokemon.getSwitchRequestData(forAlly));
    // 		}
    // 		return data;
    // 	}
    //
    pub fn get_request_data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "id": self.id_str(),
            "pokemon": self.pokemon.iter().enumerate().map(|(i, p)| {
                serde_json::json!({
                    "ident": format!("{}: {}", self.id_str(), p.name),
                    "details": format!("{}, L{}", p.species_id.as_str(), p.level),
                    "condition": format!("{}/{}", p.hp, p.maxhp),
                    "active": self.active.contains(&Some(i)),
                    "moves": p.move_slots.iter().map(|m| m.id.as_str().to_string()).collect::<Vec<_>>(),
                    "ability": p.ability.as_str(),
                    "item": p.item.as_str()
                })
            }).collect::<Vec<_>>()
        })
    }
}
