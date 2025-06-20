/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageEnumTrafficType {
    #[serde(rename = "free")]
    Free,
}

impl std::fmt::Display for MessageEnumTrafficType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "free"),
        }
    }
}

impl Default for MessageEnumTrafficType {
    fn default() -> MessageEnumTrafficType {
        Self::Free
    }
}
