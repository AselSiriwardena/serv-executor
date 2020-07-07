use actix_web::{HttpServer, web, HttpResponse};

mod executor {
    pub mod validate_password;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger:: init_with_level(log::Level::Info).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/serv/")
                    .service(executor::validate_password::validate_password)
            )
    })
        .workers(10)
        .keep_alive(15)
        .bind("127.0.0.1:8088")?
        .run()
        .await
}