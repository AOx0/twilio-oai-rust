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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) associated with this MessageFeedback resource.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// The SID of the Message resource associated with this MessageFeedback resource.
    #[serde(
        rename = "message_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_sid: Option<Option<String>>,
    #[serde(rename = "outcome", skip_serializing_if = "Option::is_none")]
    pub outcome: Option<models::MessageFeedbackEnumOutcome>,
    /// The date and time in GMT when this MessageFeedback resource was created, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(
        rename = "date_created",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT when this MessageFeedback resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(
        rename = "date_updated",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(
        rename = "uri",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback {
        ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback {
            account_sid: None,
            message_sid: None,
            outcome: None,
            date_created: None,
            date_updated: None,
            uri: None,
        }
    }
}
