// arrange_maps_folder_command.rs

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct MapAuthor {
    author: String,
    packages: Vec<String>,
    maps: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MapArrangement {
    map_authors: Vec<MapAuthor>,
}

// TODO
// - Fix bug where not all files are moved, currently need to click the button multiple times
// - Errors & Notifications should be sent to UI
// - Folder should not be hardcoded but grabbed from a public function elsewhere (maybe the exe finder?)
// - Would be worth error-ing and breaking out of loops if the amount of files/folders is extreme.
// - Multiple file handling? I'd assume newest date file would be the one to keep. 
//   - Or even simply checking the hash of the file with the one on the server or prompt an update after arrangement


#[tauri::command]
pub async fn arrange_maps_folder() -> Result<(), String> {
    let maps_yaml_url = "https://client.update.tamods.org/test-ask-dodge/community-maps.yaml";

    let response = reqwest::get(maps_yaml_url)
        .await
        .map_err(|e| format!("Failed to fetch YAML: {}", e))?;

    if response.status().is_success() {
        let yaml_string = response
            .text()
            .await
            .map_err(|e| format!("Failed to read YAML response: {}", e))?;
        let maps_yaml: MapArrangement = serde_yaml::from_str(&yaml_string)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;
        move_files(&maps_yaml)?;
        Ok(())
    } else {
        Err(format!("Failed to fetch YAML: {}", response.status()))
    }
}

fn move_files(maps_yaml: &MapArrangement) -> Result<(), String> {
    let base_folder = PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Tribes\\TribesGame\\CookedPC\\Maps\\Community Maps");
    
    // Check if the base folder exists, create it if it doesn't
    if !base_folder.exists() {
        fs::create_dir_all(&base_folder)
            .map_err(|e| format!("Failed to create Community Maps folder: {}", e))?;
        return Err("Base folder does not exist".to_string());
    }

    // Iterate over files in the maps directory and its subdirectories
    for entry in WalkDir::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Tribes\\TribesGame\\CookedPC\\Maps") {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        // Check if it's a regular file
        if path.is_file() && !path.to_string_lossy().contains("Community Maps"){
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let ext = path.extension().and_then(|s| s.to_str());

            // If it's a udk or upk file
            match ext {
                Some("udk") | Some("upk") => {
                    for map_author in &maps_yaml.map_authors {
                        let author_folder = base_folder.join(&map_author.author);
                        if !author_folder.exists() {
                            println!("Create Author directory: {:?} ", author_folder);
                            fs::create_dir(&author_folder)
                                .map_err(|e| format!("Failed to create author folder: {}", e))?;
                        }
                        for map in &map_author.maps {
                            if file_name.contains(map) {
                                let dest_path = author_folder.join(&file_name);

                                // Check if file already exists in destination
                                if !dest_path.exists() {
                                    println!("Would move map file from {:?} to {:?}", path, dest_path);
                                    // Uncomment to perform the actual move
                                    fs::rename(&path, &dest_path).map_err(|e| format!("Failed to move file: {}", e))?;
                                }
                            }
                        }
                        for package in &map_author.packages {
                            if file_name.contains(package) {
                                let dest_path = author_folder.join(&file_name);

                                // Check if file already exists in destination
                                if !dest_path.exists() {
                                    println!("Will move package file from {:?} to {:?}", path, dest_path);
                                    // Uncomment to perform the actual move
                                    fs::rename(&path, &dest_path).map_err(|e| format!("Failed to move file: {}", e))?;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}