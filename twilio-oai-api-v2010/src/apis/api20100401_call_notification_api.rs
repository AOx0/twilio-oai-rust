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

/// struct for passing parameters to the method [`fetch_call_notification`]
#[derive(Clone, Debug)]
pub struct FetchCallNotificationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resource to fetch.
    pub account_sid: String,
    /// The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the Call Notification resource to fetch.
    pub call_sid: String,
    /// The Twilio-provided string that uniquely identifies the Call Notification resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_call_notification`]
#[derive(Clone, Debug)]
pub struct ListCallNotificationParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resources to read.
    pub account_sid: String,
    /// The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the Call Notification resources to read.
    pub call_sid: String,
    /// Only read notifications of the specified log level. Can be:  `0` to read only ERROR notifications or `1` to read only WARNING notifications. By default, all notifications are read.
    pub log: Option<i32>,
    /// Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date.
    pub message_date: Option<String>,
    /// Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date.
    pub message_date_less_than: Option<String>,
    /// Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date.
    pub message_date_greater_than: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i64>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for typed successes of method [`fetch_call_notification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchCallNotificationSuccess {
    Status200(models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotificationInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_call_notification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCallNotificationSuccess {
    Status200(models::ListCallNotificationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_call_notification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchCallNotificationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_call_notification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCallNotificationError {
    UnknownValue(serde_json::Value),
}

///
pub async fn fetch_call_notification(
    configuration: &configuration::Configuration,
    params: FetchCallNotificationParams,
) -> Result<ResponseContent<FetchCallNotificationSuccess>, Error<FetchCallNotificationError>> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications/{Sid}.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
        CallSid = crate::apis::urlencode(params.call_sid),
        Sid = crate::apis::urlencode(params.sid)
    );
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
        let entity: Option<FetchCallNotificationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchCallNotificationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

///
pub async fn list_call_notification(
    configuration: &configuration::Configuration,
    params: ListCallNotificationParams,
) -> Result<ResponseContent<ListCallNotificationSuccess>, Error<ListCallNotificationError>> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
        CallSid = crate::apis::urlencode(params.call_sid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.log {
        req_builder = req_builder.query(&[("Log", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.message_date {
        req_builder = req_builder.query(&[("MessageDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.message_date_less_than {
        req_builder = req_builder.query(&[("MessageDate<", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.message_date_greater_than {
        req_builder = req_builder.query(&[("MessageDate>", &param_value.to_string())]);
    }
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
        let entity: Option<ListCallNotificationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListCallNotificationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
