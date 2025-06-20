/*
 * Twilio - Accounts
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCredentialPublicKeyResponse {
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<models::AccountsPeriodV1PeriodCredentialPeriodCredentialPublicKey>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::ListCredentialAwsResponseMeta>>,
}

impl ListCredentialPublicKeyResponse {
    pub fn new() -> ListCredentialPublicKeyResponse {
        ListCredentialPublicKeyResponse {
            credentials: None,
            meta: None,
        }
    }
}
