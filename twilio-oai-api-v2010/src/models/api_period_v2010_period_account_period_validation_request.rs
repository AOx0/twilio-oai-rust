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
pub struct ApiPeriodV2010PeriodAccountPeriodValidationRequest {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for the Caller ID.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Caller ID is associated with.
    #[serde(
        rename = "call_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub call_sid: Option<Option<String>>,
    /// The string that you assigned to describe the resource.
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    /// The phone number to verify in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, which consists of a + followed by the country code and subscriber number.
    #[serde(
        rename = "phone_number",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<Option<String>>,
    /// The 6 digit validation code that someone must enter to validate the Caller ID  when `phone_number` is called.
    #[serde(
        rename = "validation_code",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub validation_code: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodValidationRequest {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodValidationRequest {
        ApiPeriodV2010PeriodAccountPeriodValidationRequest {
            account_sid: None,
            call_sid: None,
            friendly_name: None,
            phone_number: None,
            validation_code: None,
        }
    }
}
