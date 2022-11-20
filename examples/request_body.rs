use mini_http::prelude::*;
use serde::{Serialize, Deserialize};

fn main() -> Context<()> {
    Server::setup()
        .POST("/api/users/signup", post_user)
        .serve_on(":8080")
}

#[derive(Serialize, Deserialize)]
struct User {
    name:     String,
    password: String,
}

fn post_user(req: Request) -> Context<Response> {
    let new_user = req.get_body::<User>()?;
    
    // record the user info into DB...

    let created_user = new_user;

    Response::OK(
        JSON::from_struct(&created_user)?
    )
}