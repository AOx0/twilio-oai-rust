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

/// struct for passing parameters to the method [`delete_transcription`]
#[derive(Clone, Debug)]
pub struct DeleteTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_transcription`]
#[derive(Clone, Debug)]
pub struct FetchTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch.
    pub account_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_transcription`]
#[derive(Clone, Debug)]
pub struct ListTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read.
    pub account_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i64>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for typed successes of method [`delete_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTranscriptionSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTranscriptionSuccess {
    Status200(models::ApiPeriodV2010PeriodAccountPeriodTranscription),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTranscriptionSuccess {
    Status200(models::ListTranscriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// Delete a transcription from the account used to make the request
pub async fn delete_transcription(
    configuration: &configuration::Configuration,
    params: DeleteTranscriptionParams,
) -> Result<ResponseContent<DeleteTranscriptionSuccess>, Error<DeleteTranscriptionError>> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
        Sid = crate::apis::urlencode(params.sid)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<DeleteTranscriptionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteTranscriptionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fetch an instance of a Transcription
pub async fn fetch_transcription(
    configuration: &configuration::Configuration,
    params: FetchTranscriptionParams,
) -> Result<ResponseContent<FetchTranscriptionSuccess>, Error<FetchTranscriptionError>> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
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
        let entity: Option<FetchTranscriptionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchTranscriptionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve a list of transcriptions belonging to the account used to make the request
pub async fn list_transcription(
    configuration: &configuration::Configuration,
    params: ListTranscriptionParams,
) -> Result<ResponseContent<ListTranscriptionSuccess>, Error<ListTranscriptionError>> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Transcriptions.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid)
    );
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
        let entity: Option<ListTranscriptionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListTranscriptionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
