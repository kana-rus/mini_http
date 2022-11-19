use std::fmt::Display;

use crate::{
    context::Context,
    response::Response
};


pub(crate) enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl Method {
    pub(crate) fn parse(string: &str) -> Context<Self> {
        match string {
            "GET"    => Ok(Self::GET),
            "POST"   => Ok(Self::POST),
            "PATCH"  => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            _ => Response::BadRequest(format!("invalid request method: `{string}`"))
        }
    }
    pub(crate) fn index(&self) -> usize {
        match self {
            Self::GET => 0,
            Self::POST => 1,
            Self::PATCH => 2,
            Self::DELETE => 3,
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::GET    => "GET",
            Self::POST   => "POST",
            Self::PATCH  => "PATCH",
            Self::DELETE => "DELETE",
        })
    }
}