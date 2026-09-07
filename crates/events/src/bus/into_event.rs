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

use urn::Urn;

use crate::bus::envelope::{EventEnvelope, Topic};

/// Trait implemented by domain event types to be packaged into an [`EventEnvelope`].
pub trait IntoEvent: Sized {
    /// Return the topic identifier for this event.
    fn topic() -> Topic;

    /// Return the schema version for payload evolution (defaults to 1).
    fn schema_version() -> u32 {
        1
    }

    /// Return optional correlation ID for distributed tracing.
    fn correlation_id(&self) -> Option<Urn> {
        None
    }

    /// Convert the instance into an envelope with serialized payload.
    fn into_envelope(self) -> EventEnvelope;
}

/// Helper macro to generate [`IntoEvent`] implementations for serializable domain events.
#[macro_export]
macro_rules! impl_into_event {
    ($type:ty, $topic:expr, $source_crate:expr) => {
        $crate::impl_into_event!($type, $topic, $source_crate, 1);
    };
    ($type:ty, $topic:expr, $source_crate:expr, $version:expr) => {
        impl $crate::bus::into_event::IntoEvent for $type {
            fn topic() -> $crate::bus::envelope::Topic {
                $crate::bus::envelope::Topic::new($topic).expect("valid static topic")
            }

            fn schema_version() -> u32 {
                $version
            }

            fn into_envelope(self) -> $crate::bus::envelope::EventEnvelope {
                $crate::bus::envelope::EventEnvelope::new(
                    Self::topic(),
                    $source_crate,
                    Self::schema_version(),
                    self.correlation_id(),
                    serde_json::to_value(&self).expect("serializable event payload"),
                )
            }
        }
    };
}
