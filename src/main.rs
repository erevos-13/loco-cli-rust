mod api;
mod commands;
mod files;
mod model;
mod utils;

use api::get_data;
use api::post_data;
use clap::Parser;
use colored::*;
use commands::Args;
use dotenv::dotenv;
use files::write_file_in;
use model::loco::ImportResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pb = indicatif::ProgressBar::new(100);
    let args = Args::parse();
    let path = args.path;
    let locale = args.locale;
    let export_path = args.export_path;
    let filters = args.filters;
    let post = args.post.unwrap_or(true);
    let get = args.get.unwrap_or(true);
    let source = args.source.unwrap_or(String::from(""));

    if post {
        let resp_post_data: ImportResponse = post_data(&path, &locale).await?;
        for locale in resp_post_data.locales {
            println!("You send {0} to localise", locale.name);
        }
    }

    if get {
        let resp_get_data = get_data(&locale, Some(filters), &source).await;
        match resp_get_data {
            Ok(data) => {
                println!("{}", "You get the data from localise".green().bold());
                let _ = write_file_in(data, &export_path).await;
                println!("{}", "You write the data to the file".green().bold());
            }
            Err(e) => {
                println!("Error: {}", e.to_string().red());
            }
        }
    }

    pb.finish();
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
