use env_logger::Env;
use actix_web::{ middleware::Logger, web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World"); format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| { App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
