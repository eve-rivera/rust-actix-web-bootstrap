use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_files as fs;

mod templates;
mod state;


#[get("/")]
async fn hello(data: web::Data<state::AppState>) -> Result<impl Responder, Error> {
    let app_name = &data.app_name;
    let mut ctx = tera::Context::new();
    ctx.insert("app_name", app_name);
    ctx.insert("text", "Welcome!");

    let html = data.templates.render("index.html", &ctx)
        .map_err(|e| {
            let ms = e.to_string();
            println!("{ms}");
            error::ErrorInternalServerError("Template error")
        })?;

    Ok(web::Html::new(html))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 8080;
    const APP_NAME: &str = "Rust Actix Bootstrap";

    let http_server = HttpServer::new(|| {
        let state = state::AppState {
            app_name: String::from(APP_NAME),
            templates: templates::load(),
        };

        let app = App::new()
            .app_data(web::Data::new(state))
            .service(fs::Files::new("/static", "static"))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello));
        app
    });

    http_server.bind((HOST, PORT))
        .map(|server| {
            println!("Running server at: http://{HOST}:{PORT}");
            server
        })?
        .run()
        .await
}
