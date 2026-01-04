//! BattleStream::receiveRequest - Process request from battle
//!
//! 1:1 port of receiveRequest from battle-stream.ts

use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Process a request from the battle
    /// Equivalent to receiveRequest() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   abstract receiveRequest(request: ChoiceRequest): void;
    ///
    /// Note: In JavaScript, this is an abstract method meant to be overridden
    /// by concrete implementations (like BattlePlayer subclasses). In the base
    /// BattleStream, it's never implemented directly.
    ///
    /// For the Rust port, since we don't have an async streaming model that
    /// requires handling requests in subclasses, we implement a simple version
    /// that stores the request for later retrieval.
    pub fn receive_request(&mut self, request: serde_json::Value) {
        // Store the request - concrete implementations can override this behavior
        // or use the stored request as needed
        self.last_request = Some(request);
    }
}
