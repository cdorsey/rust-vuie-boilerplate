use actix_web::{get, web, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ResponseBody<S>
where
    S: ToString,
{
    message: S,
}

#[get("/api/hello")]
async fn hello() -> Result<impl Responder> {
    Ok(web::Json(ResponseBody {
        message: "Hello, world!",
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let debug = std::env::var("DEBUG").is_ok();
    let bind_port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    let bind_host = if debug { "127.0.0.1" } else { "0.0.0.0" };

    let app = || {
        let assets_dir = std::env::var("ASSETS_DIR").unwrap_or("./client/dist".to_string());

        actix_web::App::new()
            .service(hello)
            .service(actix_files::Files::new("/", assets_dir).index_file("index.html"))
    };

    HttpServer::new(app)
        .bind((bind_host, bind_port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(actix_web::App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/api/hello").to_request();
        let resp = test::call_service(&app, req).await;
        let body = test::read_body(resp).await;

        let expected = serde_json::to_string(&ResponseBody {
            message: "Hello, world!",
        })
        .unwrap();

        assert_eq!(body, expected);
    }
}
