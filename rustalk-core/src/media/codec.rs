//! Codec definitions and management

use serde::{Deserialize, Serialize};
use std::fmt;

/// Audio codec with RTP payload type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Codec {
    /// Codec name
    pub name: String,
    /// RTP payload type (0-127)
    pub payload_type: u8,
    /// Clock rate in Hz
    pub clock_rate: u32,
    /// Number of audio channels (typically 1)
    pub channels: u8,
    /// Codec description
    pub description: String,
    /// Whether this codec is currently enabled
    pub enabled: bool,
    /// Whether this is a standard codec (cannot be uninstalled)
    pub is_standard: bool,
    /// Priority/order for codec negotiation (lower number = higher priority)
    /// If None, uses position in the list
    pub priority: Option<u32>,
}

impl Codec {
    /// Create a new codec
    pub fn new(
        name: &str,
        payload_type: u8,
        clock_rate: u32,
        channels: u8,
        description: &str,
        is_standard: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            payload_type,
            clock_rate,
            channels,
            description: description.to_string(),
            enabled: true,
            is_standard,
            priority: None,
        }
    }

    /// Get the SDP format attribute (a=rtpmap) line
    pub fn rtpmap(&self) -> String {
        format!(
            "{} {}/{}{}",
            self.payload_type,
            self.name,
            self.clock_rate,
            if self.channels > 1 {
                format!("/{}", self.channels)
            } else {
                String::new()
            }
        )
    }

    /// Get the SDP media format (payload type as string)
    pub fn media_format(&self) -> String {
        self.payload_type.to_string()
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (PT: {}, {}Hz, {}ch)",
            self.name, self.payload_type, self.clock_rate, self.channels
        )
    }
}

/// Standard audio codecs with their RTP payload types
pub fn standard_codecs() -> Vec<Codec> {
    vec![
        // Static payload types (0-95)
        Codec::new("PCMU", 0, 8000, 1, "G.711 μ-law (64 kbps)", true),
        Codec::new("PCMA", 8, 8000, 1, "G.711 A-law (64 kbps)", true),
        Codec::new("G722", 9, 8000, 1, "G.722 wideband (64 kbps)", true),
        Codec::new("GSM", 3, 8000, 1, "GSM Full Rate (13 kbps)", true),
        // Dynamic payload types (96-127)
        Codec::new("G729", 18, 8000, 1, "G.729 (8 kbps)", true),
        Codec::new("iLBC", 97, 8000, 1, "Internet Low Bitrate Codec (13.33/15.2 kbps)", true),
        Codec::new("opus", 111, 48000, 2, "Opus codec (6-510 kbps)", true),
        Codec::new("AMR", 96, 8000, 1, "Adaptive Multi-Rate narrowband (4.75-12.2 kbps)", true),
        Codec::new("AMR-WB", 98, 16000, 1, "Adaptive Multi-Rate wideband (6.6-23.85 kbps)", true),
        Codec::new("SILK", 99, 16000, 1, "Skype SILK codec (6-40 kbps)", true),
    ]
}

/// Codec configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecConfig {
    /// List of available codecs
    pub codecs: Vec<Codec>,
}

impl CodecConfig {
    /// Create a new codec configuration with standard codecs
    pub fn new() -> Self {
        Self {
            codecs: standard_codecs(),
        }
    }

    /// Get enabled codecs
    pub fn enabled_codecs(&self) -> Vec<&Codec> {
        self.codecs.iter().filter(|c| c.enabled).collect()
    }

    /// Get codec by payload type
    pub fn get_by_payload_type(&self, pt: u8) -> Option<&Codec> {
        self.codecs.iter().find(|c| c.payload_type == pt)
    }

