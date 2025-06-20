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

/// struct for passing parameters to the method [`create_bulk_contacts`]
#[derive(Clone, Debug)]
pub struct CreateBulkContactsParams {
    /// A list of objects where each object represents a contact's details. Each object includes the following fields: `contact_id`, which must be a string representing phone number in [E.164 format](https://www.twilio.com/docs/glossary/what-e164); `correlation_id`, a unique 32-character UUID that maps the response to the original request; `country_iso_code`, a string representing the country using the ISO format (e.g., US for the United States); and `zip_code`, a string representing the postal code.
    pub items: Vec<serde_json::Value>,
}

/// struct for typed successes of method [`create_bulk_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateBulkContactsSuccess {
    Status201(models::AccountsPeriodV1PeriodBulkContacts),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_bulk_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateBulkContactsError {
    UnknownValue(serde_json::Value),
}

///
pub async fn create_bulk_contacts(
    configuration: &configuration::Configuration,
    params: CreateBulkContactsParams,
) -> Result<ResponseContent<CreateBulkContactsSuccess>, Error<CreateBulkContactsError>> {
    let uri_str = format!("{}/v1/Contacts/Bulk", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    let mut multipart_form_params = std::collections::HashMap::new();
    multipart_form_params.insert(
        "Items",
        params
            .items
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string(),
    );
    req_builder = req_builder.form(&multipart_form_params);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateBulkContactsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateBulkContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
