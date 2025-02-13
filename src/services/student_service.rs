use actix_web::web::Data;
use log::{error, info};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, NotSet, Set};
use uuid::{uuid, Uuid};
use crate::exceptions::errors::{PasswordError, SystemError};
use crate::models::{student_model, user_model, Student, User};
use crate::models::student_model::{Model, PaginateStudentResponseDto, StudentRequestDto, StudentResponseDto};
use crate::repo::student_repo::{all_students_count_repo, all_students_repo, create_student_repo, delete_student_repo, find_student_by_user, update_student_repo};
use crate::repo::user_repo::{all_users_count_repo, all_users_repo, find_user_by_email, update_user_repo};
use crate::utill::bcrypt::hash_password;
use futures::future;
use crate::models::user_model::{PaginateUserResponseDto, UserResponseDto};

pub async fn create_student_service(db: &DatabaseConnection, dto: StudentRequestDto) -> Result<Model, SystemError> {
    let selected_user = find_user_by_email(db, &dto.user_email).await?;

    if selected_user.is_none() {
        return Err(SystemError::NotFoundError(dto.user_email + " user"));
    }
    let user = selected_user.unwrap();
    let selected_student: Option<Model> = find_student_by_user(db, user.clone()).await?;
    if selected_student.is_some() {
        return Err(SystemError::DuplicateError(user.name + " student"));
    }

    let new_student = student_model::ActiveModel {
        id: Set(Uuid::new_v4()),
        grade: Set(dto.grade),
        class_teacher: Set(dto.class_teacher),
        user_id: Set(user.id),
    };
    match create_student_repo(db, new_student).await {
        Ok(st) => {
            info!("student successfully created: {:?}", st);
            update_role(db, user).await?;
            Ok(st)
        }
        Err(e) => {
            error!("student to create user: {:?}", e);
            Err(e)
        }
    }
}

pub async fn update_student_service(db: &DatabaseConnection, id: String, dto: StudentRequestDto) -> Result<Model, SystemError> {

    //convert String to uuid
    let st_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(SystemError::ValidationError("Invalid UUID format".to_string())),
    };

    let select_student = Student::find_by_id(st_id).one(db).await?;

    if select_student.is_none() {
        return Err(SystemError::NotFoundError(id.to_string() + " id"));
    }


    let mut select_student = select_student.unwrap();
    let mut active_student: student_model::ActiveModel = select_student.into();

    active_student.grade = Set(dto.grade);
    active_student.class_teacher = Set(dto.class_teacher);

    match update_student_repo(db, active_student).await {
        Ok(update_st) => {
            info!("student successfully updated: {:?}", update_st);
            Ok(update_st)
        }
        Err(e) => {
            error!("Failed to update student: {:?}", e);
            Err(e)
        }
    }
}

pub async fn delete_student_service(db: &DatabaseConnection, id: String) -> Result<DeleteResult, SystemError> {
    let st_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(SystemError::ValidationError("Invalid UUID format".to_string()))
    };

    let selected_student = Student::find_by_id(st_id).one(db).await?;
    if selected_student.is_none() {
        return Err(SystemError::NotFoundError(id.to_string() + " id"));
    }
    let selected_student = selected_student.unwrap();
    match delete_student_repo(db, selected_student).await {
        Ok(delete_student) => {
            info!("student successfully deleted");
            Ok(delete_student)
        }
        Err(e) => {
            error!("Failed to delete student: {:?}", e);
            Err(e)
        }
    }
}


pub async fn get_all_students_paginate_service(
    db: &DatabaseConnection,
    search_text: String,
    page: u64,
    size: u64,
) -> Result<PaginateStudentResponseDto, SystemError> {
    let students: Vec<Model> = all_students_repo(db, &search_text, page, size).await?;
    let students_count = all_students_count_repo(db, &search_text).await?;

    println!("student : {:?}", students);

    let paginate_student = PaginateStudentResponseDto {
        count: students_count,
        list: students.iter().map(create_response_dto).collect(),
    };

    Ok(paginate_student)
}


fn create_response_dto(student: &Model) -> StudentResponseDto {
    StudentResponseDto {
        id: student.id,
        grade: student.grade.clone(),
        class_teacher: student.class_teacher.clone(),
        user_id: student.user_id.clone(),
    }
}


async fn update_role(db: &DatabaseConnection, user: user_model::Model) -> Result<(), SystemError> {
    let mut active_user: user_model::ActiveModel = user.into();
    active_user.role = Set("Student".to_string());

    match update_user_repo(db, active_user).await {
        Ok(_) => {
            info!("student role updated");
            Ok(())
        }
        Err(e) => {
            error!("student role update failed: {:?}", e);
            Err(e)
        }
    }
}
