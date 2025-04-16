use capnp::message::Builder;
use std::collections::HashMap;

use crate::{capnp_get_text, network_capnp};

#[derive(Debug, Clone, Copy, Default)]
pub enum NetworkMethod {
    #[default]
    Get,
    Post,
}

#[derive(Debug, Default)]
pub struct RequestOpts {
    pub method: NetworkMethod,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

#[derive(Debug, Default)]
pub struct NetworkResponse {
    pub status_code: i32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl RequestOpts {
    pub fn to_capnp(&self) -> Result<Vec<u8>, capnp::Error> {
        let mut message = Builder::new_default();
        let mut request = message.init_root::<network_capnp::network_request::Builder>();

        let method = match self.method {
            NetworkMethod::Get => network_capnp::NetworkMethod::Get,
            NetworkMethod::Post => network_capnp::NetworkMethod::Post,
        };

        request.set_method(method);
        request.set_url(&self.url);

        // Set headers
        if let Some(headers) = &self.headers {
            let mut header_list = request.reborrow().init_headers(headers.len() as u32);
            for (i, (key, value)) in headers.iter().enumerate() {
                let mut header = header_list.reborrow().get(i as u32);
                header.set_key(key);
                header.set_value(value);
            }
        }

        request.set_body(self.body.as_deref().unwrap_or("").as_bytes());

        let mut buffer = vec![];
        let mut cursor = std::io::Cursor::new(&mut buffer);
        capnp::serialize::write_message(&mut cursor, &message)?;

        Ok(buffer)
    }
}

impl From<network_capnp::network_response::Reader<'_>> for NetworkResponse {
    fn from(value: network_capnp::network_response::Reader<'_>) -> Self {
        let status_code = value.get_status();
        let mut headers = HashMap::new();

        if let Ok(header_list) = value.get_headers() {
            for header in header_list.iter() {
                let key = capnp_get_text!(header.get_key());
                let value = capnp_get_text!(header.get_value());
                headers.insert(key.to_string(), value.to_string());
            }
        }

        let body = value.get_body().unwrap_or_default().to_vec();

        NetworkResponse {
            status_code,
            headers,
            body,
        }
    }
}

impl NetworkResponse {
    pub fn read_from_memory(ptr: u32, len: u32) -> Result<Self, crate::error::Error> {
        let response = crate::capnp_message_to_type!(
            ptr,
            len,
            network_capnp::network_response::Reader,
            NetworkResponse
        )?;
        Ok(response)
    }
}
