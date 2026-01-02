// JS Source:
//
// /**
//  * Unlike cureStatus, does not give cure message
//  */
// clearStatus() {
// 	if (!this.hp || !this.status) return false;
// 	if (this.status === 'slp' && this.removeVolatile('nightmare')) {
// 		this.battle.add('-end', this, 'Nightmare', '[silent]');
// 	}
// 	this.setStatus('');
// 	return true;
// }
//
// Note: In Rust, this is an associated function (not a method) because it needs
// mutable access to Battle while operating on a Pokemon within that Battle.

use crate::*;

impl Pokemon {
    /// Clear the pokemon's status
    /// Equivalent to pokemon.ts clearStatus()
    /// Unlike cureStatus, does not give cure message
    ///
    /// In Rust: Pokemon::clear_status(battle, pokemon_pos)
    /// In JS: pokemon.clearStatus()
    pub fn clear_status(battle: &mut Battle, pokemon_pos: (usize, usize)) -> bool {
        // JS: if (!this.hp || !this.status) return false;
        let (hp, status) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            (pokemon.hp, pokemon.status.clone())
        };

        if hp == 0 || status.is_empty() {
            return false;
        }

        // JS: if (this.status === 'slp' && this.removeVolatile('nightmare')) {
        //         this.battle.add('-end', this, 'Nightmare', '[silent]');
        //     }
        if status.as_str() == "slp" {
            let nightmare_id = ID::from("nightmare");
            let had_nightmare = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                pokemon.has_volatile(&nightmare_id)
            };

            if had_nightmare {
                Pokemon::remove_volatile(battle, pokemon_pos, &nightmare_id);
            }

            if had_nightmare {
                let side_id = format!("p{}", pokemon_pos.0 + 1);
                let fullname = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return false,
                    };
                    pokemon.fullname(&side_id)
                };
                battle.add(
                    "-end",
                    &[
                        fullname.as_str().into(),
                        "Nightmare".into(),
                        "[silent]".into(),
                    ],
                );
            }
        }

        // JS: this.setStatus('');
        Pokemon::set_status(battle, pokemon_pos, ID::empty(), None, None, false);

        // JS: return true;
        true
    }
}
