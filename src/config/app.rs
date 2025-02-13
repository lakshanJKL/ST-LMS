use actix_web::web::{scope, ServiceConfig};
use crate::controllers::student_controller::{create_student_controller, delete_student_controller, get_all_students_paginate_controller, update_student_controller};
use crate::controllers::user_controller::{create_user_controller, delete_user_controller, get_all_paginate_controller, update_user_controller, user_login_controller};

pub fn app_config(cfg: &mut ServiceConfig) {

    // configuring routing *********************

    cfg
        .service(user_login_controller) // login
        .service(create_user_controller) // sign up
        .service(
            scope("/users")
                .service(update_user_controller)
                .service(delete_user_controller)
                .service(get_all_paginate_controller)
        )
        .service(
            scope("/students")
                .service(create_student_controller)
                .service(update_student_controller)
                .service(delete_student_controller)
                .service(get_all_students_paginate_controller)
        );
}