mod models;
use core::str;

use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use tokio::io; // {{ edit_1 }}

use models::models::ImportResponse;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    token: String,
    #[arg(short, long)]
    path: String,
    #[arg(short, long, default_value = "en")]
    locale: String,
    #[arg(short, long, default_value = "output.json")]
    export_path: String,
}

async fn post_data(token: &str, path: &str, locale: &str) -> Result<String, reqwest::Error> {
    let json_file = fs::read_to_string(path).unwrap();
    let url_string = format!("https://localise.biz/api/import/json?key={token}&locale={locale}&ignore-existing=true&tag-absent=obsolete&format=JSON");
    let client = reqwest::Client::new();
    let resp = client
        .post(url_string)
        .header("Authorization", format!("Bearer {}", token))
        .body(json_file)
        .send()
        .await?;
    resp.text().await
}

async fn get_data(token: &str, locale: &str) -> Result<String, reqwest::Error> {
    let url_string =
        format!("https://localise.biz/api/export/locale/{locale}.json?key={token}&fallback=en");
    let client = reqwest::Client::new();
    let resp = client
        .get(url_string)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    resp.text().await
}

async fn write_file_in(data: String, export_path: &str) -> Result<(), io::Error> {
    let file_name = format!("{export_path}.json");
    let mut file = File::create(file_name)?; // This line is correct
    file.write_all(data.as_bytes())?; // This line is correct
    Ok(()) // This line is correct
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pb = indicatif::ProgressBar::new(100);
    
    let args = Cli::parse();
    let resp = post_data(&args.token, &args.path, &args.locale).await?;
    let ressponse: ImportResponse = serde_json::from_str(&resp).unwrap();

    if ressponse.status == 200 {
        let resp = get_data(&args.token, &args.locale).await;
        match resp {
            Ok(data) => {
                let _ = write_file_in(data, &args.export_path).await;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    let msg = format!("Locale {}, Path get the: {}, Export path: {}", args.locale, args.path, args.export_path);
    pb.finish_with_message(msg);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test] // Use tokio for async testing
    async fn test_write_file_in() {
        let test_data = String::from("Hello, world!");
        let test_path = "test_output"; // Temporary file name

        // Call the function
        let result = write_file_in(test_data.clone(), test_path).await;

        // Check if the function executed successfully
        assert!(result.is_ok());

        // Verify the content of the file
        let content = fs::read_to_string(format!("{test_path}.json")).expect("Unable to read file");
        assert_eq!(content, test_data);

        // Clean up the test file
        fs::remove_file(format!("{test_path}.json")).expect("Unable to delete file");
    }
}
