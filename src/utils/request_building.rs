use crate::utils::time_format::get_date_time_now;
use crate::utils::env::{EnvVars, get_var};
use serde::Serialize;
use quick_xml::se::to_string;

#[derive(Serialize)]
#[serde(rename = "Trias")]
pub struct TriasEnvelope<T> {
    #[serde(rename = "@version")]
    pub version: String,

    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "@xmlns:siri")]
    pub xmlns_siri: String,

    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,

    #[serde(rename = "@xsi:schemaLocation")]
    pub schema_location: String,
    #[serde(rename = "ServiceRequest")]
    request: ServiceRequest<T>,
}

#[derive(Serialize)]
pub struct ServiceRequest<T> {
    #[serde(rename = "siri:RequestTimeStamp")]
    request_timestamp: String,
    #[serde(rename = "siri:RequestorRef")]
    requestor_ref: String,
    #[serde(rename = "RequestPayload")]
    payload: T,
}

pub fn build_request_envelope<T>(payload: T) -> TriasEnvelope<T> {
    let version        = get_var(EnvVars::TriasVersion);
    let xmlns          = get_var(EnvVars::Xmlns);
    let xmlns_xsi      = get_var(EnvVars::XsiXmlns);
    let schema_location = get_var(EnvVars::XsiSchemaLocation);
    let xmlns_siri     = get_var(EnvVars::SiriXmlns);

    let rref = get_var(EnvVars::RequestorRef);

    TriasEnvelope{ version, xmlns, xmlns_siri, schema_location, xmlns_xsi, request: ServiceRequest{
        request_timestamp: get_date_time_now(),
        requestor_ref: rref,
        payload
    } }
}

