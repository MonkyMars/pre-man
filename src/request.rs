use crate::structs::Response;
use crate::utils::Timer;
use reqwest;


pub struct Request {
    client: reqwest::Client,
    pub method: String,
    pub url: String,
    pub response: Option<Response>,
}

impl Request {
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_string(),
            url: url.to_string(),
            response: None,
						client: reqwest::Client::new(),
        }
    }

    fn get(&self) -> reqwest::RequestBuilder {
        self.client.get(&self.url)
    }

    fn post(&self) -> reqwest::RequestBuilder {
        self.client.post(&self.url)
    }

    fn put(&self) -> reqwest::RequestBuilder {
        self.client.put(&self.url)
    }

    fn delete(&self) -> reqwest::RequestBuilder {
        self.client.delete(&self.url)
    }

    fn patch(&self) -> reqwest::RequestBuilder {
        self.client.patch(&self.url)
    }

    pub async fn send(&mut self) -> Result<(), reqwest::Error> {
        let mut timer = Timer::start();
        let request_builder = match self.method.as_str() {
            "GET" => self.get(),
            "POST" => self.post(),
            "PUT" => self.put(),
            "DELETE" => self.delete(),
            "PATCH" => self.patch(),
            _ => panic!("Unsupported HTTP method: {}", self.method),
        };
        let response = request_builder.send().await?;
				let status = response.status();
				let body = response.text().await?;
        timer.stop();
        self.response = Some(Response {
            body,
            status: httpstatus::StatusCode::from(status.as_u16()),
            duration: timer.duration,
            success: status.is_success(),
        });
        Ok(())
    }
}
