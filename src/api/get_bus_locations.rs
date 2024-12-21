use serde_xml_rs;

struct BusLocationsResponse {
    
}

fn get_body(key_outer: &str, key: &str, value: &str) -> String {
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

pub async fn get_bus_locations(client: &reqwest::Client, line_code: &str) -> anyhow::Result<()> {
    let body = get_body("GetHatOtoKonum_json", "HatKodu", line_code);

    let response = client.post("https://api.ibb.gov.tr/iett/FiloDurum/SeferGerceklesme.asmx")
        .header("Content-Type", "text/xml; charset=UTF-8")
        .header("SOAPAction", r#""http://tempuri.org/GetHatOtoKonum_json""#)
        .body(body)
        .send()
        .await?;

    let content = response.text()
        .await?;

    println!("{content}");

    Ok(())
}
