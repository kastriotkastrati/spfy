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

async fn download_image(
    image_url: &str,
    setup: setup::SetupStruct,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(image_url).send().await?;
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());

    let extension = match content_type {
        Some("image/jpeg") => "jpg",
        Some("image/png") => "png",
        Some("image/gif") => "gif",
        Some("image/bmp") => "bmp",
        Some("image/webp") => "webp",
        _ => "unknown",
    };

    let image_name = nanoid::nanoid!();
    let image_name = format!("{}.{}", image_name, extension);
    let image_path = {
        let mut path_buff = setup.dir_path.to_path_buf();
        path_buff.push(image_name);
        path_buff
    };

    let image_bytes = response.bytes().await.unwrap();
    let _ = save_file_to_fs(&image_path, image_bytes)
        .expect("Could not save the image of the current song on your machine:(");

    let image_path = image_path.to_string_lossy().to_string();
    return Ok(image_path);
}

pub async fn run_program() -> () {
    let setup_data = setup::setup();
    let os = std::env::consts::OS;
    let impl_commands = commands::main::get_impl_commands(os);

    let image_url_from_impl = impl_commands.get_spotify_data();
    let image_url_cleaned: String = image_url_from_impl
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let saved_image_url = download_image(&image_url_cleaned, setup_data).await;
    let saved_image_url = saved_image_url.expect("Couldn't save the image:(");
    let saved_image_path = std::path::Path::new(&saved_image_url);
    let _ = impl_commands.set_wallpaper(saved_image_path);
}
