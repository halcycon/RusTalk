//! Voicemail system implementation
//!
//! Provides voicemail box management, recording storage, and message retrieval
//! similar to FreeSWITCH voicemail functionality.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Voicemail box configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicemailBox {
    /// Mailbox ID (typically extension number)
    pub id: String,
    /// Extension number
    pub extension: String,
    /// User's name
    pub name: String,
    /// PIN for accessing voicemail
    pub pin: String,
    /// Email for notifications
    pub email: Option<String>,
    /// Whether to attach recordings to email
    pub email_attach: bool,
    /// Maximum number of messages
    pub max_messages: usize,
    /// Maximum message length in seconds
    pub max_message_length: u32,
    /// Greeting type
    pub greeting: VoicemailGreeting,
    /// Whether the mailbox is enabled
    pub enabled: bool,
}

/// Voicemail greeting types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoicemailGreeting {
    /// Default system greeting
    Default,
    /// Custom recorded greeting
    Custom(String), // Path to custom greeting file
    /// Name-only greeting
    Name,
    /// Unavailable greeting
    Unavailable,
}

/// A voicemail message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicemailMessage {
    /// Unique message ID
    pub id: String,
    /// Mailbox ID
    pub mailbox_id: String,
    /// Caller number
    pub from_number: String,
    /// Caller name (if available)
    pub from_name: Option<String>,
    /// Timestamp when message was left
    pub timestamp: DateTime<Utc>,
    /// Message duration in seconds
    pub duration: u32,
    /// Path to audio file
    pub file_path: String,
    /// Whether message has been listened to
    pub read: bool,
    /// Whether message is marked as urgent
    pub urgent: bool,
}

/// Message Waiting Indicator status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MwiStatus {
    /// Mailbox ID
    pub mailbox_id: String,
    /// Number of new (unread) messages
    pub new_messages: usize,
    /// Number of old (read) messages
    pub old_messages: usize,
    /// Total messages
    pub total_messages: usize,
}

/// Voicemail system manager
#[derive(Debug, Clone)]
pub struct VoicemailManager {
    /// Base directory for voicemail storage
    base_dir: PathBuf,
    /// Map of mailbox configurations
    mailboxes: Vec<VoicemailBox>,
    /// Map of messages by mailbox
    messages: Vec<VoicemailMessage>,
}

