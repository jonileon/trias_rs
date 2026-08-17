use std::env::var;

use enum_assoc::Assoc;

#[derive(Assoc)]
#[func(pub const fn name(&self) -> &'static str)]
#[func(pub const fn description(&self) -> &'static str)]
pub enum EnvVars {
    #[assoc(name = "TRIAS_VERSION", description = 
    "Version of TRIAS that is used. Currently supported:
        - 1.2"
        )]
    TriasVersion,
    #[assoc(name = "XMLNS", description = 
        "TRIAS XML namespace. The public one by the Vdv is \"http://www.vdv.de/trias\""
        )]
    Xmlns,
    #[assoc(name = "XSI_SCHEMA_LOCATION", description = 
        "Location of the xsd TRIAS schema file"
        )]
    XsiSchemaLocation,
    #[assoc(name = "XSI_XMLNS", description = 
        "TRIAS XML namespace for XS for XSI"
        )]
    XsiXmlns,
    #[assoc(name = "SIRI_XMLNS", description = 
        "Location of the xsd TRIAS schema file"
        )]
    SiriXmlns,
    #[assoc(name = "REQUESTOR_REF", description = 
        "Your private RequestorRef. You need to aquire this from the provider you want to access."
        )]
    RequestorRef,
}

pub fn get_var(v: EnvVars) -> String{
    var(v.name()).expect(&format!(

"Missing environment variable {}!
Description:
    {}",

        v.name(), v.description()
    ))
}
