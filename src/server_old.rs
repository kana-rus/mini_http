use std::{net::TcpListener, io::{Read, Write}};

use regex::{RegexSet, Regex};
use crate::{
    request::Request,
    context::Context,
    response::Response, components::{method::Method, consts::BUF_SIZE, json::JSON}
};


pub struct Server<'s> {
    paths:     RegexSet,
    raw_paths: &'s [Regex],
    handlers:  &'s [[Option<fn(Request) -> Context<Response>>; 4]],
    // GET, POST, PATCH, DELETE
    //  0     1     2       3
}
pub struct ServerSetting {
    paths:    Vec<Regex>,
    handlers: Vec<[Option<fn(Request) -> Context<Response>>; 4]>,
    errors:   Vec<String>,
}

impl<'s> Server<'s> {
    pub fn setup() -> ServerSetting {
        ServerSetting {
            paths:    vec![],
            handlers: vec![],
            errors:   vec![],
        }
    }
    fn build(setting: &'s mut ServerSetting) -> Context<Self> {
        if !setting.errors.is_empty() {
            return Response::SetUpError(&setting.errors)
        }

        let mut paths = vec![];
        for r in &setting.paths {
            paths.push(r.as_str())
        }

        Ok(Self {
            paths:     RegexSet::new(&paths).unwrap(/* based on pre validating */),
            raw_paths: setting.paths.as_slice(),
            handlers:  setting.handlers.as_slice(),
        })
    }
    fn serve_on(self, tcp_address: String) -> Context<()> {
        let listener = TcpListener::bind(tcp_address)?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [b' '; BUF_SIZE];

            stream.read(&mut buffer)?;
            let (method, path, request) = parse_stream(&mut buffer)?;

            // based on pre validating ...
            let Some(matched_path_index) = self.paths.matches(path).into_iter().next()
                else {return Response::NotFound(format!("handler for that request is not found"))}; 
            let matched_path = &self.raw_paths[matched_path_index];
            // let params = matched_path.



            
                todo!()



            
            ;
            let Some(handler) = self.handlers[matched_path_index][method.index()]
                else {return Response::NotFound(format!("handler for that request is not found"))};

            match handler(request) {
                Ok(res)  => res,
                Err(res) => res,
            }.write_to_stream(&mut stream)?;
            stream.flush()?;
        }
        Ok(())
    }
}
impl ServerSetting {
    pub fn serve_on(&mut self, address: &'static str) -> Context<()> {
        let tcp_address = construct_tcp_address(address);

        let server = Server::build(self)?;
        server.serve_on(tcp_address)
    }

    pub fn GET(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(Method::GET, path_string, handler)
    }
    pub fn POST(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(Method::POST, path_string, handler)
    }
    pub fn PATCH(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(Method::PATCH, path_string, handler)
    }
    pub fn DELETE(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(Method::DELETE, path_string, handler)
    }

    fn add_handler(&mut self,
        method:      Method,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        let mut existing_path_index = None;
        for i in 0..self.paths.len() {
            if self.paths[i].is_match(path_string) {
                existing_path_index = Some(i);
                let duplication_error_message = format!(
                    "handler for paths matching `{method} {path_string}` are resistered DUPLICATEDLY.",
                );
                if self.handlers[i][method.index()].is_some() {
                    self.errors.push(duplication_error_message)
                }
                break;
            }
        }

        // ============================================================
        // maybe move to another place
        const PARAM_PATTERN: Regex = Regex::new(r":\w{1,}/|:\w{1,}$").unwrap();
        // ============================================================

        if let Some(index) = existing_path_index {
            self.handlers[index][method.index()] = Some(handler)

        } else {
            // ======================================
            // TODO: validate path_string here
            // - format: (/:?[a-z,A-Z,_,-]{1,}){1,}
            // - no duplicatedly named param
            // ======================================
            let param_matches = PARAM_PATTERN.find_iter(path_string);
            let mut path_string = path_string.to_owned();
            for param_match in param_matches {
                let param_name = param_match.as_str().trim_start_matches(':').trim_end_matches('/');
                let param_part_format = format!(":{param_name}");
                let param_regex_format = format!("(?P<{param_name}>\\w{{1,}})");
                path_string = path_string.replace(
                    &param_part_format, &param_regex_format
                )
            }
            self.paths.push(Regex::new(&path_string).unwrap());

            let mut handler_set = [None, None, None, None];
            handler_set[method.index()] = Some(handler);
            self.handlers.push(handler_set);
        }
            
        self
    }
}


// TODO: validate with regex
fn construct_tcp_address(string: &'static str) -> String {
    if string.starts_with(":") {
        "127.0.0.1".to_owned() + string
    } else if string.starts_with("localhost") {
        string.replace("localhost", "127.0.0.1")
    } else {
        string.to_owned()
    }
}

fn parse_stream(
    buffer: &[u8; BUF_SIZE]
) -> Context<(Method, &str, Request)> {
    let mut lines = std::str::from_utf8(buffer)?
        .trim_end()
        .lines();

    let request_line = lines.next().ok_or_else(|| Response::BadRequest::<&str, ()>("empty request").unwrap_err())?;
    let (method, path) = parse_request_line(request_line)?;

    while let Some(line) = lines.next() {
        if line.is_empty() {break}
    }

    let request = Request {
        body:
            if let Some(request_body) = lines.next() {
                Some(JSON::from_str_unchecked(request_body))
            } else {None}
    };

    Ok((method, path, request))
}

fn parse_request_line(line: &str) -> Context<(Method, &str)> {
    if line.is_empty() {
        return Response::BadRequest("can't find request status line")
    }
    let (method, path) = line
        .strip_suffix(" HTTP/1.1")
        .ok_or_else(|| Response::NotImplemented::<&str, ()>("I can't handle protocols other than `HTTP/1.1`").unwrap_err())?
        .split_once(' ')
        .ok_or_else(|| Response::BadRequest::<&str, ()>("invalid request line format").unwrap_err())?;
    Ok((Method::parse(method)?, path))
}