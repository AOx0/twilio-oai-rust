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
pub struct ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress {
    /// A 34 character string that uniquely identifies this resource.
    #[serde(
        rename = "sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sid: Option<Option<String>>,
    /// The unique id of the Account that is responsible for this resource.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// A human readable descriptive text for this resource, up to 255 characters long.
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    /// An IP address in dotted decimal notation from which you want to accept traffic. Any SIP requests from this IP address will be allowed by Twilio. IPv4 only supported today.
    #[serde(
        rename = "ip_address",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address: Option<Option<String>>,
    /// An integer representing the length of the CIDR prefix to use with this IP address when accepting traffic. By default the entire IP address is used.
    #[serde(rename = "cidr_prefix_length", skip_serializing_if = "Option::is_none")]
    pub cidr_prefix_length: Option<i32>,
    /// The unique id of the IpAccessControlList resource that includes this resource.
    #[serde(
        rename = "ip_access_control_list_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_access_control_list_sid: Option<Option<String>>,
    /// The date that this resource was created, given as GMT in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(
        rename = "date_created",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<Option<String>>,
    /// The date that this resource was last updated, given as GMT in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(
        rename = "date_updated",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<Option<String>>,
    /// The URI for this resource, relative to `https://api.twilio.com`
    #[serde(
        rename = "uri",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress {
    pub fn new(
    ) -> ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress {
        ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress {
            sid: None,
            account_sid: None,
            friendly_name: None,
            ip_address: None,
            cidr_prefix_length: None,
            ip_access_control_list_sid: None,
            date_created: None,
            date_updated: None,
            uri: None,
        }
    }
}
