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

/// AccountEnumType : The type of this account. Either `Trial` or `Full` if it's been upgraded
/// The type of this account. Either `Trial` or `Full` if it's been upgraded
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountEnumType {
    #[serde(rename = "Trial")]
    Trial,
    #[serde(rename = "Full")]
    Full,
}

impl std::fmt::Display for AccountEnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Trial => write!(f, "Trial"),
            Self::Full => write!(f, "Full"),
        }
    }
}

impl Default for AccountEnumType {
    fn default() -> AccountEnumType {
        Self::Trial
    }
}
