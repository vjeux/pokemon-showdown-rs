// JS Source:
// 
// 	getSlot(): PokemonSlot {
// 		const positionOffset = Math.floor(this.side.n / 2) * this.side.active.length;
// 		const positionLetter = 'abcdef'.charAt(this.position + positionOffset);
// 		return (this.side.id + positionLetter) as PokemonSlot;
// 	}


use crate::*;

impl Pokemon {

    /// Get the slot ID for protocol messages (e.g., "p1a", "p2b")
    //
    // 	getSlot(): PokemonSlot {
    // 		const positionOffset = Math.floor(this.side.n / 2) * this.side.active.length;
    // 		const positionLetter = 'abcdef'.charAt(this.position + positionOffset);
    // 		return (this.side.id + positionLetter) as PokemonSlot;
    // 	}
    //
    pub fn get_slot(&self) -> String {
        let position_letter = match self.position {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            _ => 'a',
        };
        format!("p{}{}", self.side_index + 1, position_letter)
    }
}
