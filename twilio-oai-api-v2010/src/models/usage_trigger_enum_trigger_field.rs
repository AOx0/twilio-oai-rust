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

/// UsageTriggerEnumTriggerField : The field in the [UsageRecord](https://www.twilio.com/docs/usage/api/usage-record) resource that fires the trigger.  Can be: `count`, `usage`, or `price`, as described in the [UsageRecords documentation](https://www.twilio.com/docs/usage/api/usage-record#usage-count-price).
/// The field in the [UsageRecord](https://www.twilio.com/docs/usage/api/usage-record) resource that fires the trigger.  Can be: `count`, `usage`, or `price`, as described in the [UsageRecords documentation](https://www.twilio.com/docs/usage/api/usage-record#usage-count-price).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageTriggerEnumTriggerField {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "usage")]
    Usage,
    #[serde(rename = "price")]
    Price,
}

impl std::fmt::Display for UsageTriggerEnumTriggerField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::Usage => write!(f, "usage"),
            Self::Price => write!(f, "price"),
        }
    }
}

impl Default for UsageTriggerEnumTriggerField {
    fn default() -> UsageTriggerEnumTriggerField {
        Self::Count
    }
}
