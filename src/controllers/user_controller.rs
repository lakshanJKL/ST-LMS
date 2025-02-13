use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web::body::MessageBody;
use actix_web::web::{Data, Json, Path, Query};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use validator::Validate;
use crate::midleware::permission::{Permission, Role};
use crate::models::user_model::{LoginRequestDto, UserQueryOptions, UserRequestDto};
use crate::services::user_service::{create_user_service, delete_user_service, get_all_paginate_service, update_user_service, user_login_service};
use crate::utill::generic_response::GenericResponse;
use crate::utill::jwt::{has_permission_with_roles};


#[post("/login")]
pub async fn user_login_controller(db: Data<DatabaseConnection>, dto: Json<LoginRequestDto>) -> HttpResponse {

    //input validation
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    match user_login_service(&db, dto.into_inner()).await {
        Ok(Some(token)) => {
            let res = GenericResponse {
                code: 200,
                message: "successfully logged in".to_string(),
                data: token,
            };
            info!("User successfully logged in : {:?}", res);
            HttpResponse::Ok().json(res)
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials".to_string()),
        Err(e) => {
            error!("User failed successfully logged in {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/signup")]
pub async fn create_user_controller(db: Data<DatabaseConnection>, dto: Json<UserRequestDto>) -> HttpResponse {
    // input validation
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    match create_user_service(&db, dto.into_inner()).await {
        Ok(user) => {
            let res = GenericResponse {
                code: 201,
                message: "user has created".to_string(),
                data: user,
            };
            info!("User successfully created : {:?}", res);
            HttpResponse::Created().json(res)
        }
        Err(e) => {
            error!("User not created : error :: {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[put("/update/{id}")]
pub async fn update_user_controller(db: Data<DatabaseConnection>, id: Path<String>, dto: Json<UserRequestDto>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Write) {
        return HttpResponse::Forbidden().body("You do not have permission to update users");
    }

    // input validation
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    match update_user_service(&db, id.to_string(), dto.into_inner()).await {
        Ok(user) => {
            let res = GenericResponse {
                code: 201,
                message: "user has updated".to_string(),
                data: user,
            };
            info!("User successfully updated: {:?}", res);
            HttpResponse::Created().json(res)
        }

        Err(e) => {
            error!("User not updated : error :: {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[delete("delete/{id}")]
pub async fn delete_user_controller(db: Data<DatabaseConnection>, id: Path<String>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Delete) {
        return HttpResponse::Forbidden().body("You do not have permission to delete users");
    }

    match delete_user_service(&db, id.to_string()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[get("/get-all-users")]
pub async fn get_all_paginate_controller(db: Data<DatabaseConnection>, query: Query<UserQueryOptions>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Read) {
        return HttpResponse::Forbidden().body("You do not have permission to get users");
    }

    // input validation
    if let Err(e) = query.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    let search_text = query.search_text.clone().unwrap_or("".to_string());
    let page = query.page;
    let page_size = query.size;
    match get_all_paginate_service(&db, search_text, page, page_size).await {
        Ok(users) => {
            let res = GenericResponse {
                code: 200,
                message: "All users".to_string(),
                data: users,
            };
            info!("All users: {:?}", res);
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            error!("Failed get all users {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}


