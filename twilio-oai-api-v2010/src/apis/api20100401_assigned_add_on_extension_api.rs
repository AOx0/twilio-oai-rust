/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for passing parameters to the method [`fetch_incoming_phone_number_assigned_add_on_extension`]
#[derive(Clone, Debug)]
pub struct FetchIncomingPhoneNumberAssignedAddOnExtensionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// The SID that uniquely identifies the assigned Add-on installation.
    pub assigned_add_on_sid: String,
    /// The Twilio-provided string that uniquely identifies the resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_incoming_phone_number_assigned_add_on_extension`]
#[derive(Clone, Debug)]
pub struct ListIncomingPhoneNumberAssignedAddOnExtensionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read.
    pub account_sid: String,
    /// The SID of the Phone Number to which the Add-on is assigned.
    pub resource_sid: String,
    /// The SID that uniquely identifies the assigned Add-on installation.
    pub assigned_add_on_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i64>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for typed successes of method [`fetch_incoming_phone_number_assigned_add_on_extension`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchIncomingPhoneNumberAssignedAddOnExtensionSuccess {
    Status200(models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOnPeriodIncomingPhoneNumberAssignedAddOnExtension),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_incoming_phone_number_assigned_add_on_extension`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncomingPhoneNumberAssignedAddOnExtensionSuccess {
    Status200(models::ListIncomingPhoneNumberAssignedAddOnExtensionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_incoming_phone_number_assigned_add_on_extension`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchIncomingPhoneNumberAssignedAddOnExtensionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_incoming_phone_number_assigned_add_on_extension`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncomingPhoneNumberAssignedAddOnExtensionError {
    UnknownValue(serde_json::Value),
}

/// Fetch an instance of an Extension for the Assigned Add-on.
pub async fn fetch_incoming_phone_number_assigned_add_on_extension(
    configuration: &configuration::Configuration,
    params: FetchIncomingPhoneNumberAssignedAddOnExtensionParams,
) -> Result<
    ResponseContent<FetchIncomingPhoneNumberAssignedAddOnExtensionSuccess>,
    Error<FetchIncomingPhoneNumberAssignedAddOnExtensionError>,
> {
    let uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions/{Sid}.json", configuration.base_path, AccountSid=crate::apis::urlencode(params.account_sid), ResourceSid=crate::apis::urlencode(params.resource_sid), AssignedAddOnSid=crate::apis::urlencode(params.assigned_add_on_sid), Sid=crate::apis::urlencode(params.sid));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<FetchIncomingPhoneNumberAssignedAddOnExtensionSuccess> =
            serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchIncomingPhoneNumberAssignedAddOnExtensionError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve a list of Extensions for the Assigned Add-on.
pub async fn list_incoming_phone_number_assigned_add_on_extension(
    configuration: &configuration::Configuration,
    params: ListIncomingPhoneNumberAssignedAddOnExtensionParams,
) -> Result<
    ResponseContent<ListIncomingPhoneNumberAssignedAddOnExtensionSuccess>,
    Error<ListIncomingPhoneNumberAssignedAddOnExtensionError>,
> {
    let uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions.json", configuration.base_path, AccountSid=crate::apis::urlencode(params.account_sid), ResourceSid=crate::apis::urlencode(params.resource_sid), AssignedAddOnSid=crate::apis::urlencode(params.assigned_add_on_sid));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.page_size {
        req_builder = req_builder.query(&[("PageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.page {
        req_builder = req_builder.query(&[("Page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.page_token {
        req_builder = req_builder.query(&[("PageToken", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListIncomingPhoneNumberAssignedAddOnExtensionSuccess> =
            serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListIncomingPhoneNumberAssignedAddOnExtensionError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
