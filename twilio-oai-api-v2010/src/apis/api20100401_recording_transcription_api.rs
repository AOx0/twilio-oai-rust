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

/// struct for passing parameters to the method [`delete_recording_transcription`]
#[derive(Clone, Debug)]
pub struct DeleteRecordingTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete.
    pub account_sid: String,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to delete.
    pub recording_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_recording_transcription`]
#[derive(Clone, Debug)]
pub struct FetchRecordingTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch.
    pub account_sid: String,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to fetch.
    pub recording_sid: String,
    /// The Twilio-provided string that uniquely identifies the Transcription resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_recording_transcription`]
#[derive(Clone, Debug)]
pub struct ListRecordingTranscriptionParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read.
    pub account_sid: String,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcriptions to read.
    pub recording_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i64>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>,
}

/// struct for typed successes of method [`delete_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRecordingTranscriptionSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchRecordingTranscriptionSuccess {
    Status200(models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRecordingTranscriptionSuccess {
    Status200(models::ListRecordingTranscriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRecordingTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchRecordingTranscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_recording_transcription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRecordingTranscriptionError {
    UnknownValue(serde_json::Value),
}

///
pub async fn delete_recording_transcription(
    configuration: &configuration::Configuration,
    params: DeleteRecordingTranscriptionParams,
) -> Result<
    ResponseContent<DeleteRecordingTranscriptionSuccess>,
    Error<DeleteRecordingTranscriptionError>,
> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
        RecordingSid = crate::apis::urlencode(params.recording_sid),
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
        let entity: Option<DeleteRecordingTranscriptionSuccess> =
            serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteRecordingTranscriptionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

///
pub async fn fetch_recording_transcription(
    configuration: &configuration::Configuration,
    params: FetchRecordingTranscriptionParams,
) -> Result<
    ResponseContent<FetchRecordingTranscriptionSuccess>,
    Error<FetchRecordingTranscriptionError>,
> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
        RecordingSid = crate::apis::urlencode(params.recording_sid),
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
        let entity: Option<FetchRecordingTranscriptionSuccess> =
            serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchRecordingTranscriptionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

///
pub async fn list_recording_transcription(
    configuration: &configuration::Configuration,
    params: ListRecordingTranscriptionParams,
) -> Result<
    ResponseContent<ListRecordingTranscriptionSuccess>,
    Error<ListRecordingTranscriptionError>,
> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid),
        RecordingSid = crate::apis::urlencode(params.recording_sid)
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
        let entity: Option<ListRecordingTranscriptionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListRecordingTranscriptionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
