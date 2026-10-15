pub mod bus_info;
pub mod bus_locations;
pub mod otobusumnerede;
pub mod stop_arrivals;

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
