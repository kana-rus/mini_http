use std::{
    net::TcpStream,
    io::Write
};
use crate::{
    context::Context,
    components::{
        status::Status,
        json::JSON
    },
};


#[derive(Debug)]
pub struct Response {
    status: Status,
    body:   Body,
}
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum Body {
        json(JSON),
        text(String),
    } impl Body {
        fn content_length(&self) -> usize {
            match self {
                Self::json(json) => json.content_length(),
                Self::text(text) => text.len(),
            }
        }
        
    }

pub(crate) trait ResponseFormat {
    fn response_format(&self) -> &str;
}
impl ResponseFormat for Body {
    fn response_format(&self) -> &str {
        match self {
            Self::json(json) => json.response_format(),
            Self::text(text) => text.as_str(),
        }
    }
}

impl Response {
    pub(crate) fn write_to_stream(self, stream: &mut TcpStream) -> std::io::Result<usize> {
        stream.write(format!(
"HTTP/1.1 {}
Content-Type: {}; charset=utf-8
Content-Length: {}
Connection: Keep-Alive
Keep-Alive: timeout=5

{}",
            self.status.response_format(),
            self.status.content_type(),
            self.body.content_length(),
            self.body.response_format(),
        ).as_bytes())
    }


    #[allow(non_snake_case)]
    pub(crate) fn SetUpError(messages: &Vec<String>) -> Context<()> {
        Err(Self {
            status: Status::SetUpError,
            body:   Body::text(messages.iter().fold(
                String::new(), |a, b| a + b + "\n"
            ))
        })
    }

    #[allow(non_snake_case)]
    pub fn OK(body: JSON) -> Context<Self> {
        Ok(Self {
            status:  Status::OK,
            body: Body::json(body),
        })
    }
    #[allow(non_snake_case)]
    pub fn NotFound<Msg: ToString, T>(msg: Msg) -> Context<T> {
        Err(Self {
            status: Status::NotFound,
            body:   Body::text(msg.to_string()),
        })
    }
    #[allow(non_snake_case)]
    pub fn BadRequest<Msg: ToString, T>(msg: Msg) -> Context<T> {
        Err(Self {
            status:  Status::BadRequest,
            body:    Body::text(msg.to_string()),
        })
    }
    #[allow(non_snake_case)]
    pub fn InternalServerError<Msg: ToString, T>(msg: Msg) -> Context<T> {
        Err(Self {
            status:  Status::InternalServerError,
            body:    Body::text(msg.to_string()),
        })
    }
    #[allow(non_snake_case)]
    pub fn NotImplemented<Msg: ToString, T>(msg: Msg) -> Context<T> {
        Err(Self {
            status:  Status::NotImplemented,
            body:    Body::text(msg.to_string()),
        })
    }
}
