use crate::side::*;
use crate::event::EventResult;

impl Side {

    /// Get the choice as a string
    // TypeScript source:
    // /** convert a Choice into a choice string */
    // 	getChoice() {
    // 		if (this.choice.actions.length > 1 && this.choice.actions.every(action => action.choice === 'team')) {
    // 			return `team ` + this.choice.actions.map(action => action.pokemon!.position + 1).join(', ');
    // 		}
    // 		return this.choice.actions.map(action => {
    // 			switch (action.choice) {
    // 			case 'move':
    // 				let details = ``;
    // 				if (action.targetLoc && this.active.length > 1) details += ` ${action.targetLoc > 0 ? '+' : ''}${action.targetLoc}`;
    // 				if (action.mega) details += (action.pokemon!.item === 'ultranecroziumz' ? ` ultra` : ` mega`);
    // 				if (action.megax) details += ` megax`;
    // 				if (action.megay) details += ` megay`;
    // 				if (action.zmove) details += ` zmove`;
    // 				if (action.maxMove) details += ` dynamax`;
    // 				if (action.terastallize) details += ` terastallize`;
    // 				return `move ${action.moveid}${details}`;
    // 			case 'switch':
    // 			case 'instaswitch':
    // 			case 'revivalblessing':
    // 				return `switch ${action.target!.position + 1}`;
    // 			case 'team':
    // 				return `team ${action.pokemon!.position + 1}`;
    // 			default:
    // 				return action.choice;
    // 			}
    // 		}).join(', ');
    // 	}
    //
    pub fn get_choice(&self) -> String {
        self.choice
            .actions
            .iter()
            .map(|action| match action.choice {
                ChoiceType::Move => {
                    let mut s = format!(
                        "move {}",
                        action.move_id.as_ref().map(|id| id.as_str()).unwrap_or("")
                    );
                    if let Some(target) = action.target_loc {
                        s.push_str(&format!(" {}", target));
                    }
                    if action.mega {
                        s.push_str(" mega");
                    }
                    if let Some(ref z) = action.zmove {
                        s.push_str(&format!(" zmove {}", z));
                    }
                    s
                }
                ChoiceType::Switch => {
                    format!("switch {}", action.switch_index.unwrap_or(0) + 1)
                }
                ChoiceType::Team => {
                    format!("team {}", action.pokemon_index + 1)
                }
                ChoiceType::Pass => "pass".to_string(),
                _ => format!("{:?}", action.choice),
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
