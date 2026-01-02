// JS Source:
// 
// 	debug(action?: any): string {
// 		if (action) {
// 			return `${action.order || ''}:${action.priority || ''}:${action.speed || ''}:${action.subOrder || ''} - ${action.choice}${action.pokemon ? ' ' + action.pokemon : ''}${action.move ? ' ' + action.move : ''}`;
// 		}
// 		return this.list.map(
// 			queueAction => this.debug(queueAction)
// 		).join('\n') + '\n';
// 	}


use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Debug output for queue state
    //
    // 	debug(action?: any): string {
    // 		if (action) {
    // 			return `${action.order || ''}:${action.priority || ''}:${action.speed || ''}:${action.subOrder || ''} - ${action.choice}${action.pokemon ? ' ' + action.pokemon : ''}${action.move ? ' ' + action.move : ''}`;
    // 		}
    // 		return this.list.map(
    // 			queueAction => this.debug(queueAction)
    // 		).join('\n') + '\n';
    // 	}
    //
    pub fn debug(&self) -> String {
        self.list
            .iter()
            .map(|action| match action {
                Action::Move(m) => format!(
                    "{}:{}:{}:0 - move {} (side {}, pos {})",
                    m.order,
                    m.priority,
                    m.speed,
                    m.move_id.as_str(),
                    m.side_index,
                    m.pokemon_index
                ),
                Action::Switch(s) => format!(
                    "{}:{}:{}:0 - switch (side {}, pos {} -> {})",
                    s.order, s.priority, s.speed, s.side_index, s.pokemon_index, s.target_index
                ),
                Action::Team(t) => format!(
                    "1:{}:1:0 - team (side {}, pos {})",
                    t.priority, t.side_index, t.pokemon_index
                ),
                Action::Field(f) => {
                    format!("{}:{}:1:0 - {:?}", action.order(), f.priority, f.choice)
                }
                Action::Pokemon(p) => format!(
                    "{}:{}:{}:0 - {:?} (side {}, pos {})",
                    action.order(),
                    p.priority,
                    p.speed,
                    p.choice,
                    p.side_index,
                    p.pokemon_index
                ),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
