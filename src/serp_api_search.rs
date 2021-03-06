#![allow(warnings)]
#![allow(dead_code)]

//! SerpApiSearch represents a search
//!
//!

use std::collections::HashMap;

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

// model serp api search
//  because of Rust designed we propose to create a new search everytime
//   as opose of modifying the same search object over and over.
//  I noticed thar updating a HashMap is difficult in Rust. (I know).
//  I guess it's cheaper to create a new object in the stack
//   than updating a mutable object in the heap.
pub struct SerpApiSearch {
    // search engine like: google, youtube, bing...
    pub engine: String,
    // search parameter like: q=coffee for google
    pub params: HashMap<String, String>,
    // private for security reason
    key: String,
}

impl SerpApiSearch {
    pub fn new(engine: String, params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: engine,
            params: params,
            key: key,
        }
    }

    pub fn google(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: GOOGLE_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn baidu(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: BAIDU_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn bing(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: BING_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn duckduckgo(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: DUCKDUCKGO_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn yahoo(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: YAHOO_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn yandex(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: YANDEX_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn ebay(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: EBAY_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn youtube(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: YOUTUBE_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn walmart(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: WALMART_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn homedepot(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: HOMEDEPOT_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn naver(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: NAVER_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub fn apple(params: HashMap<String, String>, key: String) -> SerpApiSearch {
        SerpApiSearch {
            engine: APPLE_STORE_ENGINE.to_string(),
            params: params,
            key: key,
        }
    }

    pub async fn getResults(&self, endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
        let host = "http://serpapi.com".to_string();

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
        url.push_str(endpoint);
        url.push_str("?");
        url.push_str(&arg);

        // Await the response...
        let res = reqwest::get(url).await?;
        //TODO error handling if status != 200
        // println!("Status: {}", res.status());
        // println!("Headers:\n{:#?}", res.headers());
        let body = res.text().await?;
        Ok(body)
    }

    pub async fn getJson(
        &self,
        endpoint: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = self.getResults(endpoint).await?;
        //println!("Body:\n{}", body);
        let value: serde_json::Value = serde_json::from_str(&body).unwrap();
        Ok(value)
    }

    pub async fn json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.getJson("/search").await?;
        Ok(results)
    }

    pub async fn html(&self) -> Result<String, Box<dyn std::error::Error>> {
        let body = self.getResults("/html").await?;
        Ok(body)
    }

    // Get location using Location API
    pub async fn location(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.getJson("/locations.json").await?;
        Ok(results)
    }

    // Retrieve search result from the Search Archive API
    pub async fn search_archive(
        &self,
        search_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut endpoint = "/searches/".to_string();
        endpoint.push_str(search_id);
        endpoint.push_str(".json");
        println!(">> {}", endpoint);
        let results = self.getJson(&endpoint).await?;
        Ok(results)
    }

    // Get account information using Account API
    pub async fn account(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.getJson("/account").await?;
        Ok(results)
    }
}
