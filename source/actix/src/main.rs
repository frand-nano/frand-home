use actix::spawn;
use anyhow::anyhow;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer};
use authorize::oauth;
use awc::Client;
use config::Config;
use lazy_static::lazy_static;
use server::Server;
use tokio::try_join;

mod config;
mod authorize;
mod route;
mod session;
mod server;
mod youtube; 
mod mysql;

lazy_static! {    
    static ref CONFIG: Config = Config::read_from(std::env::args(), "config").unwrap();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let log4rs_path = &CONFIG.paths.log4rs;
    
    log4rs::init_file(
        log4rs_path, 
        Default::default(),
    ).map_err(|err| anyhow!("Failed to read log4rs.yml file log4rs_path: {log4rs_path} err: {err}"))?;

    log::info!("🚀 start server");

    let tls_server_config = CONFIG.read_tls_server_config()?;   
    let session_secret = CONFIG.session_secret()?;   
     
    let (server, message_sender) = Server::new().await?;
    let server_handle = spawn(server.run());

    let ip = match CONFIG.settings.local_mode {
        true => "127.0.0.1",
        false => "0.0.0.0",
    };

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Client::default()))
            .app_data(Data::new(message_sender.clone()))
            .wrap(Logger::default())  
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(), 
                    Key::from(session_secret.as_slice()),
                ).build()
            )
            .service(web::resource("/login").to(route::get_login))
            .service(route::get_index)
            .service(route::get_index_favicon)
            .service(route::get_index_path)
            .service(route::get_res_path)
            .service(oauth::get_oauth)
            .service(route::get_ws)
            .default_service(web::route().to(|_:HttpRequest| HttpResponse::NotFound()))
    });    

    let http_server = match CONFIG.settings.local_mode {
        true => http_server.bind(
            ("localhost", CONFIG.settings.port()),
        ),
        false => http_server.bind_rustls_0_22(
            ("0.0.0.0", CONFIG.settings.port()), 
            CONFIG.read_tls_server_config()?,
        ),
    };

    let http_server = http_server?.run();

    let server_handle = async move { 
        server_handle.await
        .map_err(|err| std::io::Error::from(err)) 
    };
    
    try_join!(http_server, server_handle)?.1
}