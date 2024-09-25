//! PubSub service module.
//!
//! This module should define a `PubSubClient` trait and at least one implementation. Currently,
//! there is none so we have a hard dependency on NATS. This should be abstracted out so that we
//! can swap services if needed.

#[allow(unused)]
pub trait PubSubClient {}
