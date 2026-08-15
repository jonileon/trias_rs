use dotenv::dotenv;
use trias_vdv_431::requests::location_information_request::get_location_by_string;

// example interaction
const _EXAMPLE_PAYLOAD: &str = 
r#"
<TripRequest>
    <Origin>
        <LocationRef>
            <StopPlaceRef>de:08212:7</StopPlaceRef>
        </LocationRef>
        <DepArrTime>2026-08-16T15:00:00</DepArrTime>
    </Origin>
    <Destination>
        <LocationRef>
            <StopPlaceRef>de:08212:89</StopPlaceRef>
        </LocationRef>
    </Destination>
    <Params>
        <NumberOfResults>10</NumberOfResults>
        <IncludeTrackSections>false</IncludeTrackSections>
        <IncludeLegProjection>false</IncludeLegProjection>main
        <IncludeIntermediateStops>false</IncludeIntermediateStops>
        <IncludeOperatingDays>false</IncludeOperatingDays>
        <IncludeFares>true</IncludeFares>
    </Params>
</TripRequest>
"#;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    get_location_by_string("Durlacher Tor").await;
}

