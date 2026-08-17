use dotenv::var;
use reqwest::Client;
use serde::Serialize;
use quick_xml::se::to_string;

use crate::utils::request_building::{TriasEnvelope, build_request_envelope};

#[derive(Serialize)]
struct LocationInformationRequestPayload {
    #[serde(rename = "LocationInformationRequest")]
    request_information: LocationInformationRequest,
}

#[derive(Serialize)]
struct LocationInformationRequest {
    #[serde(rename = "InitialInput")]
    initial_input: InitialInput,
}

#[derive(Serialize)]
struct InitialInput {
    #[serde(rename = "LocationName")]
    location_name: String
}

pub async fn get_location_by_string(input: &str) {
    let payload = LocationInformationRequestPayload{request_information: LocationInformationRequest{initial_input: InitialInput{ location_name: input.to_string() }}};
    let body: TriasEnvelope<LocationInformationRequestPayload> = build_request_envelope(payload);
    let url = var("URL").expect("Missing env var URL");
    let client = Client::builder().build().unwrap();
    let body_str = to_string(&body).expect("Error while serializing xml");
    println!("{}", body_str);
    let res = client.post(url)
        .header("Content-Type", "application/xml")
        .body(body_str)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("-------------RESULT--------------");
    println!("{}", res);
    println!("---------------------------------\n");
}
