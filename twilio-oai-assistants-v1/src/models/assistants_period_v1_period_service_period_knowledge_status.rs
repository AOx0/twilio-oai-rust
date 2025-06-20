/*
 * Twilio - Assistants
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
pub struct AssistantsPeriodV1PeriodServicePeriodKnowledgeStatus {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Knowledge resource.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The status of processing the knowledge source ('QUEUED', 'PROCESSING', 'COMPLETED', 'FAILED')
    #[serde(rename = "status")]
    pub status: String,
    /// The last status of processing the knowledge source ('QUEUED', 'PROCESSING', 'COMPLETED', 'FAILED')
    #[serde(rename = "last_status", skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// The date and time in GMT when the Knowledge was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
}

impl AssistantsPeriodV1PeriodServicePeriodKnowledgeStatus {
    pub fn new(status: String) -> AssistantsPeriodV1PeriodServicePeriodKnowledgeStatus {
        AssistantsPeriodV1PeriodServicePeriodKnowledgeStatus {
            account_sid: None,
            status,
            last_status: None,
            date_updated: None,
        }
    }
}

