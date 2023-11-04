use std::fs::File;
use std::io::Write;
mod config;
use crate::config::DBD_OFFICIAL_API;

fn create_folder_if_not_exist() {
    let assets_folder: &str = "./assets";
    if !std::path::Path::new(assets_folder).exists() {
        std::fs::create_dir(assets_folder).unwrap();
    }
}

async fn download_from_url(url: &str, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = File::create(save_path)?;
    file.write_all(response.text().await?.as_bytes())?;
    println!("Downloaded to {}", save_path);
    Ok(())
}

async fn download() {
    let maps_url: String = DBD_OFFICIAL_API.to_owned() + "maps/page-data.json";
    let characters_url: String = DBD_OFFICIAL_API.to_owned() + "characters/page-data.json";
    let chapters_url: String = DBD_OFFICIAL_API.to_owned() + "chapters/page-data.json";

    let maps_save_path: &str = "./assets/maps.json";
    let characters_save_path: &str = "./assets/characters.json";
    let chapters_save_path: &str = "./assets/chapters.json";

    if let Err(err) = download_from_url(&maps_url, maps_save_path).await {
        eprintln!("Failed to download maps: {}", err);
    } else {
        println!("Maps downloaded successfully.");
    }

    if let Err(err) = download_from_url(&characters_url, characters_save_path).await {
        eprintln!("Failed to download characters: {}", err);
    } else {
        println!("Characters downloaded successfully.");
    }

    if let Err(err) = download_from_url(&chapters_url, chapters_save_path).await {
        eprintln!("Failed to download chapters: {}", err);
    } else {
        println!("Chapters downloaded successfully.");
    }
}

fn main() {
    create_folder_if_not_exist();
    println!("Downloading assets...");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(download());
}
