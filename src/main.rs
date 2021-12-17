mod serp_api_search;
use std::collections::HashMap;

// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

use crate::serp_api_search::SerpApiSearch;
use std::env;
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    // read secret api key from environment variable
    // To get the key simply copy/paste from https://serpapi.com/dashboard.
    let api_key = match env::var_os("API_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$API_KEY is not set")
    };

    // let's search something on google
    let mut params = HashMap::<String, String>::new();
    params.insert("q".to_string(), "coffee".to_string());
    params.insert("location".to_string(), "Austin, TX".to_string());

    // generic search by setting the search engine
    //let engine = "google".to_string();
   // let client = SerpApiSearch::new(engine, params, api_key);

    // get the search engine
    let client = SerpApiSearch::google(params, api_key);
    // search returns text
    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    let response = client.json().await?;
    let organic_results = response["organic_results"].as_array().unwrap();
    println!("number of organic results: {}", organic_results.len());
    println!("organic_results content: {}", response["organic_results"]);
    Ok(())
}

// async fn RunSearch() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
//     let res = reqwest::get("http://httpbin.org/get").await?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//     let value: serde_json::Value = serde_json::from_str(&body).unwrap();
//     println!("Value:\n{}", value["headers"]);
//     Ok(value)
// }

// WORKING example below
// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let res = reqwest::get("http://httpbin.org/get").await?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//     let value: serde_json::Value = serde_json::from_str(&body).unwrap();
//     println!("Value:\n{}", value["headers"]);
//     Ok(())
// }
