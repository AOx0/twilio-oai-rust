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
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec {
    /// The SID of the Siprec resource.
    #[serde(
        rename = "sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sid: Option<Option<String>>,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Siprec resource.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Siprec resource is associated with.
    #[serde(
        rename = "call_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub call_sid: Option<Option<String>>,
    /// The user-specified name of this Siprec, if one was given when the Siprec was created. This may be used to stop the Siprec.
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::SiprecEnumStatus>,
    /// The date and time in GMT that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
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

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec {
            sid: None,
            account_sid: None,
            call_sid: None,
            name: None,
            status: None,
            date_updated: None,
            uri: None,
        }
    }
}
