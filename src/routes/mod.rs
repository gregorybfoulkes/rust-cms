mod posts;
mod users;

use actix_web::{web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(posts::create_post)
            .service(posts::get_posts)
            .service(users::create_user)
            .service(users::get_users),
    );
}