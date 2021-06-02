use actix_web::{App, get, HttpRequest, HttpServer, Responder};
use log::*;

use actix_web_custom_macro_example::{
    Foo,
    macros::foo,
    middleware::FoobarMiddleware,
};

#[get("/hello")]
#[foo(bar = "test")]
async fn hello(req: HttpRequest) -> impl Responder {
    if let Some(foo) = req.extensions().get::<Foo>() {
        debug!("hello: {:?}", foo);
    }
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("DEBUG"));
    HttpServer::new(move || {
        App::new()
            .wrap(FoobarMiddleware::new())
            .configure(|cfg| {
                cfg.service(hello);
            })
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}