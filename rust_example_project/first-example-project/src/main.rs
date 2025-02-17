use actix_web::{web, App, HttpServer, Responder, HttpResponse};

struct AppState {
    // Defina o estado do aplicativo aqui
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        // Inicialize o estado do aplicativo aqui
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/users", web::get().to(get_users))
            // Rotas adicionais aqui
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Bem-vindo ao servidor Actix-web!")
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Lista de usuários")
}