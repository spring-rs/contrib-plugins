use spring::{App, auto_config};
use spring_utoipa::UtoipaConfigurator;
use spring_utoipa::UtoipaPlugin;
use spring_utoipa::utoipa;
use spring_utoipa::utoipa::OpenApi;
use spring_utoipa::utoipauto;
use spring_web::{WebConfigurator, WebPlugin, axum::response::IntoResponse, extractor::Path};
use spring_web::{get, route};

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

// Main function entry
#[auto_config(WebConfigurator)] // auto config web router
#[tokio::main]
async fn main() {
    App::new()
        .use_config_file("./examples/with-rapidoc/app.toml")
        .add_plugin(WebPlugin)
        .add_plugin(UtoipaPlugin)
        .with_openapi(ApiDoc::openapi())
        .run()
        .await
}

// The get macro specifies the Http Method and request path.
// spring-rs also provides other standard http method macros such as post, delete, patch, etc.
#[utoipa::path(get, path = "/")]
#[get("/")]
async fn hello_world() -> impl IntoResponse {
    "hello world"
}

// You can also use the route macro to specify the Http Method and request path.
// Path extracts parameters from the HTTP request path
#[utoipa::path(method(get, post), path = "/hello/{name}")]
#[route("/hello/{name}", method = "GET", method = "POST")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}