    /// Get codec by name
    pub fn get_by_name(&self, name: &str) -> Option<&Codec> {
        self.codecs
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Enable a codec by name
    pub fn enable_codec(&mut self, name: &str) -> bool {
        if let Some(codec) = self.codecs.iter_mut().find(|c| c.name.eq_ignore_ascii_case(name)) {
            codec.enabled = true;
            true
        } else {
            false
        }
    }

    /// Disable a codec by name
    pub fn disable_codec(&mut self, name: &str) -> bool {
        if let Some(codec) = self.codecs.iter_mut().find(|c| c.name.eq_ignore_ascii_case(name)) {
            codec.enabled = false;
            true
        } else {
            false
        }
    }

    /// Add a custom codec (non-standard)
    pub fn add_codec(&mut self, mut codec: Codec) -> Result<(), String> {
        // Check if codec with same payload type already exists
        if self.codecs.iter().any(|c| c.payload_type == codec.payload_type) {
            return Err(format!("Codec with payload type {} already exists", codec.payload_type));
        }

        // Custom codecs are not standard
        codec.is_standard = false;
        self.codecs.push(codec);
        Ok(())
    }

    /// Remove a codec (only non-standard codecs can be removed)
    pub fn remove_codec(&mut self, name: &str) -> Result<(), String> {
        if let Some(pos) = self.codecs.iter().position(|c| c.name.eq_ignore_ascii_case(name)) {
            if self.codecs[pos].is_standard {
                return Err("Cannot remove standard codecs".to_string());
            }
            self.codecs.remove(pos);
            Ok(())
        } else {
            Err(format!("Codec '{}' not found", name))
        }
    }

    /// Get SDP format line for all enabled codecs
    pub fn sdp_formats(&self) -> String {
        self.enabled_codecs()
            .iter()
            .map(|c| c.media_format())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Get SDP rtpmap attributes for all enabled codecs
    pub fn sdp_rtpmaps(&self) -> Vec<String> {
        self.enabled_codecs()
            .iter()
            .map(|c| format!("a=rtpmap:{}", c.rtpmap()))
            .collect()
    }

    /// Reorder codecs by moving a codec to a new position
    pub fn reorder_codec(&mut self, from_index: usize, to_index: usize) -> Result<(), String> {
        if from_index >= self.codecs.len() {
            return Err(format!("Invalid from_index: {}", from_index));
        }
        if to_index >= self.codecs.len() {
            return Err(format!("Invalid to_index: {}", to_index));
        }
        
        if from_index != to_index {
            let codec = self.codecs.remove(from_index);
            self.codecs.insert(to_index, codec);
            
            // Update priorities to reflect new order
            self.update_priorities();
        }
        
        Ok(())
    }

    /// Update priorities based on current order in the list
    fn update_priorities(&mut self) {
        for (index, codec) in self.codecs.iter_mut().enumerate() {
            codec.priority = Some(index as u32);
        }
    }

    /// Sort codecs by priority (lower number = higher priority)
    /// Falls back to list order if priority is not set
    pub fn sort_by_priority(&mut self) {
        self.codecs.sort_by(|a, b| {
            match (a.priority, b.priority) {
                (Some(pa), Some(pb)) => pa.cmp(&pb),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }

    /// Get codecs in priority order (enabled codecs first, sorted by priority)
    pub fn get_priority_ordered(&self) -> Vec<&Codec> {
        let mut enabled: Vec<&Codec> = self.enabled_codecs();
        enabled.sort_by(|a, b| {
            match (a.priority, b.priority) {
                (Some(pa), Some(pb)) => pa.cmp(&pb),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
        enabled
    }
}

impl Default for CodecConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_codecs() {
        let codecs = standard_codecs();
        assert!(!codecs.is_empty());
        assert!(codecs.iter().any(|c| c.name == "PCMU"));
        assert!(codecs.iter().any(|c| c.name == "PCMA"));
        assert!(codecs.iter().any(|c| c.name == "opus"));
    }

    #[test]
    fn test_codec_rtpmap() {
        let codec = Codec::new("PCMU", 0, 8000, 1, "Test codec", true);
        assert_eq!(codec.rtpmap(), "0 PCMU/8000");

        let codec_stereo = Codec::new("opus", 111, 48000, 2, "Opus stereo", true);
        assert_eq!(codec_stereo.rtpmap(), "111 opus/48000/2");
    }

    #[test]
    fn test_codec_config_default() {
        let config = CodecConfig::default();
        assert!(!config.codecs.is_empty());
        assert!(config.enabled_codecs().len() > 0);
    }

    #[test]
    fn test_enable_disable_codec() {
        let mut config = CodecConfig::new();
        
        // Disable a codec
        assert!(config.disable_codec("PCMU"));
        assert!(!config.get_by_name("PCMU").unwrap().enabled);
        
        // Enable it back
        assert!(config.enable_codec("PCMU"));
        assert!(config.get_by_name("PCMU").unwrap().enabled);
        
        // Try non-existent codec
        assert!(!config.enable_codec("NonExistent"));
    }

    #[test]
    fn test_get_codec_by_payload_type() {
        let config = CodecConfig::new();
        let codec = config.get_by_payload_type(0);
        assert!(codec.is_some());
        assert_eq!(codec.unwrap().name, "PCMU");
    }

    #[test]
    fn test_get_codec_by_name() {
        let config = CodecConfig::new();
        let codec = config.get_by_name("PCMA");
        assert!(codec.is_some());
        assert_eq!(codec.unwrap().payload_type, 8);
        
        // Case insensitive
        let codec = config.get_by_name("pcma");
        assert!(codec.is_some());
    }

    #[test]
    fn test_add_custom_codec() {
        let mut config = CodecConfig::new();
        let custom = Codec::new("CustomCodec", 120, 16000, 1, "Custom test codec", false);
        
        assert!(config.add_codec(custom.clone()).is_ok());
        assert!(config.get_by_name("CustomCodec").is_some());
        
        // Try to add codec with duplicate payload type
        let duplicate = Codec::new("DuplicatePT", 120, 8000, 1, "Duplicate PT", false);
        assert!(config.add_codec(duplicate).is_err());
    }

    #[test]
    fn test_remove_codec() {
        let mut config = CodecConfig::new();
        
        // Cannot remove standard codec
        assert!(config.remove_codec("PCMU").is_err());
        
        // Add and remove custom codec
        let custom = Codec::new("CustomCodec", 120, 16000, 1, "Custom test codec", false);
        config.add_codec(custom).unwrap();
        assert!(config.remove_codec("CustomCodec").is_ok());
        assert!(config.get_by_name("CustomCodec").is_none());
    }

    #[test]
    fn test_sdp_formats() {
        let mut config = CodecConfig::new();
        
        // Disable all but PCMU and PCMA
        for codec in &config.codecs.clone() {
            if codec.name != "PCMU" && codec.name != "PCMA" {
                config.disable_codec(&codec.name);
            }
        }
        
        let formats = config.sdp_formats();
        assert!(formats.contains("0")); // PCMU
        assert!(formats.contains("8")); // PCMA
    }

    #[test]
    fn test_sdp_rtpmaps() {
        let mut config = CodecConfig::new();
        
        // Keep only PCMU enabled
        for codec in &config.codecs.clone() {
            if codec.name != "PCMU" {
                config.disable_codec(&codec.name);
            }
        }
        
        let rtpmaps = config.sdp_rtpmaps();
        assert_eq!(rtpmaps.len(), 1);
        assert!(rtpmaps[0].contains("a=rtpmap:0 PCMU/8000"));
    }

    #[test]
    fn test_reorder_codec() {
        let mut config = CodecConfig::new();
        let initial_first = config.codecs[0].name.clone();
        let initial_third = config.codecs[2].name.clone();
        
        // Move codec from index 2 to index 0
        assert!(config.reorder_codec(2, 0).is_ok());
        assert_eq!(config.codecs[0].name, initial_third);
        assert_eq!(config.codecs[1].name, initial_first);
        
        // Verify priorities are updated
        assert_eq!(config.codecs[0].priority, Some(0));
        assert_eq!(config.codecs[1].priority, Some(1));
    }

    #[test]
    fn test_reorder_codec_invalid_index() {
        let mut config = CodecConfig::new();
        let len = config.codecs.len();
        
        // Invalid from_index
        assert!(config.reorder_codec(len + 1, 0).is_err());
        
        // Invalid to_index
        assert!(config.reorder_codec(0, len + 1).is_err());
    }

    #[test]
    fn test_priority_ordering() {
        let mut config = CodecConfig::new();
        
        // Set custom priorities
        config.codecs[0].priority = Some(5);
        config.codecs[1].priority = Some(2);
        config.codecs[2].priority = Some(1);
        
        let ordered = config.get_priority_ordered();
        
        // Should be sorted by priority (lower number first)
        assert_eq!(ordered[0].priority, Some(1));
        assert_eq!(ordered[1].priority, Some(2));
        assert_eq!(ordered[2].priority, Some(5));
    }

    #[test]
    fn test_sort_by_priority() {
        let mut config = CodecConfig::new();
        
        // Set reverse priorities
        let len = config.codecs.len();
        for (i, codec) in config.codecs.iter_mut().enumerate() {
            codec.priority = Some((len - i) as u32);
        }
        
        config.sort_by_priority();
        
        // After sorting, priorities should be in ascending order
        for i in 0..config.codecs.len() - 1 {
            assert!(config.codecs[i].priority.unwrap() < config.codecs[i + 1].priority.unwrap());
        }
    }
}
