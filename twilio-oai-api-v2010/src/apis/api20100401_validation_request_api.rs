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

/// struct for passing parameters to the method [`create_validation_request`]
#[derive(Clone, Debug)]
pub struct CreateValidationRequestParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for the new caller ID resource.
    pub account_sid: String,
    /// The phone number to verify in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, which consists of a + followed by the country code and subscriber number.
    pub phone_number: String,
    /// A descriptive string that you create to describe the new caller ID resource. It can be up to 64 characters long. The default value is a formatted version of the phone number.
    pub friendly_name: Option<String>,
    /// The number of seconds to delay before initiating the verification call. Can be an integer between `0` and `60`, inclusive. The default is `0`.
    pub call_delay: Option<i32>,
    /// The digits to dial after connecting the verification call.
    pub extension: Option<String>,
    /// The URL we should call using the `status_callback_method` to send status information about the verification process to your application.
    pub status_callback: Option<String>,
    /// The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`, and the default is `POST`.
    pub status_callback_method: Option<String>,
}

/// struct for typed successes of method [`create_validation_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateValidationRequestSuccess {
    Status200(models::ApiPeriodV2010PeriodAccountPeriodValidationRequest),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_validation_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateValidationRequestError {
    UnknownValue(serde_json::Value),
}

///
pub async fn create_validation_request(
    configuration: &configuration::Configuration,
    params: CreateValidationRequestParams,
) -> Result<ResponseContent<CreateValidationRequestSuccess>, Error<CreateValidationRequestError>> {
    let uri_str = format!(
        "{}/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json",
        configuration.base_path,
        AccountSid = crate::apis::urlencode(params.account_sid)
    );
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
    multipart_form_params.insert("PhoneNumber", params.phone_number.to_string());
    if let Some(param_value) = params.friendly_name {
        multipart_form_params.insert("FriendlyName", param_value.to_string());
    }
    if let Some(param_value) = params.call_delay {
        multipart_form_params.insert("CallDelay", param_value.to_string());
    }
    if let Some(param_value) = params.extension {
        multipart_form_params.insert("Extension", param_value.to_string());
    }
    if let Some(param_value) = params.status_callback {
        multipart_form_params.insert("StatusCallback", param_value.to_string());
    }
    if let Some(param_value) = params.status_callback_method {
        multipart_form_params.insert("StatusCallbackMethod", param_value.to_string());
    }
    req_builder = req_builder.form(&multipart_form_params);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateValidationRequestSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent {
            status,
            content,
            entity,
        })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateValidationRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
