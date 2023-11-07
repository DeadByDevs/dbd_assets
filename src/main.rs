use std::fs::File;
use std::io::Write;
mod config;
use crate::config::DBD_OFFICIAL_API;

fn create_folder_if_not_exist(folder_name: &str) {
    if !std::path::Path::new(folder_name).exists() {
        std::fs::create_dir(folder_name).unwrap();
    }
}

async fn download_from_url(url: &str, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = File::create(save_path)?;
    file.write_all(response.text().await?.as_bytes())?;
    println!("Downloaded to {}", save_path);
    Ok(())
}

async fn download_with_slug(slug_name: String) {
    let download_url: String =
        DBD_OFFICIAL_API.to_owned() + "characters/" + &slug_name + "/page-data.json";
    let save_path = String::from("./assets/characters/") + &slug_name + ".json";
    create_folder_if_not_exist("./assets/characters");

    if let Err(err) = download_from_url(&download_url, &save_path).await {
        eprintln!("Failed to download {}: {}", slug_name, err);
    } else {
        println!("{} downloaded successfully.", slug_name);
    }
}

fn get_slug_name(
    json_file_path: &str,
    array_index: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(json_file_path)?;
    let json: serde_json::Value = serde_json::from_reader(file)?;
    let slug_name = json["result"]["pageContext"]["postsData"]["characters"]["edges"]
        [array_index]["node"]["slug"].as_str().unwrap().to_owned();
    Ok(slug_name)
}

fn get_edges_size(json_file_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(json_file_path)?;
    let json: serde_json::Value = serde_json::from_reader(file)?;
    let edges_size = json["result"]["pageContext"]["postsData"]["characters"]["edges"].as_array().unwrap().len();
    Ok(edges_size)
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
        if let Ok(edges_size) = get_edges_size(characters_save_path) {
            println!("Downloading {} characters...", edges_size);
            for i in 0..edges_size {
                if let Ok(slug_name) = get_slug_name(characters_save_path, i) {
                    download_with_slug(slug_name).await;
                }
            }
        }
    }

    if let Err(err) = download_from_url(&chapters_url, chapters_save_path).await {
        eprintln!("Failed to download chapters: {}", err);
    } else {
        println!("Chapters downloaded successfully.");
    }
}

fn main() {
    create_folder_if_not_exist("./assets");
    println!("Downloading assets...");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(download());
}
