use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::MoveActionType;
use crate::battle_queue::PokemonAction;
use crate::battle_queue::PokemonActionType;
use crate::battle_queue::SwitchActionType;

impl BattleQueue {

    /// Resolve an action choice into one or more actions
    /// Equivalent to resolveAction in battle-queue.ts
    /// This creates the appropriate order values and may add additional actions
    /// (like megaEvo, terastallize, etc.) based on the choice
    // TypeScript source:
    // /**
    // 	 * Takes an ActionChoice, and fills it out into a full Action object.
    // 	 *
    // 	 * Returns an array of Actions because some ActionChoices (like mega moves)
    // 	 * resolve to two Actions (mega evolution + use move)
    // 	 */
    // 	resolveAction(action: ActionChoice, midTurn = false): Action[] {
    // 		if (!action) throw new Error(`Action not passed to resolveAction`);
    // 		if (action.choice === 'pass') return [];
    // 		const actions = [action];
    //
    // 		if (!action.side && action.pokemon) action.side = action.pokemon.side;
    // 		if (!action.move && action.moveid) action.move = this.battle.dex.getActiveMove(action.moveid);
    // 		if (!action.order) {
    // 			const orders: { [choice: string]: number } = {
    // 				team: 1,
    // 				start: 2,
    // 				instaswitch: 3,
    // 				beforeTurn: 4,
    // 				beforeTurnMove: 5,
    // 				revivalblessing: 6,
    //
    // 				runSwitch: 101,
    // 				switch: 103,
    // 				megaEvo: 104,
    // 				megaEvoX: 104,
    // 				megaEvoY: 104,
    // 				runDynamax: 105,
    // 				terastallize: 106,
    // 				priorityChargeMove: 107,
    //
    // 				shift: 200,
    // 				// default is 200 (for moves)
    //
    // 				residual: 300,
    // 			};
    // 			if (action.choice in orders) {
    // 				action.order = orders[action.choice];
    // 			} else {
    // 				action.order = 200;
    // 				if (!['move', 'event'].includes(action.choice)) {
    // 					throw new Error(`Unexpected orderless action ${action.choice}`);
    // 				}
    // 			}
    // 		}
    // 		if (!midTurn) {
    // 			if (action.choice === 'move') {
    // 				if (!action.maxMove && !action.zmove && action.move.beforeTurnCallback) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'beforeTurnMove', pokemon: action.pokemon, move: action.move, targetLoc: action.targetLoc,
    // 					}));
    // 				}
    // 				if (action.mega && !action.pokemon.isSkyDropped()) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'megaEvo',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.megax && !action.pokemon.isSkyDropped()) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'megaEvoX',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.megay && !action.pokemon.isSkyDropped()) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'megaEvoY',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.terastallize && !action.pokemon.terastallized) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'terastallize',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.maxMove && !action.pokemon.volatiles['dynamax']) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'runDynamax',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (!action.maxMove && !action.zmove && action.move.priorityChargeCallback) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'priorityChargeMove',
    // 						pokemon: action.pokemon,
    // 						move: action.move,
    // 					}));
    // 				}
    // 				action.fractionalPriority = this.battle.runEvent('FractionalPriority', action.pokemon, null, action.move, 0);
    // 			} else if (['switch', 'instaswitch'].includes(action.choice)) {
    // 				if (typeof action.pokemon.switchFlag === 'string') {
    // 					action.sourceEffect = this.battle.dex.moves.get(action.pokemon.switchFlag as ID) as any;
    // 				}
    // 				action.pokemon.switchFlag = false;
    // 			}
    // 		}
    //
    // 		const deferPriority = this.battle.gen === 7 && action.mega && action.mega !== 'done';
    // 		if (action.move) {
    // 			let target = null;
    // 			action.move = this.battle.dex.getActiveMove(action.move);
    //
    // 			if (!action.targetLoc) {
    // 				target = this.battle.getRandomTarget(action.pokemon, action.move);
    // 				// TODO: what actually happens here?
    // 				if (target) action.targetLoc = action.pokemon.getLocOf(target);
    // 			}
    // 			action.originalTarget = action.pokemon.getAtLoc(action.targetLoc);
    // 		}
    // 		if (!deferPriority) this.battle.getActionSpeed(action);
    // 		return actions as any;
    // 	}
    //
    pub fn resolve_action(&self, action: &mut Action, mid_turn: bool) -> Vec<Action> {
        let mut actions = Vec::new();

        // Set order based on choice type
        match action {
            Action::Move(m) => {
                // Set order if not already set
                if m.order == 0 {
                    m.order = match m.choice {
                        MoveActionType::Move => 200,
                        MoveActionType::BeforeTurnMove => 5,
                        MoveActionType::PriorityChargeMove => 107,
                    };
                }

                if !mid_turn {
                    // Add mega evolution action if needed
                    if m.mega {
                        actions.push(Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::MegaEvo,
                            order: 104,
                            priority: 0,
                            speed: m.speed,
                            pokemon_index: m.pokemon_index,
                            side_index: m.side_index,
                            event: None,
                        }));
                    }

                    // Add terastallize action if needed
                    if m.terastallize.is_some() {
                        actions.push(Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::Terastallize,
                            order: 106,
                            priority: 0,
                            speed: m.speed,
                            pokemon_index: m.pokemon_index,
                            side_index: m.side_index,
                            event: None,
                        }));
                    }

                    // Add dynamax action if using max move
                    if m.max_move.is_some() {
                        actions.push(Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::RunDynamax,
                            order: 105,
                            priority: 0,
                            speed: m.speed,
                            pokemon_index: m.pokemon_index,
                            side_index: m.side_index,
                            event: None,
                        }));
                    }
                }
            }
            Action::Switch(s) => {
                if s.order == 0 {
                    s.order = match s.choice {
                        SwitchActionType::Switch => 103,
                        SwitchActionType::InstaSwitch => 3,
                        SwitchActionType::RevivalBlessing => 6,
                    };
                }
            }
            Action::Team(_) => {
                // Team actions have order 1
            }
            Action::Field(_) => {
                // Field actions keep their order
            }
            Action::Pokemon(p) => {
                if p.order == 0 {
                    p.order = match p.choice {
                        PokemonActionType::Start => 2,
                        PokemonActionType::BeforeTurn => 4,
                        PokemonActionType::RunSwitch => 101,
                        PokemonActionType::MegaEvo => 104,
                        PokemonActionType::MegaEvoX => 104,
                        PokemonActionType::MegaEvoY => 104,
                        PokemonActionType::RunDynamax => 105,
                        PokemonActionType::Terastallize => 106,
                        PokemonActionType::Shift | PokemonActionType::Event => 200,
                        PokemonActionType::Residual => 300,
                    };
                }
            }
        }

        // Add the main action
        actions.push(action.clone());

        actions
    }
}
