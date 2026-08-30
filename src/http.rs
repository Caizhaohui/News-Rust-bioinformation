use std::time::Duration;

use serde_json::Value;

pub trait HttpClient {
    fn get_json(
        &self,
        url: &str,
        query: &[(&str, String)],
        headers: &[(&str, String)],
    ) -> Result<(u16, Value), String>;

    fn post_json(
        &self,
        url: &str,
        body: &Value,
        headers: &[(&str, String)],
    ) -> Result<(u16, Value), String>;
}

pub struct ReqwestClient {
    inner: reqwest::blocking::Client,
}

impl ReqwestClient {
    pub fn new() -> Result<Self, String> {
        let inner = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|err| err.to_string())?;
        Ok(Self { inner })
    }
}

impl HttpClient for ReqwestClient {
    fn get_json(
        &self,
        url: &str,
        query: &[(&str, String)],
        headers: &[(&str, String)],
    ) -> Result<(u16, Value), String> {
        let mut req = self.inner.get(url);
        if !query.is_empty() {
            req = req.query(&query);
        }
        for (key, value) in headers {
            req = req.header(*key, value);
        }
        let response = req.send().map_err(|err| err.to_string())?;
        let status = response.status().as_u16();
        let body = response.json().unwrap_or(Value::Null);
        Ok((status, body))
    }

    fn post_json(
        &self,
        url: &str,
        body: &Value,
        headers: &[(&str, String)],
    ) -> Result<(u16, Value), String> {
        let mut req = self.inner.post(url).json(body);
        for (key, value) in headers {
            req = req.header(*key, value);
        }
        let response = req.send().map_err(|err| err.to_string())?;
        let status = response.status().as_u16();
        let body = response.json().unwrap_or(Value::Null);
        Ok((status, body))
    }
}

#[derive(Default)]
pub struct MockHttp {
    pub calls: std::cell::RefCell<Vec<String>>,
    pub status: u16,
    pub body: Value,
}

impl MockHttp {
    pub fn new(status: u16, body: Value) -> Self {
        Self {
            calls: std::cell::RefCell::new(Vec::new()),
            status,
            body,
        }
    }
}

impl HttpClient for MockHttp {
    fn get_json(
        &self,
        url: &str,
        _query: &[(&str, String)],
        _headers: &[(&str, String)],
    ) -> Result<(u16, Value), String> {
        self.calls.borrow_mut().push(url.to_string());
        Ok((self.status, self.body.clone()))
    }

    fn post_json(
        &self,
        url: &str,
        _body: &Value,
        _headers: &[(&str, String)],
    ) -> Result<(u16, Value), String> {
        self.calls.borrow_mut().push(url.to_string());
        Ok((self.status, self.body.clone()))
    }
}
