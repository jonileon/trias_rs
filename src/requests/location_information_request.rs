use serde::{Deserialize, Serialize};
use quick_xml::se::to_string;
use reqwest::Client;

use crate::utils::request_utils::{TriasRequestEnvelope, TriasResponseEnvelope, build_request_envelope, parse_response};

#[derive(Deserialize)]
pub struct LocationInformationResultPayload {
    #[serde(rename = "LocationInformationResponse")]
    pub result_information: LocationInformationResponse,
}

#[derive(Deserialize)]
pub struct LocationInformationResponse {
    #[serde(rename = "LocationResult")]
    pub location_results: Vec<LocationResult>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocationResult {
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    #[serde(rename = "StopPoint")]
    pub stop_point: StopPoint,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopPoint {
    #[serde(rename = "StopPointRef")]
    pub id: String,
    #[serde(rename = "StopPointName")]
    pub name: StopPointName,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopPointName {
    #[serde(rename = "Language")]
    pub lang: String,
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Serialize)]
struct LocationInformationRequestPayload {
    #[serde(rename = "LocationInformationRequest")]
    request_information: LocationInformationRequest,
}

#[derive(Serialize)]
struct LocationInformationRequest {
    #[serde(rename = "InitialInput", skip_serializing_if = "Option::is_none")]
    initial_input: Option<InitialInput>,

    #[serde(rename = "LocationRef", skip_serializing_if = "Option::is_none")]
    location_ref: Option<LocationRef>,
    #[serde(rename = "Restrictions")]
    restrictions: Restrictions,
}

#[derive(Serialize)]
struct Restrictions {
    #[serde(rename = "Type")]
    result_type: &'static str
}

#[derive(Serialize)]
struct InitialInput {
    #[serde(rename = "LocationName")]
    location_name: String
}

#[derive(Serialize)]
struct LocationRef {
    #[serde(rename = "StopPointRef")]
    id: String
}


pub async fn get_location_by_string(url: &str, input: &str) -> Result<Vec<LocationResult>, Box<dyn std::error::Error>> {
    let payload = LocationInformationRequestPayload{request_information: LocationInformationRequest{initial_input: Some(InitialInput{ location_name: input.to_string()}), location_ref: None, restrictions: Restrictions{ result_type: "stop"}}};
    let body: TriasRequestEnvelope<LocationInformationRequestPayload> = build_request_envelope(payload);
    let body_str = to_string(&body).expect("Error while serializing xml: LocationInformationRequest");

    let client = Client::builder().build().unwrap();
    let res = client.post(url)
        .header("Content-Type", "application/xml")
        .body(body_str)
        .send()
        .await
        .unwrap();
    let result: TriasResponseEnvelope<LocationInformationResultPayload> = parse_response(res).await?;
    Ok(result.service_delivery.payload.result_information.location_results)
}

pub async fn get_location_by_ref(url: &str, stop_ref: &str) -> Result<LocationResult, Box<dyn std::error::Error>> {
    let payload = LocationInformationRequestPayload{request_information: LocationInformationRequest{initial_input: None, location_ref: Some(LocationRef{ id: stop_ref.to_string() }), restrictions: Restrictions{ result_type: "stop"}}};
    let body: TriasRequestEnvelope<LocationInformationRequestPayload> = build_request_envelope(payload);
    let body_str = to_string(&body).expect("Error while serializing xml: LocationInformationRequest");

    let client = Client::builder().build().unwrap();
    let res = client.post(url)
        .header("Content-Type", "application/xml")
        .body(body_str)
        .send()
        .await
        .unwrap();
    let result: TriasResponseEnvelope<LocationInformationResultPayload> = parse_response(res).await?;
    Ok(result.service_delivery.payload.result_information.location_results.first().unwrap().clone())
}
