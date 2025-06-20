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
pub struct ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resource.
    #[serde(
        rename = "account_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sid: Option<Option<String>>,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Participant resource is associated with.
    #[serde(
        rename = "call_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub call_sid: Option<Option<String>>,
    /// The user-specified label of this participant, if one was given when the participant was created. This may be used to fetch, update or delete the participant.
    #[serde(
        rename = "label",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<Option<String>>,
    /// The SID of the participant who is being `coached`. The participant being coached is the only participant who can hear the participant who is `coaching`.
    #[serde(
        rename = "call_sid_to_coach",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub call_sid_to_coach: Option<Option<String>>,
    /// Whether the participant is coaching another call. Can be: `true` or `false`. If not present, defaults to `false` unless `call_sid_to_coach` is defined. If `true`, `call_sid_to_coach` must be defined.
    #[serde(
        rename = "coaching",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub coaching: Option<Option<bool>>,
    /// The SID of the conference the participant is in.
    #[serde(
        rename = "conference_sid",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub conference_sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(
        rename = "date_created",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(
        rename = "date_updated",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<Option<String>>,
    /// Whether the conference ends when the participant leaves. Can be: `true` or `false` and the default is `false`. If `true`, the conference ends and all other participants drop out when the participant leaves.
    #[serde(
        rename = "end_conference_on_exit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_conference_on_exit: Option<Option<bool>>,
    /// Whether the participant is muted. Can be `true` or `false`.
    #[serde(
        rename = "muted",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub muted: Option<Option<bool>>,
    /// Whether the participant is on hold. Can be `true` or `false`.
    #[serde(
        rename = "hold",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hold: Option<Option<bool>>,
    /// Whether the conference starts when the participant joins the conference, if it has not already started. Can be: `true` or `false` and the default is `true`. If `false` and the conference has not started, the participant is muted and hears background music until another participant starts the conference.
    #[serde(
        rename = "start_conference_on_enter",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_conference_on_enter: Option<Option<bool>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ParticipantEnumStatus>,
    /// The wait time in milliseconds before participant's call is placed. Only available in the response to a create participant request.
    #[serde(
        rename = "queue_time",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_time: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(
        rename = "uri",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
        ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant {
            account_sid: None,
            call_sid: None,
            label: None,
            call_sid_to_coach: None,
            coaching: None,
            conference_sid: None,
            date_created: None,
            date_updated: None,
            end_conference_on_exit: None,
            muted: None,
            hold: None,
            start_conference_on_enter: None,
            status: None,
            queue_time: None,
            uri: None,
        }
    }
}
