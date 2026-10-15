use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use axum::http::HeaderMap;
use cached::concurrent_cached;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::info;

use crate::models::ist::tokens::TokensResponse;

#[concurrent_cached(
    ttl = "Duration::from_secs(3500)",
    key = "String",
    convert = r#"{ "credentials".to_string() }"#,
    redis = true
)]
pub async fn get_credentials(
    client: &reqwest::Client,
    headers: &reqwest::header::HeaderMap,
) -> anyhow::Result<TokensResponse> {
    let mut auth_body = HashMap::new();

    auth_body.insert("client_id", std::env::var("IBB_CLIENT_ID").unwrap());
    auth_body.insert("client_secret", std::env::var("IBB_CLIENT_SECRET").unwrap());
    auth_body.insert("grant_type", "client_credentials".to_string());
    auth_body.insert("scope", std::env::var("IBB_CLIENT_SCOPE").unwrap());

    info!("getting credentials for internal api of otobusum nerede");

    Ok(client
        .post("https://ntcapi.iett.istanbul/oauth2/v2/auth")
        .headers(headers.clone())
        .json(&auth_body)
        .send()
        .await?
        .json()
        .await?)
}

pub async fn fetch_otobusumnerede<R, B>(client: &reqwest::Client, body: &B) -> Result<R>
where
    R: DeserializeOwned,
    B: Serialize,
{
    let mut headers = HeaderMap::new();

    headers.append("Accept-Encoding", "gzip".parse().unwrap());
    headers.append("Host", "ntcapi.iett.istanbul".parse().unwrap());
    headers.append(
        "Content-Type",
        "application/json; charset=UTF-8".parse().unwrap(),
    );

    let credentials = get_credentials(client, &headers).await?;

    headers.insert(
        "Authorization",
        format!("Bearer {}", credentials.access_token)
            .parse()
            .unwrap(),
    );

    let response = client
        .post("https://ntcapi.iett.istanbul/service")
        .headers(headers)
        .json(body)
        .send()
        .await?
        .json::<R>()
        .await?;

    Ok(response)
}
