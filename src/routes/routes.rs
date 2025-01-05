use actix_web::web:: ServiceConfig;
use crate::controllers::user_controller::{create_user_controller,delete_user_controller,
                                         get_all_users_controller, update_user_controller};

pub fn user_route(cfg:&mut ServiceConfig){

    cfg.service(create_user_controller);
    cfg.service(update_user_controller);
    cfg.service(get_all_users_controller);
    cfg.service(delete_user_controller);

}