use std::fs;

use crate::model::loco::ImportResponse;
use crate::utils::LOCALISE_API_URL;

pub async fn post_data(path: &str, locale: &str) -> Result<ImportResponse, reqwest::Error> {
    let token = std::env::var("TOKEN").unwrap();
    let json_file = fs::read_to_string(path).unwrap();
    let url_string = format!("{LOCALISE_API_URL}import/json?key={token}&locale={locale}&ignore-existing=true&tag-absent=obsolete&format=JSON");
    let client = reqwest::Client::new();
    let resp = client
        .post(url_string)
        .header("Authorization", format!("Bearer {}", token))
        .body(json_file)
        .send()
        .await?;
    let string_resp = resp.text().await?;
    let resp_post_data: ImportResponse = serde_json::from_str(&string_resp).unwrap();
    Ok(resp_post_data)
}
