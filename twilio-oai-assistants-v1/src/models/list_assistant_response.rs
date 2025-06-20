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
pub struct ListAssistantResponse {
    #[serde(rename = "assistants", skip_serializing_if = "Option::is_none")]
    pub assistants: Option<Vec<models::AssistantsPeriodV1PeriodServicePeriodAssistant>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::ListAssistantResponseMeta>>,
}

impl ListAssistantResponse {
    pub fn new() -> ListAssistantResponse {
        ListAssistantResponse {
            assistants: None,
            meta: None,
        }
    }
}

