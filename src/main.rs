use dotenv::{dotenv, var};
use trias_rs::requests::location_information_request::{get_location_by_ref, get_location_by_string};

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
    let url = var("URL").expect("no url specified");
    let result = get_location_by_string(&url, "Büchiger Alle").await.unwrap();
    for stop in &result {
        println!("{:#?}", stop);
    }
    let stop = get_location_by_ref(&url, &result.first().unwrap().location.stop_point.id).await.unwrap();
    println!("{:#?}", stop);
}

