use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::common_structures::{LocationRef, Mode, TriasString};
use crate::utils::request_utils::send_request;

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
    num_results: usize,
    #[serde(rename = "IncludeRealtimeData")]
    include_realtime_data: bool,
    #[serde(rename = "StopEventType")]
    event_type: String,
    #[serde(rename = "TimeWindow", with = "crate::utils::xs_duration")]
    time_window: Duration,
}

//#[derive(Deserialize, Debug, Clone)]
//pub struct StopEventResponseContext {}

#[derive(Deserialize, Debug, Clone)]
pub struct StopEventResponsePayload {
    #[serde(rename = "StopEventResponse")]
    pub stop_event_response: StopEventResponse,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopEventResponse {
    //#[serde(rename = "StopEventResponseContext")]
    //pub context: StopEventResponseContext,
    #[serde(rename = "StopEventResult")]
    pub events: Vec<StopEventResult>
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopEventResult {
    #[serde(rename = "ResultId")]
    pub id: String,
    #[serde(rename = "StopEvent")]
    pub event: StopEvent,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopEvent {
    #[serde(rename = "ThisCall")]
    pub call: ThisCall,
    #[serde(rename = "Service")]
    pub service: Service,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Service {
    #[serde(rename = "JourneyRef")]
    pub journey_ref: String,
    #[serde(rename = "ServiceSection")]
    pub service_section: ServiceSection,
    #[serde(rename = "OriginStopPointRef")]
    pub origin_stop_point_ref: String,
    #[serde(rename = "OriginText")]
    pub origin_stop_point_name: TriasString,
    #[serde(rename = "DestinationText")]
    pub dest_stop_point_name: TriasString,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceSection {
    #[serde(rename = "LineRef")]
    pub line_ref: String,
    #[serde(rename = "DirectionRef")]
    pub direction: String,
    #[serde(rename = "Mode")]
    pub mode: Mode,
    #[serde(rename = "PublishedLineName")]
    pub line_name: TriasString,
    #[serde(rename = "RouteDescription")]
    pub route_descr: TriasString,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ThisCall {
    #[serde(rename = "CallAtStop")]
    pub stop: CallAtStop,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CallAtStop {
    #[serde(rename = "StopPointRef")]
    pub id: String,
    #[serde(rename = "StopPointName")]
    pub name: TriasString,
    #[serde(rename = "PlannedBay")]
    pub bay: TriasString,
    #[serde(rename = "ServiceDeparture")]
    pub departure: ServiceDeparture,
    #[serde(rename = "StopSeqNumber")]
    pub stop_seq_number: usize
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceDeparture {
    #[serde(rename = "TimetabledTime")]
    pub original_time: DateTime<Utc>,
    #[serde(rename = "EstimatedTime")]
    pub estimated_time: Option<DateTime<Utc>>,
}

pub async fn get_trips_for_location(location_ref: &str, num_results: usize, url: &str) -> Result<Vec<StopEventResult>,Box<dyn std::error::Error>> {
    let payload = StopEventRequestPayload{ request: StopEventRequest{ location: LocationContext{ id: LocationRef { id: location_ref.to_string() } } , params: StopEventParams{ num_results, include_realtime_data: true, event_type: "departure".to_string(), time_window: Duration::hours(12) }} };
    let result: StopEventResponsePayload = send_request(payload, url).await?;
    Ok(result.stop_event_response.events)
}
