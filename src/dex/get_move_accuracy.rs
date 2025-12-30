use crate::*;
use crate::dex::Accuracy;

impl Dex {

    /// Get move accuracy as Option<i32> (None if always hits)
    pub fn get_move_accuracy(&self, move_name: &str) -> Option<i32> {
        self.get_move(move_name).and_then(|m| match &m.accuracy {
            Accuracy::Percent(p) => Some(*p),
            Accuracy::AlwaysHits => None,
        })
    }
}
