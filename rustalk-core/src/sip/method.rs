//! SIP Methods

use std::fmt;
use std::str::FromStr;

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
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INVITE" => Ok(Method::Invite),
            "ACK" => Ok(Method::Ack),
            "BYE" => Ok(Method::Bye),
            "CANCEL" => Ok(Method::Cancel),
            "REGISTER" => Ok(Method::Register),
            "OPTIONS" => Ok(Method::Options),
            "INFO" => Ok(Method::Info),
            "PRACK" => Ok(Method::Prack),
            "SUBSCRIBE" => Ok(Method::Subscribe),
            "NOTIFY" => Ok(Method::Notify),
            "UPDATE" => Ok(Method::Update),
            "REFER" => Ok(Method::Refer),
            "MESSAGE" => Ok(Method::Message),
            _ => Err(format!("Unknown SIP method: {}", s)),
        }
    }
}
