use crate::model::loco::Locale;
use crate::utils::LOCALISE_API_URL;

pub async fn get_data(
    locale: &str,
    filters: &str,
    source: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN").unwrap();
    let source_param = source;
    let url_string = format!(
        "{LOCALISE_API_URL}export/locale/{locale}.json?key={token}&fallback=en{filters}&source={source_param}",
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(url_string)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    // Check for HTTP error status codes
    let status = resp.status();
    if status.is_client_error() || status.is_server_error() {
        let error_body = resp.text().await?;
        return Err(format!("HTTP Error: Status {} - {}", status, error_body).into());
    }
    let string_resp = resp.text().await?;
    Ok(string_resp)
}

pub async fn get_all_locales() -> Result<Vec<Locale>, Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN").unwrap();
    let url_string = format!("{LOCALISE_API_URL}locales?key={token}");
    let client = reqwest::Client::new();
    let resp = client.get(url_string).send().await?;
    let string_resp = resp.text().await?;
    let locales: Vec<Locale> = serde_json::from_str(&string_resp)?;

    Ok(locales)
}
