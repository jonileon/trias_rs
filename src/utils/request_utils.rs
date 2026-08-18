use crate::utils::time_format::get_date_time_now;
use crate::utils::env::{EnvVars, get_var};
use serde::{Deserialize, Serialize};
use reqwest::Response;
use tokio_util::io::{StreamReader, SyncIoBridge};
use futures_util::TryStreamExt;

#[derive(Serialize)]
#[serde(rename = "Trias")]
pub struct TriasRequestEnvelope<T> {
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

#[derive(Deserialize)]
#[serde(rename = "Trias")]
pub struct TriasResponseEnvelope<T> {
    #[serde(rename = "ServiceDelivery")]
    pub service_delivery: ServiceDelivery<T>,
}

#[derive(Deserialize)]
pub struct ServiceDelivery<T> {
    #[serde(rename = "DeliveryPayload")]
    pub payload: T,
}

pub fn build_request_envelope<T>(payload: T) -> TriasRequestEnvelope<T> {
    let version        = get_var(EnvVars::TriasVersion);
    let xmlns          = get_var(EnvVars::Xmlns);
    let xmlns_xsi      = get_var(EnvVars::XsiXmlns);
    let schema_location = get_var(EnvVars::XsiSchemaLocation);
    let xmlns_siri     = get_var(EnvVars::SiriXmlns);

    let rref = get_var(EnvVars::RequestorRef);

    TriasRequestEnvelope{ version, xmlns, xmlns_siri, schema_location, xmlns_xsi, request: ServiceRequest{
        request_timestamp: get_date_time_now(),
        requestor_ref: rref,
        payload
    } }
}

pub async fn parse_response<T>(response: Response) -> Result<T, Box<dyn std::error::Error>> 
    where
    T: serde::de::DeserializeOwned + Send + 'static,
{
    let byte_stream = response
        .bytes_stream()
        .map_err(std::io::Error::other);

    let async_reader = StreamReader::new(byte_stream);

    let result = tokio::task::spawn_blocking(move || {
        let sync_reader = SyncIoBridge::new(async_reader);
        let buf_reader = std::io::BufReader::new(sync_reader);

        quick_xml::de::from_reader(buf_reader)
    })
    .await??;

    Ok(result)
}
