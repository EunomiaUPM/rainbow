/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use urn::Urn;
use uuid::Uuid;

/// Validated dot-separated topic path identifying an event category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Topic(String);

impl Topic {
    /// Create a new topic from a string slice.
    pub fn new(topic: impl Into<String>) -> Result<Self, String> {
        let s = topic.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("topic cannot be empty".to_string());
        }
        if trimmed.contains('*') || trimmed.contains('?') {
            return Err("topic cannot contain wildcard characters".to_string());
        }
        let segments: Vec<&str> = trimmed.split('.').collect();
        if segments.iter().any(|seg| seg.trim().is_empty()) {
            return Err("topic segments cannot be empty".to_string());
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Access the underlying string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return the individual dot-separated segments.
    pub fn segments(&self) -> Vec<&str> {
        self.0.split('.').collect()
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Topic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Pattern with wildcard support for topic subscriptions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TopicPattern(String);

impl TopicPattern {
    /// Create a new topic pattern.
    pub fn new(pattern: impl Into<String>) -> Result<Self, String> {
        let s = pattern.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("pattern cannot be empty".to_string());
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Return wildcard pattern that matches all topics.
    pub fn match_all() -> Self {
        Self("**".to_string())
    }

    /// Access the underlying pattern string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Test if a given topic matches this pattern.
    pub fn matches(&self, topic: &Topic) -> bool {
        if self.0 == "*" || self.0 == "**" || self.0 == "*.*.*" {
            return true;
        }

        let pat_segments: Vec<&str> = self.0.split('.').collect();
        let topic_segments = topic.segments();

        Self::match_segments(&pat_segments, &topic_segments)
    }

    fn match_segments(pat: &[&str], topic: &[&str]) -> bool {
        if pat.is_empty() {
            return topic.is_empty();
        }

        if pat[0] == "**" {
            if pat.len() == 1 {
                return true;
            }
            for i in 0..=topic.len() {
                if Self::match_segments(&pat[1..], &topic[i..]) {
                    return true;
                }
            }
            return false;
        }

        if topic.is_empty() {
            return false;
        }

        if pat[0] == "*" || pat[0] == topic[0] {
            return Self::match_segments(&pat[1..], &topic[1..]);
        }

        false
    }
}

impl Display for TopicPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TopicPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Generic immutable envelope carrying domain events across the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Urn,
    pub topic: Topic,
    pub source_crate: String,
    pub schema_version: u32,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Urn>,
    pub payload: serde_json::Value,
}

impl EventEnvelope {
    /// Construct a new event envelope with generated URN id and current timestamp.
    pub fn new(
        topic: Topic,
        source_crate: impl Into<String>,
        schema_version: u32,
        correlation_id: Option<Urn>,
        payload: serde_json::Value,
    ) -> Self {
        let id_str = format!("urn:uuid:{}", Uuid::new_v4());
        let id = Urn::from_str(&id_str).expect("valid URN format");
        Self {
            id,
            topic,
            source_crate: source_crate.into(),
            schema_version,
            timestamp: Utc::now(),
            correlation_id,
            payload,
        }
    }

    /// Construct envelope with an explicit URN identifier and timestamp.
    pub fn with_metadata(
        id: Urn,
        topic: Topic,
        source_crate: impl Into<String>,
        schema_version: u32,
        timestamp: DateTime<Utc>,
        correlation_id: Option<Urn>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id,
            topic,
            source_crate: source_crate.into(),
            schema_version,
            timestamp,
            correlation_id,
            payload,
        }
    }
}
