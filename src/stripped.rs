//! Stripped-down versions of certain state events.

use serde::{Deserialize, Serialize};

use EventType;
use room::avatar::AvatarEventContent;
use room::canonical_alias::CanonicalAliasEventContent;
use room::join_rules::JoinRulesEventContent;
use room::name::NameEventContent;

/// A stripped-down version of a state event that is included along with some other events.
#[derive(Debug, Deserialize, Serialize)]
pub enum StrippedState {
    RoomAvatar(StrippedRoomAvatar),
    RoomCanonicalAlias(StrippedRoomCanonicalAlias),
    RoomJoinRules(StrippedRoomJoinRules),
    RoomName(StrippedRoomName),
}

/// The general form of a `StrippedState`.
#[derive(Debug, Deserialize, Serialize)]
pub struct StrippedStateContent<T> where T: Deserialize + Serialize {
    pub content: T,
    #[serde(rename="type")]
    pub event_type: EventType,
    pub state_key: String,
}

pub type StrippedRoomAvatar = StrippedStateContent<AvatarEventContent>;
pub type StrippedRoomCanonicalAlias = StrippedStateContent<CanonicalAliasEventContent>;
pub type StrippedRoomJoinRules = StrippedStateContent<JoinRulesEventContent>;
pub type StrippedRoomName = StrippedStateContent<NameEventContent>;