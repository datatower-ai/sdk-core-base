use std::sync::{OnceLock};
use std::time::Duration;
use reqwest::blocking::Client;
use serde_json::{Map, Value};
use crate::log_error;

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
    ) -> bool {
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
                if status_code != 200 {
                    log_error!("Upload failed with status code: \"{}\"", status_code);
                    false
                } else {
                    if let Ok(response) = response.json::<Map<String, Value>>() {
                        println!("response: {:?}", response);
                        if let Some(code) = response.get("code") {
                            if code == 0 {
                                true
                            } else {
                                log_error!("Failed to upload, \"{:?}\", {:?}", code, response.get("msg").unwrap_or(&Value::String(String::new())));
                                false
                            }
                        } else {
                            log_error!("Server response is invalid, \"{:?}\"", response);
                            false
                        }
                    } else {
                        log_error!("Failed to parse response, \"{}\"", status_code);
                        false
                    }
                }
            },
            Err(e) => {
                log_error!("Network Failed! reason: {}", e);
                false
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
        hs.post_event(
            &String::from("https://test.roiquery.com/sync"),
            String::from("[]"),
            &String::from(""),
            0,
            &String::from(""),
            &String::from(""),
            &String::from("")
        );
    }
}