//! SharedActiveMove - Reference-counted wrapper for ActiveMove
//!
//! This provides cheap cloning for ActiveMove by using Rc<RefCell<>>.
//! Similar to SharedEffectState, this mimics JavaScript's reference semantics.

use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::ActiveMove;

/// Shared active move - allows multiple references to the same move
/// This mimics JavaScript's reference semantics where activeMove is shared.
///
/// Cloning a SharedActiveMove is cheap (just increments the reference count).
/// Serialization/deserialization works transparently - it serializes the inner ActiveMove.
pub struct SharedActiveMove(Rc<RefCell<ActiveMove>>);

impl SharedActiveMove {
    /// Create a new SharedActiveMove with the given ActiveMove
    pub fn new(active_move: ActiveMove) -> Self {
        Self(Rc::new(RefCell::new(active_move)))
    }

    /// Borrow the inner ActiveMove immutably
    pub fn borrow(&self) -> Ref<'_, ActiveMove> {
        self.0.borrow()
    }

    /// Borrow the inner ActiveMove mutably
    pub fn borrow_mut(&self) -> RefMut<'_, ActiveMove> {
        self.0.borrow_mut()
    }

    /// Get the reference count (for debugging)
    pub fn ref_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

impl Clone for SharedActiveMove {
    /// Cheap clone - just increments the reference count
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl fmt::Debug for SharedActiveMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SharedActiveMove")
            .field(&*self.0.borrow())
            .finish()
    }
}

// Serialize the inner ActiveMove
impl serde::Serialize for SharedActiveMove {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.borrow().serialize(serializer)
    }
}

// Deserialize creates a new SharedActiveMove with the deserialized ActiveMove
impl<'de> serde::Deserialize<'de> for SharedActiveMove {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let active_move = ActiveMove::deserialize(deserializer)?;
        Ok(Self::new(active_move))
    }
}

// Allow dereferencing to access inner RefCell directly
// Note: For mutation, use borrow_mut() explicitly
impl Deref for SharedActiveMove {
    type Target = RefCell<ActiveMove>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
