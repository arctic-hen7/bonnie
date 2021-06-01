use dircpy::*;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use std::{fs, fs::File, path::Path};
use tar::Archive;
use url::Url;
use async_recursion::async_recursion;

const NODE_MODULES: &str = "node_modules";

pub async fn get_latest_version(dependency: &String) -> Result<(&String, String), String> {
    let url = format!("https://registry.npmjs.org/{}", dependency);
    let reponse = reqwest::get(url).await.unwrap();
    let object: std::result::Result<serde_json::Value, reqwest::Error> =
        reponse.json::<serde_json::Value>().await;
    match object {
        Ok(object) => {
            let data = &*object.get("dist-tags").unwrap();
            return Ok((dependency, String::from(data["latest"].as_str().unwrap())));
        }
        Err(_) => return Err(String::from("Error getting latest version of package")),
    };
}

pub async fn get_tarball_download_link_and_name(
    dependency: &String,
    version: &String,
) -> Result<(String, String), String> {
    let url = format!("https://registry.npmjs.org/{}/{}", dependency, version);
    let reponse = reqwest::get(url).await.unwrap();
    let object: std::result::Result<serde_json::Value, reqwest::Error> =
        reponse.json::<serde_json::Value>().await;
    match object {
        Ok(object) => {
            let data = &*object.get("dist").unwrap();
            let name = &*object.get("name").unwrap();
            return Ok((
                String::from(data["tarball"].as_str().unwrap()),
                (String::from(name.as_str().unwrap())),
            ));
        }
        Err(_) => return Err(String::from("Error getting download link")),
    };
}

pub async fn get_dependencies_and_dev_dependencies(
    dependency: &String,
    version: &String,
) -> Result<HashMap<std::string::String, std::string::String>, String> {
    let url = format!("https://registry.npmjs.org/{}/{}", dependency, version);
    let mut all_dependencies: HashMap<std::string::String, std::string::String> = HashMap::new();
    let reponse = reqwest::get(url).await.unwrap();
    let object: std::result::Result<serde_json::Value, reqwest::Error> =
        reponse.json::<serde_json::Value>().await;
    match object {
        Ok(object) => {
            let dependencies = object.get("dependencies");
            // let dev_dependencies = object.get("devDependencies");
            match dependencies {
                Some(dependency_value) => {
                    let dependency_map = dependency_value.as_object().unwrap();
                    for (key, value) in dependency_map.iter() {
                        all_dependencies
                            .insert(String::from(key), String::from(value.as_str().unwrap()));
                    }
                }
                None => {}
            }
            //don't install dev dep
            // match dev_dependencies {
            //     Some(dependency_value) => {
            //         let dependency_map = dependency_value.as_object().unwrap();
            //         for (key, value) in dependency_map.iter() {
            //             all_dependencies
            //                 .insert(String::from(key), String::from(value.as_str().unwrap()));
            //         }
            //     }
            //     None => {}
            // }
            return Ok(all_dependencies);
        }
        Err(_) => return Err(String::from("Error getting package")),
    };
}

pub async fn download_package(url: (String, String)) -> Result<(), Box<dyn Error>> {
    check_node_module().unwrap_or_else(|error| {
        eprintln!("error creating node_modules dir {}", error);
        std::process::exit(1);
    });
    let response = reqwest::get(&url.0).await?;
    let url_parse = Url::parse(&url.0.as_str()).unwrap();
    let filename = url_parse.path().split("/").last().unwrap();
    let write_path = format!("{}/{}", NODE_MODULES, filename);
    let mut file = std::fs::File::create(&write_path)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    exract(&write_path, &url.1).unwrap();
    Ok(())
}

fn check_node_module() -> Result<(), Box<dyn Error>> {
    let exist = Path::new(NODE_MODULES).exists();

    if !exist {
        fs::create_dir(NODE_MODULES)?;
    }
    Ok(())
}

fn exract(zip_file: &str, folder: &String) -> Result<(), Box<dyn Error>> {
    let tar_gz = File::open(zip_file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let write_path = format!("{}/{}", NODE_MODULES, folder);
    archive.unpack(&write_path)?;
    let from = format!("{}/package/", &write_path);
    copy_dir(&from, write_path)?;
    fs::remove_dir_all(&from)?;
    fs::remove_file(zip_file)?;
    Ok(())
}


#[async_recursion]
pub async fn get_related_dependencies(dependency:&String, version:&String)
->Result<HashMap<std::string::String, std::string::String>, String>{
    let url = format!("https://registry.npmjs.org/{}/{}", dependency, version);
    let mut all_dependencies: HashMap<std::string::String, std::string::String> = HashMap::new();
    let reponse = reqwest::get(url).await.unwrap();
    let object: std::result::Result<serde_json::Value, reqwest::Error> =
        reponse.json::<serde_json::Value>().await;
    match object {
        Ok(object)=>{
            let dependencies = object.get("dependencies");
            let dev_dependencies = object.get("devDependencies");

            match dependencies {
                Some(dependency_value) => {
                    let dependency_map = dependency_value.as_object().unwrap();
                    if dependency_map.len()>0 {
                        for (key, value) in dependency_map.iter() {
                            all_dependencies
                                .insert(String::from(key), String::from(value.as_str().unwrap()));
                            get_related_dependencies(key, &String::from(value.as_str().unwrap())).await?;
                        }
                    }
                }
                None => {}
            }
            return Ok(all_dependencies);

        }
        Err(_)=>{return Err(String::from("Error getting package"))}
    }
}