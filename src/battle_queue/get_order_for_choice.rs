use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get the order value for a choice type
    pub fn get_order_for_choice(choice: &str) -> i32 {
        match choice {
            "team" => 1,
            "start" => 2,
            "instaswitch" => 3,
            "beforeTurn" => 4,
            "beforeTurnMove" => 5,
            "revivalblessing" => 6,
            "runSwitch" => 101,
            "switch" => 103,
            "megaEvo" | "megaEvoX" | "megaEvoY" => 104,
            "runDynamax" => 105,
            "terastallize" => 106,
            "priorityChargeMove" => 107,
            "shift" | "move" => 200,
            "residual" => 300,
            _ => 200,
        }
    }
}