impl VoicemailManager {
    /// Create a new voicemail manager
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            mailboxes: Vec::new(),
            messages: Vec::new(),
        }
    }

    /// Add a mailbox
    pub fn add_mailbox(&mut self, mailbox: VoicemailBox) -> Result<()> {
        // Check if mailbox already exists
        if self.mailboxes.iter().any(|m| m.id == mailbox.id) {
            anyhow::bail!("Mailbox already exists: {}", mailbox.id);
        }

        // Create mailbox directory
        let mailbox_dir = self.mailbox_dir(&mailbox.id);
        std::fs::create_dir_all(&mailbox_dir).context("Failed to create mailbox directory")?;

        self.mailboxes.push(mailbox);
        Ok(())
    }

    /// Get a mailbox by ID
    pub fn get_mailbox(&self, mailbox_id: &str) -> Option<&VoicemailBox> {
        self.mailboxes.iter().find(|m| m.id == mailbox_id)
    }

    /// Remove a mailbox
    pub fn remove_mailbox(&mut self, mailbox_id: &str) -> Result<bool> {
        let pos = self.mailboxes.iter().position(|m| m.id == mailbox_id);

        if let Some(pos) = pos {
            self.mailboxes.remove(pos);

            // Remove all messages for this mailbox
            self.messages.retain(|m| m.mailbox_id != mailbox_id);

            // Remove mailbox directory
            let mailbox_dir = self.mailbox_dir(mailbox_id);
            if mailbox_dir.exists() {
                std::fs::remove_dir_all(&mailbox_dir)
                    .context("Failed to remove mailbox directory")?;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Leave a voicemail message
    pub fn leave_message(
        &mut self,
        mailbox_id: &str,
        from_number: &str,
        from_name: Option<String>,
        audio_data: &[u8],
        duration: u32,
    ) -> Result<String> {
        let mailbox = self
            .get_mailbox(mailbox_id)
            .context(format!("Mailbox not found: {}", mailbox_id))?;

        if !mailbox.enabled {
            anyhow::bail!("Mailbox is disabled");
        }

        // Check message limits
        let message_count = self
            .messages
            .iter()
            .filter(|m| m.mailbox_id == mailbox_id)
            .count();

        if message_count >= mailbox.max_messages {
            anyhow::bail!("Mailbox is full");
        }

        if duration > mailbox.max_message_length {
            anyhow::bail!("Message too long");
        }

        // Generate message ID
        let message_id = uuid::Uuid::new_v4().to_string();

        // Save audio file
        let file_path = self.message_file_path(mailbox_id, &message_id);
        std::fs::write(&file_path, audio_data).context("Failed to write audio file")?;

        // Create message record
        let message = VoicemailMessage {
            id: message_id.clone(),
            mailbox_id: mailbox_id.to_string(),
            from_number: from_number.to_string(),
            from_name,
            timestamp: Utc::now(),
            duration,
            file_path: file_path.to_string_lossy().to_string(),
            read: false,
            urgent: false,
        };

        self.messages.push(message);

        Ok(message_id)
    }

    /// Get messages for a mailbox
    pub fn get_messages(&self, mailbox_id: &str, include_read: bool) -> Vec<&VoicemailMessage> {
        self.messages
            .iter()
            .filter(|m| m.mailbox_id == mailbox_id && (include_read || !m.read))
            .collect()
    }

    /// Mark a message as read
    pub fn mark_message_read(&mut self, message_id: &str) -> Result<()> {
        let message = self
            .messages
            .iter_mut()
            .find(|m| m.id == message_id)
            .context("Message not found")?;

        message.read = true;
        Ok(())
    }

    /// Delete a message
    pub fn delete_message(&mut self, message_id: &str) -> Result<()> {
        let pos = self
            .messages
            .iter()
            .position(|m| m.id == message_id)
            .context("Message not found")?;

        let message = &self.messages[pos];

        // Delete audio file
        let file_path = Path::new(&message.file_path);
        if file_path.exists() {
            std::fs::remove_file(file_path).context("Failed to delete audio file")?;
        }

        self.messages.remove(pos);
        Ok(())
    }

    /// Get MWI status for a mailbox
    pub fn get_mwi_status(&self, mailbox_id: &str) -> MwiStatus {
        let messages: Vec<_> = self
            .messages
            .iter()
            .filter(|m| m.mailbox_id == mailbox_id)
            .collect();

        let new_messages = messages.iter().filter(|m| !m.read).count();
        let old_messages = messages.iter().filter(|m| m.read).count();

        MwiStatus {
            mailbox_id: mailbox_id.to_string(),
            new_messages,
            old_messages,
            total_messages: messages.len(),
        }
    }

    /// Verify PIN for mailbox access
    pub fn verify_pin(&self, mailbox_id: &str, pin: &str) -> bool {
        if let Some(mailbox) = self.get_mailbox(mailbox_id) {
            mailbox.pin == pin
        } else {
            false
        }
    }

    /// Get mailbox directory path
    fn mailbox_dir(&self, mailbox_id: &str) -> PathBuf {
        self.base_dir.join(mailbox_id)
    }

    /// Get message file path
    fn message_file_path(&self, mailbox_id: &str, message_id: &str) -> PathBuf {
        self.mailbox_dir(mailbox_id)
            .join(format!("{}.wav", message_id))
    }

    /// List all mailboxes
    pub fn list_mailboxes(&self) -> Vec<&VoicemailBox> {
        self.mailboxes.iter().collect()
    }
}

impl Default for VoicemailBox {
    fn default() -> Self {
        Self {
            id: String::new(),
            extension: String::new(),
            name: String::new(),
            pin: "0000".to_string(),
            email: None,
            email_attach: false,
            max_messages: 100,
            max_message_length: 300, // 5 minutes
            greeting: VoicemailGreeting::Default,
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mailbox() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };

        assert!(manager.add_mailbox(mailbox.clone()).is_ok());
        assert!(manager.get_mailbox("1001").is_some());
    }

    #[test]
    fn test_leave_message() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };

        manager.add_mailbox(mailbox).unwrap();

        let audio_data = b"fake audio data";
        let result =
            manager.leave_message("1001", "5551234", Some("Bob".to_string()), audio_data, 10);

        assert!(result.is_ok());

        let messages = manager.get_messages("1001", false);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].from_number, "5551234");
    }

    #[test]
    fn test_mwi_status() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };

        manager.add_mailbox(mailbox).unwrap();

        // Leave two messages
        let audio_data = b"fake audio data";
        manager
            .leave_message("1001", "5551234", None, audio_data, 10)
            .unwrap();
        manager
            .leave_message("1001", "5555678", None, audio_data, 15)
            .unwrap();

        let status = manager.get_mwi_status("1001");
        assert_eq!(status.new_messages, 2);
        assert_eq!(status.old_messages, 0);
        assert_eq!(status.total_messages, 2);
    }

    #[test]
    fn test_mark_message_read() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };

        manager.add_mailbox(mailbox).unwrap();

        let audio_data = b"fake audio data";
        let message_id = manager
            .leave_message("1001", "5551234", None, audio_data, 10)
            .unwrap();

        manager.mark_message_read(&message_id).unwrap();

        let status = manager.get_mwi_status("1001");
        assert_eq!(status.new_messages, 0);
        assert_eq!(status.old_messages, 1);
    }

    #[test]
    fn test_delete_message() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };

        manager.add_mailbox(mailbox).unwrap();

        let audio_data = b"fake audio data";
        let message_id = manager
            .leave_message("1001", "5551234", None, audio_data, 10)
            .unwrap();

        manager.delete_message(&message_id).unwrap();

        let messages = manager.get_messages("1001", true);
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_verify_pin() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };

        manager.add_mailbox(mailbox).unwrap();

        assert!(manager.verify_pin("1001", "1234"));
        assert!(!manager.verify_pin("1001", "0000"));
    }

    #[test]
    fn test_mailbox_full() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");

        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            max_messages: 2,
            ..Default::default()
        };

        manager.add_mailbox(mailbox).unwrap();

        let audio_data = b"fake audio data";
        manager
            .leave_message("1001", "5551234", None, audio_data, 10)
            .unwrap();
        manager
            .leave_message("1001", "5555678", None, audio_data, 10)
            .unwrap();

        // Third message should fail
        let result = manager.leave_message("1001", "5559999", None, audio_data, 10);
        assert!(result.is_err());
    }
}
