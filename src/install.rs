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


pub async fn get_tarball_download_link(dependency:&String, version:&String)->Result<String, String>{
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