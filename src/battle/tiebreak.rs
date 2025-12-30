use crate::*;

impl Battle {

    /// Execute tiebreak logic
    /// Equivalent to battle.ts tiebreak()
    /// Tiebreaker logic for determining winner when time runs out
    /// Equivalent to battle.ts tiebreak() (battle.ts:1421-1462)
    ///
    //
    // 	tiebreak() {
    // 		if (this.ended) return false;
    //
    // 		this.inputLog.push(`>tiebreak`);
    // 		this.add('message', "Time's up! Going to tiebreaker...");
    // 		const notFainted = this.sides.map(side => (
    // 			side.pokemon.filter(pokemon => !pokemon.fainted).length
    // 		));
    // 		this.add('-message', this.sides.map((side, i) => (
    // 			`${side.name}: ${notFainted[i]} Pokemon left`
    // 		)).join('; '));
    // 		const maxNotFainted = Math.max(...notFainted);
    // 		let tiedSides = this.sides.filter((side, i) => notFainted[i] === maxNotFainted);
    // 		if (tiedSides.length <= 1) {
    // 			return this.win(tiedSides[0]);
    // 		}
    //
    // 		const hpPercentage = tiedSides.map(side => (
    // 			side.pokemon.map(pokemon => pokemon.hp / pokemon.maxhp).reduce((a, b) => a + b) * 100 / 6
    // 		));
    // 		this.add('-message', tiedSides.map((side, i) => (
    // 			`${side.name}: ${Math.round(hpPercentage[i])}% total HP left`
    // 		)).join('; '));
    // 		const maxPercentage = Math.max(...hpPercentage);
    // 		tiedSides = tiedSides.filter((side, i) => hpPercentage[i] === maxPercentage);
    // 		if (tiedSides.length <= 1) {
    // 			return this.win(tiedSides[0]);
    // 		}
    //
    // 		const hpTotal = tiedSides.map(side => (
    // 			side.pokemon.map(pokemon => pokemon.hp).reduce((a, b) => a + b)
    // 		));
    // 		this.add('-message', tiedSides.map((side, i) => (
    // 			`${side.name}: ${Math.round(hpTotal[i])} total HP left`
    // 		)).join('; '));
    // 		const maxTotal = Math.max(...hpTotal);
    // 		tiedSides = tiedSides.filter((side, i) => hpTotal[i] === maxTotal);
    // 		if (tiedSides.length <= 1) {
    // 			return this.win(tiedSides[0]);
    // 		}
    // 		return this.tie();
    // 	}
    //
    pub fn tiebreak(&mut self) -> Option<usize> {
        // JS: if (this.ended) return false;
        if self.ended {
            return None;
        }

        // Helper to convert usize to SideID
        let to_side_id = |idx: usize| -> SideID {
            match idx {
                0 => SideID::P1,
                1 => SideID::P2,
                2 => SideID::P3,
                _ => SideID::P4,
            }
        };

        // JS: this.add('message', "Time's up! Going to tiebreaker...");
        self.add("message", &[Arg::Str("Time's up! Going to tiebreaker...")]);

        // JS: const notFainted = this.sides.map(side => (
        //         side.pokemon.filter(pokemon => !pokemon.fainted).length
        //     ));
        let not_fainted: Vec<usize> = self
            .sides
            .iter()
            .map(|side| side.pokemon.iter().filter(|p| !p.fainted).count())
            .collect();

        // Log Pokemon counts
        let mut messages = Vec::new();
        for (i, side) in self.sides.iter().enumerate() {
            messages.push(format!("{}: {} Pokemon left", side.name, not_fainted[i]));
        }
        self.add("-message", &[Arg::Str(&messages.join("; "))]);

        // JS: const maxNotFainted = Math.max(...notFainted);
        // JS: let tiedSides = this.sides.filter((side, i) => notFainted[i] === maxNotFainted);
        let max_not_fainted = *not_fainted.iter().max().unwrap_or(&0);
        let mut tied_sides: Vec<usize> = (0..self.sides.len())
            .filter(|&i| not_fainted[i] == max_not_fainted)
            .collect();

        // JS: if (tiedSides.length <= 1) return this.win(tiedSides[0]);
        if tied_sides.len() == 1 {
            self.win(Some(to_side_id(tied_sides[0])));
            return Some(tied_sides[0]);
        }
        if tied_sides.is_empty() {
            self.tie();
            return None;
        }

        // JS: const hpPercentage = tiedSides.map(side => (
        //         side.pokemon.map(pokemon => pokemon.hp / pokemon.maxhp).reduce((a, b) => a + b) * 100 / 6
        //     ));
        let hp_percentage: Vec<f64> = tied_sides
            .iter()
            .map(|&side_idx| {
                let side = &self.sides[side_idx];
                let total: f64 = side
                    .pokemon
                    .iter()
                    .map(|p| {
                        if p.maxhp > 0 {
                            p.hp as f64 / p.maxhp as f64
                        } else {
                            0.0
                        }
                    })
                    .sum();
                total * 100.0 / 6.0
            })
            .collect();

        // Log HP percentages
        let mut messages = Vec::new();
        for (i, &side_idx) in tied_sides.iter().enumerate() {
            messages.push(format!(
                "{}: {}% total HP left",
                self.sides[side_idx].name,
                hp_percentage[i].round() as i32
            ));
        }
        self.add("-message", &[Arg::Str(&messages.join("; "))]);

        // JS: const maxPercentage = Math.max(...hpPercentage);
        // JS: tiedSides = tiedSides.filter((side, i) => hpPercentage[i] === maxPercentage);
        let max_percentage = hp_percentage.iter().cloned().fold(0.0_f64, f64::max);
        tied_sides = tied_sides
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (hp_percentage[*i] - max_percentage).abs() < 0.0001)
            .map(|(_, side_idx)| side_idx)
            .collect();

        // JS: if (tiedSides.length <= 1) return this.win(tiedSides[0]);
        if tied_sides.len() == 1 {
            self.win(Some(to_side_id(tied_sides[0])));
            return Some(tied_sides[0]);
        }
        if tied_sides.is_empty() {
            self.tie();
            return None;
        }

        // JS: const hpTotal = tiedSides.map(side => (
        //         side.pokemon.map(pokemon => pokemon.hp).reduce((a, b) => a + b)
        //     ));
        let hp_total: Vec<i32> = tied_sides
            .iter()
            .map(|&side_idx| self.sides[side_idx].pokemon.iter().map(|p| p.hp).sum())
            .collect();

        // Log HP totals
        let mut messages = Vec::new();
        for (i, &side_idx) in tied_sides.iter().enumerate() {
            messages.push(format!(
                "{}: {} total HP left",
                self.sides[side_idx].name, hp_total[i]
            ));
        }
        self.add("-message", &[Arg::Str(&messages.join("; "))]);

        // JS: const maxTotal = Math.max(...hpTotal);
        // JS: tiedSides = tiedSides.filter((side, i) => hpTotal[i] === maxTotal);
        let max_total = *hp_total.iter().max().unwrap_or(&0);
        tied_sides = tied_sides
            .into_iter()
            .enumerate()
            .filter(|(i, _)| hp_total[*i] == max_total)
            .map(|(_, side_idx)| side_idx)
            .collect();

        // JS: if (tiedSides.length <= 1) return this.win(tiedSides[0]);
        if tied_sides.len() == 1 {
            self.win(Some(to_side_id(tied_sides[0])));
            return Some(tied_sides[0]);
        }

        // JS: return this.tie();
        self.tie();
        None
    }
}
