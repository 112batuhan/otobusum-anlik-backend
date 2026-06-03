pub mod fetch_bus_info;
pub mod fetch_bus_locations;

pub fn get_opendata_xml_body(key_outer: &str, key: &str, value: &str) -> String {
    format!(
        r#"
        <soap:Envelope
            xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
                <soap:Body>
                    <{key_outer}
                        xmlns="http://tempuri.org/">
                        <{key}>{value}</{key}>
                    </{key_outer}>
                </soap:Body>
            </soap:Envelope>
        "#
    )
}
