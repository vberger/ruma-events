//! Enums for heterogeneous collections of events, inclusive for every event type that implements
//! the trait of the same name.

use {
    CustomEvent, CustomRoomEvent, CustomStateEvent, EventType, InvalidEvent, InvalidRoomEvent,
    InvalidStateEvent
};
use call::answer::AnswerEvent;
use call::candidates::CandidatesEvent;
use call::hangup::HangupEvent;
use call::invite::InviteEvent;
use presence::PresenceEvent;
use receipt::ReceiptEvent;
use room::aliases::AliasesEvent;
use room::avatar::AvatarEvent;
use room::canonical_alias::CanonicalAliasEvent;
use room::create::CreateEvent;
use room::guest_access::GuestAccessEvent;
use room::history_visibility::HistoryVisibilityEvent;
use room::join_rules::JoinRulesEvent;
use room::member::MemberEvent;
use room::message::MessageEvent;
use room::name::NameEvent;
use room::power_levels::PowerLevelsEvent;
use room::redaction::RedactionEvent;
use room::third_party_invite::ThirdPartyInviteEvent;
use room::topic::TopicEvent;
use tag::TagEvent;
use typing::TypingEvent;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use serde_json::{Value, from_value};

/// A basic event, room event, or state event.
#[derive(Clone, Debug)]
pub enum Event {
    /// m.call.answer
    CallAnswer(AnswerEvent),
    /// m.call.candidates
    CallCandidates(CandidatesEvent),
    /// m.call.hangup
    CallHangup(HangupEvent),
    /// m.call.invite
    CallInvite(InviteEvent),
    /// m.presence
    Presence(PresenceEvent),
    /// m.receipt
    Receipt(ReceiptEvent),
    /// m.room.aliases
    RoomAliases(AliasesEvent),
    /// m.room.avatar
    RoomAvatar(AvatarEvent),
    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),
    /// m.room.create
    RoomCreate(CreateEvent),
    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),
    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),
    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),
    /// m.room.member
    RoomMember(MemberEvent),
    /// m.room.message
    RoomMessage(MessageEvent),
    /// m.room.name
    RoomName(NameEvent),
    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),
    /// m.room.redaction
    RoomRedaction(RedactionEvent),
    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),
    /// m.room.topic
    RoomTopic(TopicEvent),
    /// m.tag
    Tag(TagEvent),
    /// m.typing
    Typing(TypingEvent),
    /// Any known event, but with missing or invalid contents.
    Invalid(InvalidEvent),
    /// Any basic event that is not part of the specification.
    Custom(CustomEvent),
    /// Any known room event, but with missing or invalid contents.
    InvalidRoom(InvalidRoomEvent),
    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),
    /// Any known state event, but with missing or invalid contents.
    InvalidState(InvalidStateEvent),
    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}


/// A room event or state event.
#[derive(Clone, Debug)]
pub enum RoomEvent {
    /// m.call.answer
    CallAnswer(AnswerEvent),
    /// m.call.candidates
    CallCandidates(CandidatesEvent),
    /// m.call.hangup
    CallHangup(HangupEvent),
    /// m.call.invite
    CallInvite(InviteEvent),
    /// m.room.aliases
    RoomAliases(AliasesEvent),
    /// m.room.avatar
    RoomAvatar(AvatarEvent),
    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),
    /// m.room.create
    RoomCreate(CreateEvent),
    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),
    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),
    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),
    /// m.room.member
    RoomMember(MemberEvent),
    /// m.room.message
    RoomMessage(MessageEvent),
    /// m.room.name
    RoomName(NameEvent),
    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),
    /// m.room.redaction
    RoomRedaction(RedactionEvent),
    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),
    /// m.room.topic
    RoomTopic(TopicEvent),
    /// Any known room event, but with missing or invalid contents.
    InvalidRoom(InvalidRoomEvent),
    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),
    /// Any known state event, but with missing or invalid contents.
    InvalidState(InvalidStateEvent),
    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

