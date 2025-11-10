//! Session management for B2BUA

use crate::b2bua::CallLeg;
use std::fmt;
use uuid::Uuid;

/// Unique session identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Initial state
    Initial,
    /// Ringing
    Ringing,
    /// Call established
    Established,
    /// Call being terminated
    Terminating,
    /// Call terminated
    Terminated,
}

/// Call session between two legs
#[derive(Debug, Clone)]
pub struct Session {
    id: SessionId,
    call_id: String,
    state: SessionState,
    a_leg: Option<CallLeg>,
    b_leg: Option<CallLeg>,
}

impl Session {
    pub fn new(call_id: String) -> Self {
        Self {
            id: SessionId::new(),
            call_id,
            state: SessionState::Initial,
            a_leg: None,
            b_leg: None,
        }
    }

    pub fn id(&self) -> &SessionId {
        &self.id
    }

    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
    }

    pub fn a_leg(&self) -> Option<&CallLeg> {
        self.a_leg.as_ref()
    }

    pub fn b_leg(&self) -> Option<&CallLeg> {
        self.b_leg.as_ref()
    }

    pub fn set_a_leg(&mut self, leg: CallLeg) {
        self.a_leg = Some(leg);
    }

    pub fn set_b_leg(&mut self, leg: CallLeg) {
        self.b_leg = Some(leg);
    }
}
