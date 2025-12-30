use crate::*;

impl<'a> BattleActions<'a> {

    /// Get accuracy modifier from stages
    pub fn get_accuracy_modifier(stages: i8) -> (i32, i32) {
        match stages {
            -6 => (3, 9),
            -5 => (3, 8),
            -4 => (3, 7),
            -3 => (3, 6),
            -2 => (3, 5),
            -1 => (3, 4),
            0 => (3, 3),
            1 => (4, 3),
            2 => (5, 3),
            3 => (6, 3),
            4 => (7, 3),
            5 => (8, 3),
            6 => (9, 3),
            _ if stages < -6 => (3, 9),
            _ => (9, 3),
        }
    }
}
