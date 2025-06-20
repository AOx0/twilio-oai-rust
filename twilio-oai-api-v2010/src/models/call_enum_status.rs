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

/// CallEnumStatus : The status of this call. Can be: `queued`, `ringing`, `in-progress`, `canceled`, `completed`, `failed`, `busy` or `no-answer`. See [Call Status Values](https://www.twilio.com/docs/voice/api/call-resource#call-status-values) below for more information.
/// The status of this call. Can be: `queued`, `ringing`, `in-progress`, `canceled`, `completed`, `failed`, `busy` or `no-answer`. See [Call Status Values](https://www.twilio.com/docs/voice/api/call-resource#call-status-values) below for more information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallEnumStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "no-answer")]
    NoAnswer,
    #[serde(rename = "canceled")]
    Canceled,
}

impl std::fmt::Display for CallEnumStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Ringing => write!(f, "ringing"),
            Self::InProgress => write!(f, "in-progress"),
            Self::Completed => write!(f, "completed"),
            Self::Busy => write!(f, "busy"),
            Self::Failed => write!(f, "failed"),
            Self::NoAnswer => write!(f, "no-answer"),
            Self::Canceled => write!(f, "canceled"),
        }
    }
}

impl Default for CallEnumStatus {
    fn default() -> CallEnumStatus {
        Self::Queued
    }
}
