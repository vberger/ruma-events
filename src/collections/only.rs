//! Enums for heterogeneous collections of events, exclusive to event types that implement "at
//! most" the trait of the same name.

use {CustomEvent, CustomRoomEvent, EventType, InvalidEvent, InvalidRoomEvent};
use call::answer::AnswerEvent;
use call::candidates::CandidatesEvent;
use call::hangup::HangupEvent;
use call::invite::InviteEvent;
use presence::PresenceEvent;
use receipt::ReceiptEvent;
use room::message::MessageEvent;
use room::redaction::RedactionEvent;
use tag::TagEvent;
use typing::TypingEvent;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use serde_json::{Value, from_value};

pub use super::all::StateEvent;

/// A basic event.
#[derive(Clone, Debug)]
pub enum Event {
    /// m.presence
    Presence(PresenceEvent),
    /// m.receipt
    Receipt(ReceiptEvent),
    /// m.tag
    Tag(TagEvent),
    /// m.typing
    Typing(TypingEvent),
    /// Any known basic event, but with missing or invalid contents.
    Invalid(InvalidEvent),
    /// Any basic event that is not part of the specification.
    Custom(CustomEvent),
}

/// A room event.
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
    /// m.room.message
    RoomMessage(MessageEvent),
    /// m.room.redaction
    RoomRedaction(RedactionEvent),
    /// Any known room event, but with missing or invalid contents.
    InvalidRoom(InvalidRoomEvent),
    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            Event::Presence(ref event) => event.serialize(serializer),
            Event::Receipt(ref event) => event.serialize(serializer),
            Event::Tag(ref event) => event.serialize(serializer),
            Event::Typing(ref event) => event.serialize(serializer),
            Event::Invalid(ref event) => event.serialize(serializer),
            Event::Custom(ref event) => event.serialize(serializer),
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
                match from_value::<CustomEvent>(value) {
                    Ok(event) => Ok(Event::Custom(event)),
                    Err(error) => Err(D::Error::custom(error.to_string())),
                }
            }
            EventType::CallAnswer | EventType::CallCandidates | EventType::CallHangup |
            EventType::CallInvite | EventType::RoomAliases | EventType::RoomAvatar |
            EventType::RoomCanonicalAlias | EventType::RoomCreate | EventType::RoomGuestAccess |
            EventType::RoomHistoryVisibility | EventType::RoomJoinRules | EventType::RoomMember |
            EventType::RoomMessage | EventType::RoomName | EventType::RoomPowerLevels |
            EventType::RoomRedaction | EventType::RoomThirdPartyInvite | EventType::RoomTopic => {
                Err(D::Error::custom("not exclusively a basic event".to_string()))
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
            RoomEvent::RoomMessage(ref event) => event.serialize(serializer),
            RoomEvent::RoomRedaction(ref event) => event.serialize(serializer),
            RoomEvent::InvalidRoom(ref event) => event.serialize(serializer),
            RoomEvent::CustomRoom(ref event) => event.serialize(serializer),
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
            EventType::RoomMessage => {
                match from_value::<MessageEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomMessage(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::RoomRedaction => {
                match from_value::<RedactionEvent>(value.clone()) {
                    Ok(event) => Ok(RoomEvent::RoomRedaction(event)),
                    Err(error) => invalid_room_event(value, error.to_string()),
                }
            }
            EventType::Custom(_) => {
                match from_value::<CustomRoomEvent>(value) {
                    Ok(event) => Ok(RoomEvent::CustomRoom(event)),
                    Err(error) => Err(D::Error::custom(error.to_string())),
                }
            }
            EventType::Presence | EventType::Receipt | EventType::RoomAliases |
            EventType::RoomAvatar | EventType::RoomCanonicalAlias | EventType::RoomCreate |
            EventType::RoomGuestAccess | EventType::RoomHistoryVisibility |
            EventType::RoomJoinRules | EventType::RoomMember | EventType::RoomName |
            EventType::RoomPowerLevels |EventType::RoomThirdPartyInvite | EventType::RoomTopic |
            EventType::Tag | EventType::Typing => {
                Err(D::Error::custom("not exclusively a room event".to_string()))
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

impl_from_t_for_event!(PresenceEvent, Presence);
impl_from_t_for_event!(ReceiptEvent, Receipt);
impl_from_t_for_event!(TagEvent, Tag);
impl_from_t_for_event!(TypingEvent, Typing);
impl_from_t_for_event!(CustomEvent, Custom);

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
impl_from_t_for_room_event!(MessageEvent, RoomMessage);
impl_from_t_for_room_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_room_event!(CustomRoomEvent, CustomRoom);
