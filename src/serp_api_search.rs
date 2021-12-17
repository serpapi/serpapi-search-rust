#![allow(warnings)]
#![allow(dead_code)]

//use std::error::Error;
use std::collections::HashMap;
//use reqwest::get;
// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }
// search engine supported by Serp Api
static GOOGLE_ENGINE: &'static str = "google";
static BAIDU_ENGINE: &'static str = "baidu";
static BING_ENGINE: &'static str = "bing";
static DUCKDUCKGO_ENGINE: &'static str = "duckduckgo";
static YAHOO_ENGINE: &'static str = "yahoo";
static YANDEX_ENGINE: &'static str = "yandex";
static EBAY_ENGINE: &'static str = "ebay";
static YOUTUBE_ENGINE: &'static str = "youtube";
static WALMART_ENGINE: &'static str = "walmart";
static HOMEDEPOT_ENGINE: &'static str = "home_depot";
static APPLE_STORE_ENGINE: &'static str = "apple_app_store";
static NAVER_ENGINE: &'static str = "naver";

pub struct SerpApiSearch {
  engine: String,
  params: HashMap<String, String>,
  key: String,
}

impl SerpApiSearch {

  
 pub fn new(engine: String,  params: HashMap<String, String>, key: String) -> SerpApiSearch {
   SerpApiSearch{
     engine: engine,
     params: params,
     key: key,
   }
 }

 pub fn google(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: GOOGLE_ENGINE.to_string(),
    params: params,
    key: key,
  }
}


pub fn baidu(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: BAIDU_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn bing(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: BING_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn duckduckgo(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: DUCKDUCKGO_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn yahoo(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: YAHOO_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn yandex(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: YANDEX_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn ebay(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: EBAY_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn youtube(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: YOUTUBE_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn walmart(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: WALMART_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn homedepot(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: HOMEDEPOT_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn naver(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: NAVER_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

pub fn apple(params: HashMap<String, String>, key: String) -> SerpApiSearch {
  SerpApiSearch{
    engine: APPLE_STORE_ENGINE.to_string(),
    params: params,
    key: key,
  }
}

 pub async fn search(&self) -> Result<String, Box<dyn std::error::Error>> {
    let host = "http://serpapi.com/search".to_string(); 

    // concatenate the argument
    let mut arg = "&source=rust&engine=".to_string();
    arg.push_str(&self.engine);
    arg.push_str("&api_key=");
    arg.push_str(&self.key);
    for (k, v) in self.params.iter() {
      arg.push_str("&");
      arg.push_str(k);
      arg.push_str("=");
      arg.push_str(v);
    }
    let mut url = host;
    url.push_str("?");
    url.push_str(&arg);

    // Await the response...
    let res = reqwest::get(url).await?;
    //TODO error handling if status != 200
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    Ok(body)
 }

 pub async fn json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
   let body = self.search().await?;
    println!("Body:\n{}", body);
    let value: serde_json::Value = serde_json::from_str(&body).unwrap();
    Ok(value)
 }

}
