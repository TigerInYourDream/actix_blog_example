use jsonwebtoken as jwt;

mod api;
mod handler;
mod middlewares;
mod routes;

fn main() {
    //setting log
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix_web::actix::System::new("actix_blog");
    let port = "127.0.0.1:9096";
    actix_web::server::new(|| routes::app())
        .bind(port)
        .unwrap()
        .start();
    println!("start server at: {}", port);
    let _ = sys.run();
}
