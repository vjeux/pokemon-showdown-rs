use crate::*;
use crate::dex::MoveData;

impl Dex {

    /// Get move by ID (equivalent to DexMoves.getByID)
    pub fn get_move_by_id(&self, id: &ID) -> Option<&MoveData> {
        self.moves.get(id)
    }
}
