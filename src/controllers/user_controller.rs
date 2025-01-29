use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use serde_json::json;
use validator::Validate;
use crate::midleware::permission::{Permission, Role};
use crate::models::user_model::{CreateUser, UpdateUser, UserLoginDto};
use crate::services::user_service::UserService;
use crate::utill::generic_response::GenericResponse;
use crate::utill::jwt::{has_permission_with_roles};

//login user (POST /users)
#[post("/login")]
pub async fn user_login_controller(service: Data<UserService>, user: Json<UserLoginDto>) -> HttpResponse {
    // validation
    if let Err(err) = user.validate() {
        return HttpResponse::BadRequest().body(err.to_string());
    }

    match service.user_login_service(user.into_inner()).await {
        Ok(Some(token)) => {
            let response = GenericResponse {
                code: 200,
                message: "Token created".to_string(),
                data: token,
            };

            HttpResponse::Ok().json(response)
        }
        Ok(None) => HttpResponse::Unauthorized().json(json!({ "error": "Invalid credentials" })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

//signup a new user (POST /users)
#[post("/signup")]
pub async fn create_user_controller(service: Data<UserService>, create_user: Json<CreateUser>) -> HttpResponse {
    // validation
    if let Err(err) = create_user.validate() {
        return HttpResponse::BadRequest().body(err.to_string());
    }

    match service.create_user_service(create_user.into_inner()).await {
        Ok(user) => {
            let response = GenericResponse {
                code: 200,
                message: "User has created".to_string(),
                data: user,
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Create a new user (GET/users)
#[get("/get-all")]
pub async fn get_all_users_controller(service: Data<UserService>, req: HttpRequest) -> HttpResponse {
    // permission
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Read) {
        return HttpResponse::Forbidden().body("You do not have permission to get users.");
    }

    match service.get_all_users_service().await {
        Ok(users) => {
            let response = GenericResponse {
                code: 200,
                message: "All users".to_string(),
                data: users,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
/*
// #[get("/get-all-paginate")]
// pub async fn get_all_users_paginate_controller(service: Data<UserService>,params:web::Query<QueryOptions>,req:HttpRequest) -> HttpResponse {
//     // permission
//     if !has_permission_with_roles(&req,&vec![Role::Admin],&Permission::Read) {
//         return HttpResponse::Forbidden().body("You do not have permission to get users.");
//     }
//
//    let  search_test =  params.search_text ;
//    let  page = params.page;
//    let  size = params.size;
//
//
//     match service.get_all_users_paginate_service(search_test,page,size).await {
//         Ok(users) => HttpResponse::Ok().json(users),
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string())
//     }
// }
 */

//Update a user by ID (PUT /users/{id})
#[put("/update/{id}")]
pub async fn update_user_controller(service: Data<UserService>, update_user: Json<UpdateUser>, id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    // permission
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Write) {
        return HttpResponse::Forbidden().body("You do not have permission to update users.");
    }
    // validation
    if let Err(err) = update_user.validate() {
        return HttpResponse::BadRequest().body(err.to_string());
    }

    println!("user id : {}", &id);
    match service.update_user_service(update_user.into_inner(), id.to_string()).await {
        Ok(Some(user)) => {
            let response = GenericResponse {
                code: 201,
                message: "User has updated".to_string(),
                data: user,
            };
            HttpResponse::Created().json(response)
        }
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(e) => HttpResponse::InternalServerError().body( e.to_string())
    }
}

// Delete a user by ID (DELETE /users/{id})
#[delete("/delete/{id}")]
pub async fn delete_user_controller(service: Data<UserService>, id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    // permission
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Delete) {
        return HttpResponse::Forbidden().body("You do not have permission to delete users.");
    }

    match service.delete_user_service(id.to_string()).await {
        Ok(_) => {
            let response = GenericResponse {
                code: 204,
                message: "User has deleted".to_string(),
                data: (),
            };
            HttpResponse::NoContent().json(response)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}


