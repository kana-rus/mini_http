use std::{
    collections::HashMap,
    net::TcpListener,
    io::{Read, Write}
};
use crate::{
    components::{method::Method, consts::BUF_SIZE},
    response::Response,
    request::Request,
    context::Context,
    utils::{parse::parse_stream, validation::construct_tcp_address},
};


pub struct Server<'method, 'path>(
    HashMap<
        (&'method Method, &'path str, bool),
        fn(Request) -> Context<Response>,
    >
);
pub struct ServerSetting<'method, 'path> {
    map: HashMap<
        (&'method Method, &'path str, bool),
        fn(Request) -> Context<Response>,
    >,
    errors: Vec<String>,
}


impl<'method, 'path> ServerSetting<'method, 'path> {
    pub fn serve_on(&self, address: &'static str) -> Context<()> {
        if !self.errors.is_empty() {
            return Response::SetUpError(&self.errors)
        }
        let server = Server(self.map.clone());
        let tcp_address = construct_tcp_address(address);
        server.serve_on(tcp_address)
    }

    #[allow(non_snake_case)]
    pub fn GET(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(&Method::GET, path_string, handler)
    }
    #[allow(non_snake_case)]
    pub fn POST(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(&Method::POST, path_string, handler)
    }
    #[allow(non_snake_case)]
    pub fn PATCH(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(&Method::PATCH, path_string, handler)
    }
    #[allow(non_snake_case)]
    pub fn DELETE(&mut self,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        self.add_handler(&Method::DELETE, path_string, handler)
    }

    fn add_handler(&mut self,
        method:      &'method Method,
        path_string: &'static str,
        handler:     fn(Request) -> Context<Response>,
    ) -> &mut Self {
        // ===============================================================
        // TODO: vaidate path string here
        // ===============================================================

        let (path, has_param) =
            if let Some((path, _param_name)) = path_string.rsplit_once("/:") {
                (path, true)
            } else {
                (path_string, false)
            };

        
        if self.map.insert(
            (method, &path, has_param), handler
        ).is_some() {
            self.errors.push(format!("handler for `{method} {path_string}` is resistered duplicatedly"))
        }

        self
    }
}
impl<'method, 'path> Server<'method, 'path> {
    pub fn setup() -> ServerSetting<'method, 'path> {
        ServerSetting {
            map:    HashMap::new(),
            errors: vec![]
        }
    }
    fn serve_on(&self, tcp_address: String) -> Context<()> {
        let listener = TcpListener::bind(tcp_address)?;

        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [b' '; BUF_SIZE];
            stream.read(&mut buffer)?;

            let (method, path_str, request) = parse_stream(&buffer)?;
            match self.handle_request(method, path_str, request) {
                Ok(res)  => res,
                Err(res) => res,
            }.write_to_stream(
                &mut stream
            )?;
            stream.flush()?
        }

        Ok(())
    }

    fn handle_request(&self,
        method:      Method,
        path_str:    &'path str,
        mut request: Request<'path>,
    ) -> Context<Response> {
        let handler = 
                if let Some(handler) = self.0.get(&(&method, path_str, false)) {
                    handler
                } else {
                    let (path, param) = path_str.rsplit_once('/')
                        .ok_or_else(|| Response::BadRequest(format!(
                            "invalid request path format: {path_str}"
                        )))?;
                    let handler = self.0.get(&(&method, path, true))
                        .ok_or_else(||
                            if let Some(_) = self.0.get(&(&method, path_str, true)) {
                                Response::BadRequest(format!(
                                    "expected a path parameter"
                                ))
                            } else {
                                Response::NotFound(format!(
                                    "handler for `{method} {path_str}` is not found"
                                ))
                            }
                        )?;
                    request.param = Some(param);
                    handler
                };
        handler(request)
    }
}
