use dotenv::var;
use reqwest::Client;

use crate::utils::request_building::build_header;


fn get_payload_string_search(input: &str) -> String {
    format!(

r#"
<LocationInformationRequest>
    <InitialInput>
        <LocationName>"{}"</LocationName>
    </InitialInput>
</LocationInformationRequest>
"#,
        
        input
    )
}

pub async fn get_location_by_string(input: &str) {
    let payload = get_payload_string_search(input);
    let (start, end) = build_header();
    let req_body = format!(
        "{}
         {}
         {}
        ",
        start,
        payload,
        end
    );
    let url = var("URL").expect("Missing env var URL");
    let client = Client::builder().build().unwrap();
    println!("-------------REQUEST-------------");
    println!("{}", req_body);
    let res = client.post(url)
        .header("Content-Type", "application/xml")
        .body(req_body)
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
