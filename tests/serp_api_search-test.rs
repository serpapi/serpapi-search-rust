//#[cfg(test)]
#![cfg(not(target_arch = "wasm32"))]

fn api_key() -> String {
    let api_key = match std::env::var_os("API_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$API_KEY is not set"),
    };
    return api_key;
}

#[tokio::test]
async fn json() {
    let mut params = std::collections::HashMap::<String, String>::new();
    params.insert("q".to_string(), "coffee".to_string());
    params.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );

    // initialize the search engine
    let search = serpapi_search_rust::serp_api_search::SerpApiSearch::google(params, api_key());

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    let results = search.json().await.expect("request");
    let organic_results = results["organic_results"].as_array().unwrap();
    assert!(organic_results.len() > 1);

    let places = results["local_results"]["places"].as_array().unwrap();
    assert!(places.len() > 0);

}

#[tokio::test]
async fn html() {
    let mut params = std::collections::HashMap::<String, String>::new();
    params.insert("q".to_string(), "coffee".to_string());
    params.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );

    // initialize the search engine
    let search = serpapi_search_rust::serp_api_search::SerpApiSearch::google(params, api_key());

    let raw = search.html().await.expect("request");
    assert!(raw.len() > 100);
}
