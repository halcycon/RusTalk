//! B2BUA (Back-to-Back User Agent) implementation

use crate::sip::{Message, Method, Request, Response, StatusCode};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

pub mod call_leg;
pub mod session;

pub use call_leg::CallLeg;
pub use session::{Session, SessionId};

/// B2BUA core engine
///
/// The B2BUA acts as both a UAC (User Agent Client) and UAS (User Agent Server),
/// maintaining call state between two legs of a call.
#[derive(Clone)]
pub struct B2BUA {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
}

impl B2BUA {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle incoming SIP message
    pub async fn handle_message(&self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Request(req) => self.handle_request(req).await,
            Message::Response(res) => self.handle_response(res).await,
        }
    }

    /// Handle SIP request
    async fn handle_request(&self, request: Request) -> Result<Option<Message>> {
        info!("Handling {} request to {}", request.method, request.uri);

        match request.method {
            Method::Invite => self.handle_invite(request).await,
            Method::Bye => self.handle_bye(request).await,
            Method::Options => self.handle_options(request).await,
            Method::Ack => self.handle_ack(request).await,
            Method::Cancel => self.handle_cancel(request).await,
            _ => {
                debug!("Method {} not implemented", request.method);
                Ok(Some(Message::Response(Response::new(
                    StatusCode::NOT_IMPLEMENTED,
                ))))
            }
        }
    }

    /// Handle SIP response
    async fn handle_response(&self, response: Response) -> Result<Option<Message>> {
        info!("Handling response: {}", response.status_code);

        // Find the session this response belongs to
        let call_id = response
            .get_header_value("Call-ID")
            .ok_or_else(|| anyhow::anyhow!("No Call-ID in response"))?;

        let sessions = self.sessions.read().await;
        if let Some(_session) = sessions.values().find(|s| s.call_id() == call_id) {
            // Forward response to the other leg
            debug!("Forwarding response to other leg");
            // In a real implementation, we would modify headers and forward
            Ok(None)
        } else {
            error!("No session found for Call-ID: {}", call_id);
            Ok(None)
        }
    }

    /// Handle INVITE request - establish new session
    async fn handle_invite(&self, request: Request) -> Result<Option<Message>> {
        let call_id = request
            .get_header_value("Call-ID")
            .ok_or_else(|| anyhow::anyhow!("No Call-ID in INVITE"))?
            .to_string();

        info!("Creating new session for Call-ID: {}", call_id);

        // Create new session
        let session = Session::new(call_id.clone());

        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id().clone(), session);

        // Send 100 Trying
        let response = Response::new(StatusCode::TRYING).with_header("Call-ID", call_id.as_str());

        Ok(Some(Message::Response(response)))
    }

    /// Handle BYE request - terminate session
    async fn handle_bye(&self, request: Request) -> Result<Option<Message>> {
        let call_id = request
            .get_header_value("Call-ID")
            .ok_or_else(|| anyhow::anyhow!("No Call-ID in BYE"))?;

        info!("Terminating session for Call-ID: {}", call_id);

        // Find and remove session
        let mut sessions = self.sessions.write().await;
        let session_id = sessions
            .values()
            .find(|s| s.call_id() == call_id)
            .map(|s| s.id().clone());

        if let Some(id) = session_id {
            sessions.remove(&id);
            info!("Session terminated: {:?}", id);
        }

        // Send 200 OK
        let response = Response::new(StatusCode::OK).with_header("Call-ID", call_id);

        Ok(Some(Message::Response(response)))
    }

    /// Handle OPTIONS request - capability query
    async fn handle_options(&self, request: Request) -> Result<Option<Message>> {
        info!("Handling OPTIONS request");

        let call_id = request
            .get_header_value("Call-ID")
            .unwrap_or("none")
            .to_string();

        // Send 200 OK with capabilities
        let response = Response::new(StatusCode::OK)
            .with_header("Call-ID", call_id.as_str())
            .with_header("Allow", "INVITE, ACK, BYE, CANCEL, OPTIONS, INFO")
            .with_header("Accept", "application/sdp")
            .with_header("Supported", "replaces, timer");

        Ok(Some(Message::Response(response)))
    }

    /// Handle ACK request
    async fn handle_ack(&self, _request: Request) -> Result<Option<Message>> {
        debug!("Handling ACK request");
        // ACK doesn't require a response
        Ok(None)
    }

    /// Handle CANCEL request
    async fn handle_cancel(&self, request: Request) -> Result<Option<Message>> {
        let call_id = request
            .get_header_value("Call-ID")
            .ok_or_else(|| anyhow::anyhow!("No Call-ID in CANCEL"))?;

        info!("Canceling session for Call-ID: {}", call_id);

        let response = Response::new(StatusCode::OK).with_header("Call-ID", call_id);

        Ok(Some(Message::Response(response)))
    }

    /// Get session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }
}

impl Default for B2BUA {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sip::Uri;

    #[tokio::test]
    async fn test_b2bua_options() {
        let b2bua = B2BUA::new();

        let request = Request::new(
            Method::Options,
            Uri::new("sip".to_string(), "example.com".to_string()),
        )
        .with_header("Call-ID", "test123");

        let result = b2bua.handle_message(Message::Request(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.is_some());

        if let Some(Message::Response(res)) = response {
            assert_eq!(res.status_code, StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_b2bua_invite_bye() {
        let b2bua = B2BUA::new();

        // Send INVITE
        let invite = Request::new(
            Method::Invite,
            Uri::new("sip".to_string(), "bob@example.com".to_string()),
        )
        .with_header("Call-ID", "call123");

        let result = b2bua.handle_message(Message::Request(invite)).await;
        assert!(result.is_ok());
        assert_eq!(b2bua.session_count().await, 1);

        // Send BYE
        let bye = Request::new(
            Method::Bye,
            Uri::new("sip".to_string(), "bob@example.com".to_string()),
        )
        .with_header("Call-ID", "call123");

        let result = b2bua.handle_message(Message::Request(bye)).await;
        assert!(result.is_ok());
        assert_eq!(b2bua.session_count().await, 0);
    }
}
