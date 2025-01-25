use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use serde_json::json;
use validator::Validate;
use crate::midleware::permission::{Permission, Role};
use crate::models::user_model::{CreateUser, UpdateUser, UserLoginDto};
use crate::services::user_service::UserService;
use crate::utill::jwt::{ has_permission_with_roles};

//login user (POST /users)
#[post("/login")]
pub async fn user_login_controller(service: Data<UserService>, user: Json<UserLoginDto>) -> HttpResponse {
    // validation
    if let Err(err) = user.validate(){
        return HttpResponse::BadRequest().body(err.to_string())
    }

    match service.user_login_service(user.into_inner()).await {
        Ok(Some(token)) => HttpResponse::Ok().json(json!({"token":token})),
        Ok(None) => HttpResponse::Unauthorized().json(json!({ "error": "Invalid credentials" })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Login failed" }))
    }
}

//signup a new user (POST /users)
#[post("/signup")]
pub async fn create_user_controller(service: Data<UserService>, create_user: Json<CreateUser>) -> HttpResponse {
    // validation
    if let Err(err) = create_user.validate(){
        return HttpResponse::BadRequest().body(err.to_string())
    }

    match service.create_user_service(create_user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Create a new user (GET/users)
#[get("/get-all")]
pub async fn get_all_users_controller(service: Data<UserService>,req:HttpRequest) -> HttpResponse {
    // permission
    if !has_permission_with_roles(&req,&vec![Role::Admin],&Permission::Read) {
        return HttpResponse::Forbidden().body("You do not have permission to get users.");
    }


    match service.get_all_users_service().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

//Update a user by ID (PUT /users/{id})
#[put("/update/{id}")]
pub async fn update_user_controller(service: Data<UserService>, update_user: Json<UpdateUser>, id: web::Path<String>,req:HttpRequest) -> HttpResponse {
    // permission
    if !has_permission_with_roles(&req,&vec![Role::Admin],&Permission::Write){
        return HttpResponse::Forbidden().body("You do not have permission to update users.");
    }
    // validation
    if let Err(err) = update_user.validate(){
        return HttpResponse::BadRequest().body(err.to_string())
    }

    println!("user id : {}", &id);
    match service.update_user_service(update_user.into_inner(), id.to_string()).await {
        Ok(Some(user)) => HttpResponse::Created().json(user),
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e))
    }
}

// Delete a user by ID (DELETE /users/{id})
#[delete("/delete/{id}")]
pub async fn delete_user_controller(service: Data<UserService>, id: web::Path<String>,req:HttpRequest) -> HttpResponse {
    // permission
    if !has_permission_with_roles(&req,&vec![Role::Admin],&Permission::Delete) {
        return HttpResponse::Forbidden().body("You do not have permission to delete users.");
    }

    match service.delete_user_service(id.to_string()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e))
    }
}


