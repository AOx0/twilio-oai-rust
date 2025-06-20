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
pub enum SmsMessageEnumStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "received")]
    Received,
}

impl std::fmt::Display for SmsMessageEnumStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Sending => write!(f, "sending"),
            Self::Sent => write!(f, "sent"),
            Self::Failed => write!(f, "failed"),
            Self::Received => write!(f, "received"),
        }
    }
}

impl Default for SmsMessageEnumStatus {
    fn default() -> SmsMessageEnumStatus {
        Self::Queued
    }
}
