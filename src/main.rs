use std::error;

use axum::{Router, response::Html, routing::get};
use maud::{Markup, Render, html};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use tokio;

async fn handler() -> Markup {
    html! {
        link rel="stylesheet" type="text/css" href="assets/css/style.css";

        h1 { "Hello, world!" }

        p.intro {
            "This is an example of the "
            a href="https://github.com/lambda-fairy/maud" { "Maud" }
            " template language."
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app = Router::new().route("/", get(handler)).nest_service(
        "/assets",
        ServiceBuilder::new().service(ServeDir::new("assets")),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
