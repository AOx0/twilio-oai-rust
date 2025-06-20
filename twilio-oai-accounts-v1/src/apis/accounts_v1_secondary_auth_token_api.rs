/*
 * Twilio - Accounts
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

/// struct for typed successes of method [`create_secondary_auth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecondaryAuthTokenSuccess {
    Status201(models::AccountsPeriodV1PeriodSecondaryAuthToken),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_secondary_auth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecondaryAuthTokenSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_secondary_auth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecondaryAuthTokenError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_secondary_auth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecondaryAuthTokenError {
    UnknownValue(serde_json::Value),
}

/// Create a new secondary Auth Token
pub async fn create_secondary_auth_token(
    configuration: &configuration::Configuration,
) -> Result<ResponseContent<CreateSecondaryAuthTokenSuccess>, Error<CreateSecondaryAuthTokenError>>
{
    let uri_str = format!("{}/v1/AuthTokens/Secondary", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
        let entity: Option<CreateSecondaryAuthTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateSecondaryAuthTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete the secondary Auth Token from your account
pub async fn delete_secondary_auth_token(
    configuration: &configuration::Configuration,
) -> Result<ResponseContent<DeleteSecondaryAuthTokenSuccess>, Error<DeleteSecondaryAuthTokenError>>
{
    let uri_str = format!("{}/v1/AuthTokens/Secondary", configuration.base_path);
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
        let entity: Option<DeleteSecondaryAuthTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteSecondaryAuthTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
