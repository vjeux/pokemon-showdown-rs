//! Extract Channel Messages
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! 1:1 port of extractChannelMessages from battle.ts

use std::collections::{HashMap, HashSet};
use regex::Regex;

/// Channel ID - represents a player channel (0-4) or omniscient (-1)
///
/// JavaScript:
/// export type ChannelID = 0 | 1 | 2 | 3 | 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelID {
    Omniscient = -1,
    Player0 = 0,
    Player1 = 1,
    Player2 = 2,
    Player3 = 3,
    Player4 = 4,
}

impl ChannelID {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            -1 => Some(ChannelID::Omniscient),
            0 => Some(ChannelID::Player0),
            1 => Some(ChannelID::Player1),
            2 => Some(ChannelID::Player2),
            3 => Some(ChannelID::Player3),
            4 => Some(ChannelID::Player4),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

/// Channel messages - maps channel IDs to arrays of message lines
///
/// JavaScript:
/// export type ChannelMessages<T extends ChannelID | -1> = Record<T, string[]>;
pub type ChannelMessages = HashMap<i32, Vec<String>>;

/// Extract channel messages from a battle log
/// Handles split messages (different content for different players)
///
/// JavaScript:
/// export function extractChannelMessages<T extends ChannelID | -1>(message: string, channelIds: T[]): ChannelMessages<T> {
///     const channelIdSet = new Set(channelIds);
///     const channelMessages: ChannelMessages<ChannelID | -1> = {
///         [-1]: [],
///         0: [],
///         1: [],
///         2: [],
///         3: [],
///         4: [],
///     };
///
///     for (const [lineMatch, playerMatch, secretMessage, sharedMessage] of message.matchAll(splitRegex)) {
///         const player = playerMatch ? parseInt(playerMatch) : 0;
///         for (const channelId of channelIdSet) {
///             let line = lineMatch;
///             if (player) {
///                 line = channelId === -1 || player === channelId ? secretMessage : sharedMessage;
///                 if (!line) continue;
///             }
///             channelMessages[channelId].push(line);
///         }
///     }
///
///     return channelMessages;
/// }
pub fn extract_channel_messages(message: &str, channel_ids: &[i32]) -> ChannelMessages {
    // JavaScript: const splitRegex = /^\|split\|p([1234])\n(.*)\n(.*)|.+/gm;
    let split_regex = Regex::new(r"(?m)^\|split\|p([1234])\n(.*)\n(.*)|.+").unwrap();

    // JavaScript: const channelIdSet = new Set(channelIds);
    let channel_id_set: HashSet<i32> = channel_ids.iter().copied().collect();

    // JavaScript: const channelMessages: ChannelMessages<ChannelID | -1> = {
    //     [-1]: [],
    //     0: [],
    //     1: [],
    //     2: [],
    //     3: [],
    //     4: [],
    // };
    let mut channel_messages: ChannelMessages = HashMap::new();
    for &channel_id in &channel_id_set {
        channel_messages.insert(channel_id, Vec::new());
    }

    // JavaScript: for (const [lineMatch, playerMatch, secretMessage, sharedMessage] of message.matchAll(splitRegex)) {
    for captures in split_regex.captures_iter(message) {
        // Full match is captures[0]
        let line_match = captures.get(0).unwrap().as_str();
        let player_match = captures.get(1).map(|m| m.as_str());
        let secret_message = captures.get(2).map(|m| m.as_str());
        let shared_message = captures.get(3).map(|m| m.as_str());

        // JavaScript: const player = playerMatch ? parseInt(playerMatch) : 0;
        let player = if let Some(pm) = player_match {
            pm.parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        // JavaScript: for (const channelId of channelIdSet) {
        for &channel_id in &channel_id_set {
            // JavaScript: let line = lineMatch;
            let mut line = line_match;

            // JavaScript: if (player) {
            if player != 0 {
                // JavaScript: line = channelId === -1 || player === channelId ? secretMessage : sharedMessage;
                line = if channel_id == -1 || player == channel_id {
                    secret_message.unwrap_or("")
                } else {
                    shared_message.unwrap_or("")
                };

                // JavaScript: if (!line) continue;
                if line.is_empty() {
                    continue;
                }
            }

            // JavaScript: channelMessages[channelId].push(line);
            if let Some(messages) = channel_messages.get_mut(&channel_id) {
                messages.push(line.to_string());
            }
        }
    }

    // JavaScript: return channelMessages;
    channel_messages
}
