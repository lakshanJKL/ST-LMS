use actix_web::{delete, get, post, put, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json, Path, Query};
use log::{error, info};
use sea_orm::DatabaseConnection;
use validator::Validate;
use crate::midleware::permission::{Permission, Role};
use crate::models::student_model::{StudentQueryOptions, StudentRequestDto};
use crate::services::student_service::{create_student_service, delete_student_service, get_all_students_paginate_service, update_student_service};
use crate::utill::generic_response::GenericResponse;
use crate::utill::jwt::has_permission_with_roles;

#[post("/create")]
pub async fn create_student_controller(db: Data<DatabaseConnection>, dto: Json<StudentRequestDto>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Write) {
        return HttpResponse::Forbidden().body("You do not have permission to create student");
    }

    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    match create_student_service(&db, dto.into_inner()).await {
        Ok(student) => {
            let res = GenericResponse {
                code: 201,
                message: "student has created".to_string(),
                data: student,
            };
            info!("student has created {:?}",res);
            HttpResponse::Created().json(res)
        }
        Err(e) => {
            error!("student not created : error :: {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[put("/update/{id}")]
pub async fn update_student_controller(db: Data<DatabaseConnection>, id: Path<String>, dto: Json<StudentRequestDto>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Write) {
        return HttpResponse::Forbidden().body("You do not have permission to update student");
    }

    // input validation
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    match update_student_service(&db, id.to_string(), dto.into_inner()).await {
        Ok(update_student) => {
            let res = GenericResponse {
                code: 201,
                message: "student has updated".to_string(),
                data: update_student,
            };
            info!("Student successfully updated: {:?}", res);
            HttpResponse::Created().json(res)
        }

        Err(e) => {
            error!("Student not updated : error :: {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[delete("delete/{id}")]
pub async fn delete_student_controller(db: Data<DatabaseConnection>, id: Path<String>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin], &Permission::Delete) {
        return HttpResponse::Forbidden().body("You do not have permission to delete student");
    }

    match delete_student_service(&db, id.to_string()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[get("/get-all-students")]
pub async fn get_all_students_paginate_controller(db: Data<DatabaseConnection>, query: Query<StudentQueryOptions>, req: HttpRequest) -> HttpResponse {
    if !has_permission_with_roles(&req, &vec![Role::Admin, Role::Student, Role::User], &Permission::Read) {
        return HttpResponse::Forbidden().body("You do not have permission to get users");
    }

    // input validation
    if let Err(e) = query.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    let search_text = query.search_text.clone().unwrap_or("".to_string());
    let page = query.page;
    let page_size = query.size;
    match get_all_students_paginate_service(&db, search_text, page, page_size).await {
        Ok(users) => {
            let res = GenericResponse {
                code: 200,
                message: "All students".to_string(),
                data: users,
            };
            info!("All students: {:?}", res);
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            error!("Failed get all students {:?}",e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}