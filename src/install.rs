use std::collections::HashMap;

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

pub async fn get_tarball_download_link(
    dependency: &String,
    version: &String,
) -> Result<String, String> {
    let url = format!("https://registry.npmjs.org/{}/{}", dependency, version);
    let reponse = reqwest::get(url).await.unwrap();
    let object: std::result::Result<serde_json::Value, reqwest::Error> =
        reponse.json::<serde_json::Value>().await;
    match object {
        Ok(object) => {
            let data = &*object.get("dist").unwrap();
            return Ok(String::from(data["tarball"].as_str().unwrap()));
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
            let dev_dependencies = object.get("devDependencies");
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
            match dev_dependencies {
                Some(dependency_value) => {
                    let dependency_map = dependency_value.as_object().unwrap();
                    for (key, value) in dependency_map.iter() {
                        all_dependencies
                            .insert(String::from(key), String::from(value.as_str().unwrap()));
                    }
                }
                None => {}
            }
            return Ok(all_dependencies);
        }
        Err(_) => return Err(String::from("Error getting package")),
    };
}
