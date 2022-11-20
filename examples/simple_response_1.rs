use mini_http::{
    request::Request,
    context::Context,
    response::Response,
    components::json::JSON, server::Server
};


fn main() -> Context<()> {
    Server::setup()
        .GET("/", hello)
        .serve_on(":3000")
}

fn hello(_: Request) -> Context<Response> {
    Response::OK(
        JSON::from("hello!")
    )
}