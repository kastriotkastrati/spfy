use crate::commands;
use crate::setup;
use bytes;
use nanoid;
use reqwest;

fn save_file_to_fs(
    destination: &std::path::Path,
    bytes: bytes::Bytes,
) -> Result<std::fs::File, std::io::Error> {
    let mut destination_file = std::fs::File::create(destination)?;
    let _ = std::io::copy(&mut bytes.as_ref(), &mut destination_file)?;
    return Ok(destination_file);
}

async fn download_image(image_url: &str, setup: setup::SetupStruct) {
    let client = reqwest::Client::new();
    let response = client.get(image_url).send().await;
    let response = response.unwrap();
    let image_bytes = response.bytes().await.unwrap();

    let image_name = nanoid::nanoid!();
    let image_path = {
        let mut path_buff = setup.dir_path.to_path_buf();
        path_buff.push(image_name);
        path_buff
    };

    let image_file = save_file_to_fs(&image_path, image_bytes);

    println!("image_file: {:?}", image_file);
}

pub async fn run_program() -> () {
    let setup_data = setup::setup();
    let os = std::env::consts::OS;
    let image_url_form_command = commands::main::run_command(os);
    let image_url_cleaned: String = image_url_form_command
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let _ = download_image(&image_url_cleaned, setup_data).await;
}
