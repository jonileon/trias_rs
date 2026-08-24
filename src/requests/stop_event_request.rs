use serde::Serialize;

use crate::utils::common_structures::LocationRef;
use crate::utils::request_utils::send_request_test;

#[derive(Serialize)]
struct StopEventRequestPayload {
    #[serde(rename = "StopEventRequest")]
    request: StopEventRequest 
}
#[derive(Serialize)]
struct StopEventRequest {
    #[serde(rename = "Location")]
    location: LocationContext,
    #[serde(rename = "Params")]
    params: StopEventParams
}

#[derive(Serialize)]
struct LocationContext {
    #[serde(rename = "LocationRef")]
    id: LocationRef
}

#[derive(Serialize)]
struct StopEventParams {
    #[serde(rename = "NumberOfResults")]
    num_results: usize
}

pub async fn get_trips_for_location(location_ref: &str, num_results: usize, url: &str) {
    let payload = StopEventRequestPayload{ request: StopEventRequest{ location: LocationContext{ id: LocationRef { id: location_ref.to_string() } } , params: StopEventParams{ num_results }} };
    send_request_test(payload, url).await.unwrap();
}
