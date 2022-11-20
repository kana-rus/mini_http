use mini_http::{
    JSON,
    Server,
    Context,
    Request,
    Response,
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