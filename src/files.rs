use std::{fs::File, io::Write};

pub async fn write_file_in(
    data: String,
    export_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{export_path}.json");
    let mut file = File::create(file_name)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
