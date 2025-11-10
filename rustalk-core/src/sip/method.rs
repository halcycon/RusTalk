//! SIP Methods

use std::fmt;

/// SIP Method types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// INVITE - initiate a session
    Invite,
    /// ACK - acknowledge INVITE
    Ack,
    /// BYE - terminate a session
    Bye,
    /// CANCEL - cancel pending request
    Cancel,
    /// REGISTER - register contact
    Register,
    /// OPTIONS - query capabilities
    Options,
    /// INFO - send mid-session information
    Info,
    /// PRACK - provisional acknowledgement
    Prack,
    /// SUBSCRIBE - subscribe to events
    Subscribe,
    /// NOTIFY - notify of events
    Notify,
    /// UPDATE - update session
    Update,
    /// REFER - refer to another resource
    Refer,
    /// MESSAGE - instant message
    Message,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Invite => "INVITE",
            Method::Ack => "ACK",
            Method::Bye => "BYE",
            Method::Cancel => "CANCEL",
            Method::Register => "REGISTER",
            Method::Options => "OPTIONS",
            Method::Info => "INFO",
            Method::Prack => "PRACK",
            Method::Subscribe => "SUBSCRIBE",
            Method::Notify => "NOTIFY",
            Method::Update => "UPDATE",
            Method::Refer => "REFER",
            Method::Message => "MESSAGE",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "INVITE" => Some(Method::Invite),
            "ACK" => Some(Method::Ack),
            "BYE" => Some(Method::Bye),
            "CANCEL" => Some(Method::Cancel),
            "REGISTER" => Some(Method::Register),
            "OPTIONS" => Some(Method::Options),
            "INFO" => Some(Method::Info),
            "PRACK" => Some(Method::Prack),
            "SUBSCRIBE" => Some(Method::Subscribe),
            "NOTIFY" => Some(Method::Notify),
            "UPDATE" => Some(Method::Update),
            "REFER" => Some(Method::Refer),
            "MESSAGE" => Some(Method::Message),
            _ => None,
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
