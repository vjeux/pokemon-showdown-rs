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
    ///
    /// JavaScript:
    /// getRequestData(forAlly?: boolean): SideRequestData {
    ///     const data: SideRequestData = {
    ///         name: this.name,
    ///         id: this.id,
    ///         pokemon: [] as PokemonSwitchRequestData[],
    ///     };
    ///     for (const pokemon of this.pokemon) {
    ///         data.pokemon.push(pokemon.getSwitchRequestData(forAlly));
    ///     }
    ///     return data;
    /// }
    pub fn get_request_data(&self, for_ally: bool) -> serde_json::Value {
        // JS: const data: SideRequestData = {
        // JS:     name: this.name,
        // JS:     id: this.id,
        // JS:     pokemon: [] as PokemonSwitchRequestData[],
        // JS: };

        // JS: for (const pokemon of this.pokemon) {
        // JS:     data.pokemon.push(pokemon.getSwitchRequestData(forAlly));
        // JS: }
        let pokemon_data: Vec<serde_json::Value> = self.pokemon.iter()
            .map(|p| p.get_switch_request_data(for_ally))
            .collect();

        // JS: return data;
        serde_json::json!({
            "name": self.name,
            "id": self.id_str(),
            "pokemon": pokemon_data
        })
    }
}
