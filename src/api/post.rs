use std::fs;

use crate::model::loco::ImportResponse;
use crate::utils::LOCALISE_API_URL;

pub async fn post_data(
    path: &str,
    locale: &str,
) -> Result<ImportResponse, Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN");

    match token {
        Ok(tok) => {
            let json_file = fs::read_to_string(path).unwrap();
            let url_string = format!("{LOCALISE_API_URL}import/json?key={tok}&locale={locale}&ignore-existing=false&tag-absent=obsolete&untag-all=obsolete&format=JSON");
            let client = reqwest::Client::new();
            let resp = client
                .post(url_string)
                .header("Authorization", format!("Bearer {}", tok))
                .body(json_file)
                .send()
                .await?;
            match resp.text().await {
                Ok(content) => {
                    let resp_post_data: ImportResponse = serde_json::from_str(&content).unwrap();
                    Ok(resp_post_data)
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
        Err(err) => {
            println!("Token not found");
            return Err(Box::new(err));
        }
    }
}
