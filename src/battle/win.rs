use crate::*;
use crate::battle::BattleRequestState;
use crate::side::RequestState;

impl Battle {

    /// Declare a winner
    /// Equivalent to battle.ts win() (battle.ts:1474-1497)
    /// Pass None for a tie, or Some(side_id) for a winner
    //
    // 	win(side?: SideID | '' | Side | null) {
    // 		if (this.ended) return false;
    // 		if (side && typeof side === 'string') {
    // 			side = this.getSide(side);
    // 		} else if (!side || !this.sides.includes(side)) {
    // 			side = null;
    // 		}
    // 		this.winner = side ? side.name : '';
    //
    // 		this.add('');
    // 		if (side?.allySide) {
    // 			this.add('win', side.name + ' & ' + side.allySide.name);
    // 		} else if (side) {
    // 			this.add('win', side.name);
    // 		} else {
    // 			this.add('tie');
    // 		}
    // 		this.ended = true;
    // 		this.requestState = '';
    // 		for (const s of this.sides) {
    // 			if (s) s.activeRequest = null;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn win(&mut self, side: Option<SideID>) -> bool {
        // JavaScript: if (this.ended) return false;
        if self.ended {
            return false;
        }

        // JavaScript: if (side && typeof side === 'string') side = this.getSide(side);
        // (Rust uses SideID enum, not string)

        // JavaScript: else if (!side || !this.sides.includes(side)) side = null;
        // (Rust Option<SideID> handles this)

        // JavaScript: this.winner = side ? side.name : '';
        let (winner_name, ally_name) = if let Some(side_id) = side {
            if let Some(side) = self.get_side(side_id) {
                let ally = side
                    .ally_index
                    .and_then(|idx| self.sides.get(idx))
                    .map(|s| s.name.clone());
                (Some(side.name.clone()), ally)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        self.winner = winner_name.clone();

        // JavaScript: this.add('');
        self.add_log("", &[]);

        // JavaScript: if (side?.allySide) { this.add('win', side.name + ' & ' + side.allySide.name); }
        if let (Some(ref name), Some(ref ally)) = (&winner_name, &ally_name) {
            let combined = format!("{} & {}", name, ally);
            self.add_log("win", &[&combined]);
        } else if let Some(ref name) = winner_name {
            // JavaScript: else if (side) { this.add('win', side.name); }
            self.add_log("win", &[name]);
        } else {
            // JavaScript: else { this.add('tie'); }
            self.add_log("tie", &[]);
        }

        // JavaScript: this.ended = true;
        self.ended = true;

        // JavaScript: this.requestState = '';
        self.request_state = BattleRequestState::None;

        // JavaScript: for (const s of this.sides) { if (s) s.activeRequest = null; }
        for side in &mut self.sides {
            side.request_state = RequestState::None;
        }

        // JavaScript: return true;
        true
    }
}
