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
pub struct ApiPeriodV2010PeriodAccountPeriodConnectApp {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resource.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// The URL we redirect the user to after we authenticate the user and obtain authorization to access the Connect App.
    #[serde(
        rename = "authorize_redirect_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorize_redirect_url: Option<Option<String>>,
    /// The company name set for the Connect App.
    #[serde(
        rename = "company_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_name: Option<Option<String>>,
    /// The HTTP method we use to call `deauthorize_callback_url`.
    #[serde(
        rename = "deauthorize_callback_method",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deauthorize_callback_method: Option<Option<DeauthorizeCallbackMethod>>,
    /// The URL we call using the `deauthorize_callback_method` to de-authorize the Connect App.
    #[serde(
        rename = "deauthorize_callback_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deauthorize_callback_url: Option<Option<String>>,
    /// The description of the Connect App.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// The string that you assigned to describe the resource.
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    /// The public URL where users can obtain more information about this Connect App.
    #[serde(
        rename = "homepage_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub homepage_url: Option<Option<String>>,
    /// The set of permissions that your ConnectApp requests.
    #[serde(
        rename = "permissions",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions: Option<Option<Vec<models::ConnectAppEnumPermission>>>,
    /// The unique string that that we created to identify the ConnectApp resource.
    #[serde(
        rename = "sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sid: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(
        rename = "uri",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodConnectApp {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodConnectApp {
        ApiPeriodV2010PeriodAccountPeriodConnectApp {
            account_sid: None,
            authorize_redirect_url: None,
            company_name: None,
            deauthorize_callback_method: None,
            deauthorize_callback_url: None,
            description: None,
            friendly_name: None,
            homepage_url: None,
            permissions: None,
            sid: None,
            uri: None,
        }
    }
}
/// The HTTP method we use to call `deauthorize_callback_url`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeauthorizeCallbackMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}

impl Default for DeauthorizeCallbackMethod {
    fn default() -> DeauthorizeCallbackMethod {
        Self::Get
    }
}
