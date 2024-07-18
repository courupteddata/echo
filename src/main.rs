use axum::{response::Html, routing::get, routing::post, Router, Form};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().
        route("/", get(show_form)).
        route("/echo_user_input", post(echo_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(r#"
    <form action="/echo_user_input" method="POST">
         <input name="user_input">
         <input type="submit" value="Submit!">
     </form>
    "#)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
}

async fn echo_handler(Form(input): Form<Input>) -> Response {
    dbg!(&input);
    return input.user_input.into_response();
}

