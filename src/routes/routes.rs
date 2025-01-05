use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::controllers::user_controller::{create_user_controller, delete_user_controller, get_all_users_controller, update_user_controller};

pub fn user_route(cfg:&mut ServiceConfig){

    cfg.service(
        web::scope("/users")
            .service(create_user_controller)
            .service(update_user_controller)
            .service(get_all_users_controller)
            .service(delete_user_controller)
    );
}