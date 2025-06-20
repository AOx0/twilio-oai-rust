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
pub struct AssistantsPeriodV1PeriodServicePeriodKnowledge {
    /// The type of knowledge source.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The description of knowledge.
    #[serde(rename = "id")]
    pub id: String,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Knowledge resource.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The details of the knowledge source based on the type.
    #[serde(rename = "knowledge_source_details", skip_serializing_if = "Option::is_none")]
    pub knowledge_source_details: Option<serde_json::Value>,
    /// The name of the knowledge source.
    #[serde(rename = "name")]
    pub name: String,
    /// The status of processing the knowledge source ('QUEUED', 'PROCESSING', 'COMPLETED', 'FAILED')
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The type of knowledge source ('Web', 'Database', 'Text', 'File')
    #[serde(rename = "type")]
    pub r#type: String,
    /// The url of the knowledge resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The embedding model to be used for the knowledge source.
    #[serde(rename = "embedding_model", skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    /// The date and time in GMT when the Knowledge was created specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "date_created")]
    pub date_created: String,
    /// The date and time in GMT when the Knowledge was last updated specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "date_updated")]
    pub date_updated: String,
}

impl AssistantsPeriodV1PeriodServicePeriodKnowledge {
    pub fn new(id: String, name: String, r#type: String, date_created: String, date_updated: String) -> AssistantsPeriodV1PeriodServicePeriodKnowledge {
        AssistantsPeriodV1PeriodServicePeriodKnowledge {
            description: None,
            id,
            account_sid: None,
            knowledge_source_details: None,
            name,
            status: None,
            r#type,
            url: None,
            embedding_model: None,
            date_created,
            date_updated,
        }
    }
}

