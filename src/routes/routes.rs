use actix_web::{web,Scope};
use actix_web::web::{delete, get};
use crate::controllers::user_controller::{create_user_controller,
                                          delete_user_controller,
                                          get_all_users_controller,
                                          update_user_controller
};

pub fn user_route() ->Scope{
    web::scope("/users")
        .route("/create",web::post().to(create_user_controller))
        .route("/update/{id}",web::put().to(update_user_controller))
        .route("/get-all",get().to(get_all_users_controller))
        .route("/delete/{id}",delete().to(delete_user_controller))
}