use dotenv::var;
use crate::utils::time_format::get_date_time_now;

pub fn build_header() -> (String, String) {
    let trias_v = var("TRIAS_VERSION").expect("Missing env var TRIAS_VERSION");
    let xmlns = var("XMLNS").expect("Missing env var XMLNS");
    let xsi_xmlns = var("XSI_XMLNS").expect("Missing env var XSI_XMLNS");
    let xsi_schema_loc = var("XSI_SCHEMA_LOCATION").expect("Missing env var XSI_SCHEMA_LOCATION");
    let siri_xmlns = var("SIRI_XMLNS").expect("Missing env var SIRI_XMLNS");

    let rref = var("REQUESTOR_REF").expect("Mising env var REQUESTOR_REF");

    let start = format!(
r#"
<?xml version="1.0" encoding="UTF-8"?>
<Trias version="{}" xmlns="{}" xmlns:siri="{}" xmlns:xsi="{}" xsi:schemaLocation="{}">

<ServiceRequest>
<siri:RequestTimeStamp>{}</siri:RequestTimeStamp>
<siri:RequestorRef>{}</siri:RequestorRef>
<RequestPayload>"#,

        trias_v,
        xmlns,
        xsi_schema_loc,
        xsi_xmlns,
        siri_xmlns,
        get_date_time_now(),
        rref
    );
    (start, String::from(

r#"
</RequestPayload>
</ServiceRequest>   

</Trias>
"#

))
}

