mod api;
mod commands;
mod files;
mod model;
mod utils;

use std::fs::create_dir_all;
use std::path::Path;

use api::get::get_all_locales;
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
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
    );

    let args = Args::parse();
    let path = args.path;
    let locale = args.locale;
    let export_path = args.export_path;
    let filters = args.filters;
    let post = args.post.unwrap_or(true);
    let get = args.get.unwrap_or(true);
    let source = args.source.unwrap_or(String::from(""));
    let extract_all = args.extract_all.unwrap_or(false);
    let filters_string = match Some(filters) {
        Some(filters) => {
            let filters_join = filters.join(",");
            format!("&filter={filters_join}")
        }
        _ => String::from(""),
    };

    if post {
        pb.set_length(1);
        let resp_post_data: ImportResponse = post_data(&path, &locale).await?;
        for locale in resp_post_data.locales {
            let msg = format!("We send {} the localise", locale.name);
            println!("{}", msg.green().bold());
        }
        pb.set_position(1);
    }

    if extract_all {
        println!("{}", "Extract all locales".white().bold());
        let resp_get_data = get_all_locales().await;
        let all_locales = match resp_get_data {
            Err(_) => panic!("Error on extract all locale"),
            Ok(locales) => locales,
        };
        pb.set_length(all_locales.len() as u64 - 1);
        for (i, locale) in all_locales.iter().enumerate() {
            pb.set_message(format!("Processing locale: {}", locale.code));
            let resp_get_data = get_data(&locale.code, &filters_string, &source).await;
            match resp_get_data {
                Ok(data) => {
                    let msg = format!(
                        "We extract the locale {} of the name {}",
                        locale.code, locale.name
                    );
                    println!("{}", msg.yellow().bold());
                    if !Path::new(&export_path).exists() {
                        match create_dir_all(&export_path) {
                            Ok(_) => println!("Directory created"),
                            Err(e) => println!(
                                "Failed to create directory: {} the dir wit pass {}",
                                e, export_path
                            ),
                        }
                    }
                    let export_to_path = format!("{}/{}", export_path, locale.code);
                    write_file_in(data, &export_to_path).await?;
                    let msg = format!(
                        "We store the locale {} to the path {}/{}",
                        &locale.code, export_to_path, &locale.code
                    );
                    println!("{}", msg.green().italic().bold())
                }
                Err(e) => {
                    println!("Error: {}", e.to_string().red());
                }
            }
            pb.set_position(i as u64 + 1);
        }
    }

    if get {
        pb.set_length(1);
        let resp_get_data = get_data(&locale, &filters_string, &source).await;
        match resp_get_data {
            Ok(data) => {
                println!("{}", "You get the data from localise".green().bold());
                write_file_in(data, &export_path).await?;
                println!("{}", "Write the data to the file".green().bold());
            }
            Err(e) => {
                println!("Error: {}", e.to_string().red());
            }
        }
        pb.set_position(1);
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