/// A state event.
#[derive(Clone, Debug)]
pub enum StateEvent {
    /// m.room.aliases
    RoomAliases(AliasesEvent),
    /// m.room.avatar
    RoomAvatar(AvatarEvent),
    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),
    /// m.room.create
    RoomCreate(CreateEvent),
    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),
    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),
    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),
    /// m.room.member
    RoomMember(MemberEvent),
    /// m.room.name
    RoomName(NameEvent),
    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),
    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),
    /// m.room.topic
    RoomTopic(TopicEvent),
    /// One of the above, but with missing or invalid contents.
    InvalidState(InvalidStateEvent),
    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            Event::CallAnswer(ref event) => event.serialize(serializer),
            Event::CallCandidates(ref event) => event.serialize(serializer),
            Event::CallHangup(ref event) => event.serialize(serializer),
            Event::CallInvite(ref event) => event.serialize(serializer),
            Event::Presence(ref event) => event.serialize(serializer),
            Event::Receipt(ref event) => event.serialize(serializer),
            Event::RoomAliases(ref event) => event.serialize(serializer),
            Event::RoomAvatar(ref event) => event.serialize(serializer),
            Event::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            Event::RoomCreate(ref event) => event.serialize(serializer),
            Event::RoomGuestAccess(ref event) => event.serialize(serializer),
            Event::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            Event::RoomJoinRules(ref event) => event.serialize(serializer),
            Event::RoomMember(ref event) => event.serialize(serializer),
            Event::RoomMessage(ref event) => event.serialize(serializer),
            Event::RoomName(ref event) => event.serialize(serializer),
            Event::RoomPowerLevels(ref event) => event.serialize(serializer),
            Event::RoomRedaction(ref event) => event.serialize(serializer),
            Event::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            Event::RoomTopic(ref event) => event.serialize(serializer),
            Event::Tag(ref event) => event.serialize(serializer),
            Event::Typing(ref event) => event.serialize(serializer),
            Event::Invalid(ref event) => event.serialize(serializer),
            Event::Custom(ref event) => event.serialize(serializer),
            Event::InvalidRoom(ref event) => event.serialize(serializer),
            Event::CustomRoom(ref event) => event.serialize(serializer),
            Event::InvalidState(ref event) => event.serialize(serializer),
            Event::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let invalid_event = |value, error| {
            match from_value::<InvalidEvent>(value) {
                Ok(event) => Ok(Event::Invalid(event.with_error(error))),
                Err(error) => Err(D::Error::custom(error.to_string())),
            }
        };

        let invalid_room_event = |value, error| {
            match from_value::<InvalidRoomEvent>(value) {
                Ok(event) => Ok(Event::InvalidRoom(event.with_error(error))),
                Err(error) => Err(D::Error::custom(error.to_string())),
            }
        };

        let invalid_state_event = |value, error| {
            match from_value::<InvalidStateEvent>(value) {
                Ok(event) => Ok(Event::InvalidState(event.with_error(error))),
                Err(error) => Err(D::Error::custom(error.to_string())),
            }
        };

        let value: Value = Deserialize::deserialize(deserializer)?;

        let event_type_value = match value.get("type") {
            Some(value) => value.clone(),
            None => return Err(D::Error::missing_field("type")),
        };

        let event_type = match from_value::<EventType>(event_type_value.clone()) {
            Ok(event_type) => event_type,
            Err(error) => return Err(D::Error::custom(error.to_string())),
        };

        match event_type {
            EventType::CallAnswer => {
                match from_value::<AnswerEvent>(value.clone()) {
                    Ok(event) => Ok(Event::CallAnswer(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::CallCandidates => {
                match from_value::<CandidatesEvent>(value.clone()) {
                    Ok(event) => Ok(Event::CallCandidates(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::CallHangup => {
                match from_value::<HangupEvent>(value.clone()) {
                    Ok(event) => Ok(Event::CallHangup(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::CallInvite => {
                match from_value::<InviteEvent>(value.clone()) {
                    Ok(event) => Ok(Event::CallInvite(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::Presence => {
                match from_value::<PresenceEvent>(value.clone()) {
                    Ok(event) => Ok(Event::Presence(event)),
                    Err(error) => invalid_event(value, error.to_string()),
                }
            }
            EventType::Receipt => {
                match from_value::<ReceiptEvent>(value.clone()) {
                    Ok(event) => Ok(Event::Receipt(event)),
                    Err(error) => invalid_event(value, error.to_string()),
                }
            }
            EventType::RoomAliases => {
                match from_value::<AliasesEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomAliases(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomAvatar => {
                match from_value::<AvatarEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomAvatar(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomCanonicalAlias => {
                match from_value::<CanonicalAliasEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomCanonicalAlias(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomCreate => {
                match from_value::<CreateEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomCreate(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomGuestAccess => {
                match from_value::<GuestAccessEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomGuestAccess(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomHistoryVisibility => {
                match from_value::<HistoryVisibilityEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomHistoryVisibility(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomJoinRules => {
                match from_value::<JoinRulesEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomJoinRules(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomMember => {
                match from_value::<MemberEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomMember(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomMessage => {
                match from_value::<MessageEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomMessage(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::RoomName => {
                match from_value::<NameEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomName(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomPowerLevels => {
                match from_value::<PowerLevelsEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomPowerLevels(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomRedaction => {
                match from_value::<RedactionEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomRedaction(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::RoomThirdPartyInvite => {
                match from_value::<ThirdPartyInviteEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomThirdPartyInvite(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomTopic => {
                match from_value::<TopicEvent>(value.clone()) {
                    Ok(event) => Ok(Event::RoomTopic(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::Tag => {
                match from_value::<TagEvent>(value.clone()) {
                    Ok(event) => Ok(Event::Tag(event)),
                    Err(error) => invalid_event(value, error.to_string()),
                }
            }
            EventType::Typing => {
                match from_value::<TypingEvent>(value.clone()) {
                    Ok(event) => Ok(Event::Typing(event)),
                    Err(error) => invalid_event(value, error.to_string()),
                }
            }
            EventType::Custom(_) => {
                if value.get("state_key").is_some() {
                    match from_value::<CustomStateEvent>(value) {
                        Ok(event) => Ok(Event::CustomState(event)),
                        Err(error) => Err(D::Error::custom(error.to_string())),
                    }
                } else if value.get("event_id").is_some() && value.get("room_id").is_some() &&
                    value.get("sender").is_some() {
                    match from_value::<CustomRoomEvent>(value) {
                        Ok(event) => Ok(Event::CustomRoom(event)),
                        Err(error) => Err(D::Error::custom(error.to_string())),
                    }
                } else {
                    match from_value::<CustomEvent>(value) {
                        Ok(event) => Ok(Event::Custom(event)),
                        Err(error) => Err(D::Error::custom(error.to_string())),
                    }
                }
            }
        }
    }
}

impl Serialize for RoomEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            RoomEvent::CallAnswer(ref event) => event.serialize(serializer),
            RoomEvent::CallCandidates(ref event) => event.serialize(serializer),
            RoomEvent::CallHangup(ref event) => event.serialize(serializer),
            RoomEvent::CallInvite(ref event) => event.serialize(serializer),
            RoomEvent::RoomAliases(ref event) => event.serialize(serializer),
            RoomEvent::RoomAvatar(ref event) => event.serialize(serializer),
            RoomEvent::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            RoomEvent::RoomCreate(ref event) => event.serialize(serializer),
            RoomEvent::RoomGuestAccess(ref event) => event.serialize(serializer),
            RoomEvent::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            RoomEvent::RoomJoinRules(ref event) => event.serialize(serializer),
            RoomEvent::RoomMember(ref event) => event.serialize(serializer),
            RoomEvent::RoomMessage(ref event) => event.serialize(serializer),
            RoomEvent::RoomName(ref event) => event.serialize(serializer),
            RoomEvent::RoomPowerLevels(ref event) => event.serialize(serializer),
            RoomEvent::RoomRedaction(ref event) => event.serialize(serializer),
            RoomEvent::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            RoomEvent::RoomTopic(ref event) => event.serialize(serializer),
            RoomEvent::InvalidRoom(ref event) => event.serialize(serializer),
            RoomEvent::CustomRoom(ref event) => event.serialize(serializer),
            RoomEvent::InvalidState(ref event) => event.serialize(serializer),
            RoomEvent::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for RoomEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let invalid_room_event = |value, error| {
            match from_value::<InvalidRoomEvent>(value) {
                Ok(event) => Ok(RoomEvent::InvalidRoom(event.with_error(error))),
                Err(error) => Err(D::Error::custom(error.to_string())),
            }
        };

        let invalid_state_event = |value, error| {
            match from_value::<InvalidStateEvent>(value) {
                Ok(event) => Ok(RoomEvent::InvalidState(event.with_error(error))),
                Err(error) => Err(D::Error::custom(error.to_string())),
            }
        };

        let value: Value = Deserialize::deserialize(deserializer)?;

        let event_type_value = match value.get("type") {
            Some(value) => value.clone(),
            None => return Err(D::Error::missing_field("type")),
        };

        let event_type = match from_value::<EventType>(event_type_value.clone()) {
            Ok(event_type) => event_type,
            Err(error) => return Err(D::Error::custom(error.to_string())),
        };

        match event_type {
            EventType::CallAnswer => {
                match from_value::<AnswerEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::CallAnswer(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::CallCandidates => {
                match from_value::<CandidatesEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::CallCandidates(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::CallHangup => {
                match from_value::<HangupEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::CallHangup(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::CallInvite => {
                match from_value::<InviteEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::CallInvite(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::RoomAliases => {
                match from_value::<AliasesEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomAliases(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomAvatar => {
                match from_value::<AvatarEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomAvatar(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomCanonicalAlias => {
                match from_value::<CanonicalAliasEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomCanonicalAlias(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomCreate => {
                match from_value::<CreateEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomCreate(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomGuestAccess => {
                match from_value::<GuestAccessEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomGuestAccess(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomHistoryVisibility => {
                match from_value::<HistoryVisibilityEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomHistoryVisibility(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomJoinRules => {
                match from_value::<JoinRulesEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomJoinRules(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomMember => {
                match from_value::<MemberEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomMember(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomMessage => {
                match from_value::<MessageEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomMessage(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::RoomName => {
                match from_value::<NameEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomName(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomPowerLevels => {
                match from_value::<PowerLevelsEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomPowerLevels(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomRedaction => {
                match from_value::<RedactionEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomRedaction(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::RoomThirdPartyInvite => {
                match from_value::<ThirdPartyInviteEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomThirdPartyInvite(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomTopic => {
                match from_value::<TopicEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomTopic(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::Custom(_) => {
                if value.get("state_key").is_some() {
                    match from_value::<CustomStateEvent>(value) {
                        Ok(event) => Ok(RoomEvent::CustomState(event)),
                        Err(error) => Err(D::Error::custom(error.to_string())),
                    }
                } else {
                    match from_value::<CustomRoomEvent>(value) {
                        Ok(event) => Ok(RoomEvent::CustomRoom(event)),
                        Err(error) => Err(D::Error::custom(error.to_string())),
                    }
                }
            }
            EventType::Presence | EventType::Receipt | EventType::Tag | EventType::Typing => {
                Err(D::Error::custom("not a room event".to_string()))
            }
        }
    }
}

impl Serialize for StateEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            StateEvent::RoomAliases(ref event) => event.serialize(serializer),
            StateEvent::RoomAvatar(ref event) => event.serialize(serializer),
            StateEvent::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            StateEvent::RoomCreate(ref event) => event.serialize(serializer),
            StateEvent::RoomGuestAccess(ref event) => event.serialize(serializer),
            StateEvent::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            StateEvent::RoomJoinRules(ref event) => event.serialize(serializer),
            StateEvent::RoomMember(ref event) => event.serialize(serializer),
            StateEvent::RoomName(ref event) => event.serialize(serializer),
            StateEvent::RoomPowerLevels(ref event) => event.serialize(serializer),
            StateEvent::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            StateEvent::RoomTopic(ref event) => event.serialize(serializer),
            StateEvent::InvalidState(ref event) => event.serialize(serializer),
            StateEvent::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for StateEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let invalid_state_event = |value, error| {
            match from_value::<InvalidStateEvent>(value) {
                Ok(event) => Ok(StateEvent::InvalidState(event.with_error(error))),
                Err(error) => Err(D::Error::custom(error.to_string())),
            }
        };

        let value: Value = Deserialize::deserialize(deserializer)?;

        let event_type_value = match value.get("type") {
            Some(value) => value.clone(),
            None => return Err(D::Error::missing_field("type")),
        };

        let event_type = match from_value::<EventType>(event_type_value.clone()) {
            Ok(event_type) => event_type,
            Err(error) => return Err(D::Error::custom(error.to_string())),
        };

        match event_type {
            EventType::RoomAliases => {
                match from_value::<AliasesEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomAliases(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomAvatar => {
                match from_value::<AvatarEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomAvatar(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomCanonicalAlias => {
                match from_value::<CanonicalAliasEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomCanonicalAlias(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomCreate => {
                match from_value::<CreateEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomCreate(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomGuestAccess => {
                match from_value::<GuestAccessEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomGuestAccess(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomHistoryVisibility => {
                match from_value::<HistoryVisibilityEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomHistoryVisibility(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomJoinRules => {
                match from_value::<JoinRulesEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomJoinRules(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomMember => {
                match from_value::<MemberEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomMember(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomName => {
                match from_value::<NameEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomName(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomPowerLevels => {
                match from_value::<PowerLevelsEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomPowerLevels(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomThirdPartyInvite => {
                match from_value::<ThirdPartyInviteEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomThirdPartyInvite(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::RoomTopic => {
                match from_value::<TopicEvent>(value.clone()) {
                    Ok(event) => Ok(StateEvent::RoomTopic(event)),
                    Err(error) => invalid_state_event(value, error.to_string()),
                }
            }
            EventType::Custom(_) => {
                match from_value::<CustomStateEvent>(value) {
                    Ok(event) => Ok(StateEvent::CustomState(event)),
                    Err(error) => Err(D::Error::custom(error.to_string())),
                }
            }
            EventType::CallAnswer | EventType::CallCandidates | EventType::CallHangup |
            EventType::CallInvite | EventType::Presence | EventType::Receipt |
            EventType::RoomMessage | EventType::RoomRedaction | EventType::Tag |
            EventType::Typing => {
                Err(D::Error::custom("not a state event".to_string()))
            }
        }
    }
}

macro_rules! impl_from_t_for_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for Event {
            fn from(event: $ty) -> Self {
                Event::$variant(event)
            }
        }
    };
}

impl_from_t_for_event!(AnswerEvent, CallAnswer);
impl_from_t_for_event!(CandidatesEvent, CallCandidates);
impl_from_t_for_event!(HangupEvent, CallHangup);
impl_from_t_for_event!(InviteEvent, CallInvite);
impl_from_t_for_event!(PresenceEvent, Presence);
impl_from_t_for_event!(ReceiptEvent, Receipt);
impl_from_t_for_event!(AliasesEvent, RoomAliases);
impl_from_t_for_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_event!(CreateEvent, RoomCreate);
impl_from_t_for_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_event!(MemberEvent, RoomMember);
impl_from_t_for_event!(MessageEvent, RoomMessage);
impl_from_t_for_event!(NameEvent, RoomName);
impl_from_t_for_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_event!(TopicEvent, RoomTopic);
impl_from_t_for_event!(TagEvent, Tag);
impl_from_t_for_event!(TypingEvent, Typing);
impl_from_t_for_event!(CustomEvent, Custom);
impl_from_t_for_event!(CustomRoomEvent, CustomRoom);
impl_from_t_for_event!(CustomStateEvent, CustomState);

macro_rules! impl_from_t_for_room_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for RoomEvent {
            fn from(event: $ty) -> Self {
                RoomEvent::$variant(event)
            }
        }
    };
}

impl_from_t_for_room_event!(AnswerEvent, CallAnswer);
impl_from_t_for_room_event!(CandidatesEvent, CallCandidates);
impl_from_t_for_room_event!(HangupEvent, CallHangup);
impl_from_t_for_room_event!(InviteEvent, CallInvite);
impl_from_t_for_room_event!(AliasesEvent, RoomAliases);
impl_from_t_for_room_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_room_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_room_event!(CreateEvent, RoomCreate);
impl_from_t_for_room_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_room_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_room_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_room_event!(MemberEvent, RoomMember);
impl_from_t_for_room_event!(MessageEvent, RoomMessage);
impl_from_t_for_room_event!(NameEvent, RoomName);
impl_from_t_for_room_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_room_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_room_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_room_event!(TopicEvent, RoomTopic);
impl_from_t_for_room_event!(CustomRoomEvent, CustomRoom);
impl_from_t_for_room_event!(CustomStateEvent, CustomState);

macro_rules! impl_from_t_for_state_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for StateEvent {
            fn from(event: $ty) -> Self {
                StateEvent::$variant(event)
            }
        }
    };
}

impl_from_t_for_state_event!(AliasesEvent, RoomAliases);
impl_from_t_for_state_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_state_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_state_event!(CreateEvent, RoomCreate);
impl_from_t_for_state_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_state_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_state_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_state_event!(MemberEvent, RoomMember);
impl_from_t_for_state_event!(NameEvent, RoomName);
impl_from_t_for_state_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_state_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_state_event!(TopicEvent, RoomTopic);
impl_from_t_for_state_event!(CustomStateEvent, CustomState);
