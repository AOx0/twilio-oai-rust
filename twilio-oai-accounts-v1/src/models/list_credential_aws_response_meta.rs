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
pub struct ListCredentialAwsResponseMeta {
    #[serde(rename = "first_page_url", skip_serializing_if = "Option::is_none")]
    pub first_page_url: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(
        rename = "next_page_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_url: Option<Option<String>>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(
        rename = "previous_page_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_page_url: Option<Option<String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListCredentialAwsResponseMeta {
    pub fn new() -> ListCredentialAwsResponseMeta {
        ListCredentialAwsResponseMeta {
            first_page_url: None,
            key: None,
            next_page_url: None,
            page: None,
            page_size: None,
            previous_page_url: None,
            url: None,
        }
    }
}
