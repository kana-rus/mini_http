use regex::RegexSet;
use crate::{
    request::Request,
    context::Context,
    response::Response, components::method::Method
};


pub struct Server<const N: usize> {
    paths:    RegexSet,  // including path info
    handlers: [[Option<fn(Request) -> Context<Response>>; 4]; N],
    // GET, POST, PATCH, DELETE
    //  0     1     2       3
}
pub struct ServerSetting {
    paths:    Vec<&'static str>,
    handlers: Vec<[Option<fn(Request) -> Context<Response>>; 4]>,
    errors:   Vec<String>,
}

impl<const N: usize> Server<N> {
    pub fn setup() -> ServerSetting {
        ServerSetting {
            paths:    vec![],
            handlers: vec![],
            errors:   vec![],
        }
    }
}
impl ServerSetting {
    fn add_handler(&mut self,
        method:      Method,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut self {
        for i in 0..self.paths.len() {
            if path_string == self.paths[i]
            && self.handlers[i][method as isize as usize].is_some() {
                self.errors.push(format!(
                    "handler for `{method} {path_string}` is already resister",
                ))
            }
        }


        todo!()
    }
}