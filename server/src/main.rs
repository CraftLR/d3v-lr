use axum::{
  body::Body,
  http::{header, HeaderMap},
  response::{Html, Response},
  routing::{get, post},
  Json, Router,
};
use md::MdToHtml;
use serde::Serialize;

mod md;

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    // 1st challenge : post a letter to our Boite Au lettres
    .route("/bal", post(bal))
    // for the lol
    .route("/wp_admin", get(you_shall_not_pass));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<String> {
  let md = "**Hello, World!** but with Markdown!".md_to_html();
  format!(
    r#"
    <!DOCTYPE html>
    <html>
      <head>
        <title>Markdown to HTML</title>
      </head>
      <body>
        {md}
      </body>
    </html>
    "#,
  )
  .into()
}

async fn bal() -> Json<User> {
  let user = User { id: 1337, username: "plop".to_string() };
  Json(user)
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
  id: u64,
  username: String,
}

const WEBP: &[u8] = include_bytes!("../../you_shall_not_pass.webp");
async fn you_shall_not_pass() -> Response<Body> {
  let mut headers = HeaderMap::new();
  headers.insert(header::CONTENT_TYPE, "image/webp".parse().unwrap());

  let body = Body::from(WEBP);

  Response::builder()
    .status(200)
    .header("Content-Type", "image/webp")
    .body(body)
    .unwrap()
}
