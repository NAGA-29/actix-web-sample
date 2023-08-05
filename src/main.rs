use actix_web::{ App, HttpServer, middleware, web }
use tera::Tera;

#[actix_web::main]
async fn main()  -> std::io::Result<()>{
    // ロガーを初期化
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // HttpServerの起動
    HttpServer::new(move || {
        // 提供するサービスの登録
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
    }).bind("127.0.0.1:8080")?.workers(2).run().await
    println!("Hello, world!");
}
