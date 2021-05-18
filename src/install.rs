// use crate::reqwest;

pub async fn get_latest_version(dependency:&String){
    let url=format!("https://registry.npmjs.org/{}", dependency);
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    println!("the response {}", body)
}