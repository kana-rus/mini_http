use mini_http::prelude::*;

fn main() -> Context<()> {
    Server::setup()
        .GET("/hello/:name", hello_to_someone)
        .serve_on(":3000")
}

fn hello_to_someone(req: Request) -> Context<Response> {
    let name = req.param
        .ok_or_else(|| Response::BadRequest("expected a path parameter, but it's not found"))?;
    Response::OK(
        JSON::from(format!("Hello, {name}!"))
    )
}