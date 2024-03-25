use std::sync::{OnceLock};
use std::time::Duration;
use reqwest::blocking::Client;
use serde_json::{Map, Value};
use crate::util::error::macros::{network_error, remote_error};
use crate::util::error::{Result, DTError};

#[cfg(all(feature = "network"))]
#[derive(Debug)]
pub(crate) struct HttpService {
    client: Client,
}

#[cfg(all(feature = "network"))]
impl HttpService {
    fn new() -> Self {
        HttpService {
            client: Client::builder()
                .timeout(Duration::from_millis(3000))
                .build()
                .unwrap()
        }
    }

    pub fn get() -> &'static Box<Self> {
        static MEM: OnceLock<Box<HttpService>> = OnceLock::new();
        MEM.get_or_init(|| {
            Box::new(HttpService::new())
        })
    }

    pub fn post_event(
        self: &'static Box<Self>,
        url: &String, data: String,
        app_id: &String, data_count: usize, token: &String,
        sdk_type: &String, sdk_version: &String
    ) -> Result<Map<String, Value>> {
        let response = self.client
            .post(url)
            .header("app_id", app_id)
            .header("data-count", data_count)
            .header("DT-type", sdk_type)
            .header("sdk-version", sdk_version)
            .header("token", token)
            .body(data)
            .send();

        match response {
            Ok(response) => {
                let status_code = response.status();
                if !status_code.is_success() {
                    network_error!("Upload failed with status code: \"{}\"", status_code)
                } else {
                    match response.json::<Map<String, Value>>() {
                        Ok(response) => Ok(response),
                        Err(e) => remote_error!("Failed to parse network response!\n\tStatus code: {},\n\tReason: {}", status_code, e)
                    }
                }
            },
            Err(e) => {
                network_error!("Network Failed! reason: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::upload::http_service::HttpService;

    #[test]
    fn it_works() {
        unsafe {
            crate::util::logger::LOG_ENABLED = true;
        }
        let hs = HttpService::get();
        let response = hs.post_event(
            &String::from("https://baidu.com/"/*"https://test.roiquery.com/sync"*/),
            String::from("[]"),
            &String::from(""),
            0,
            &String::from(""),
            &String::from(""),
            &String::from("")
        );
        match response {
            Ok(response) => println!("{response:?}"),
            Err(e) => println!("{e}")
        }
    }
}