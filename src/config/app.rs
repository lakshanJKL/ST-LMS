use actix_web::web;
use actix_web::web::{ServiceConfig};
use crate::controllers::user_controller::{create_user_controller, delete_user_controller, get_all_users_controller, update_user_controller, user_login_controller};

pub fn app_config(cfg: &mut ServiceConfig) {

    // configuring routing *********************

    cfg
        .service(user_login_controller)  // login
        .service(create_user_controller) // signup
        .service(
            web::scope("/users")
                .service(update_user_controller)
                .service(get_all_users_controller)
                .service(delete_user_controller)
        );
}