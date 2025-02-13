use crate::exceptions::errors::{JwtError, PasswordError, SystemError};
use crate::models::user_model::{Entity, LoginRequestDto, Model, PaginateUserResponseDto, UserRequestDto, UserResponseDto};
use crate::models::{user_model, User};
use crate::repo::user_repo::{all_users_count_repo, all_users_repo, create_user_repo, delete_user_repo, find_user_by_email, update_user_repo};
use crate::utill::bcrypt::{hash_password, verify_password};
use crate::utill::jwt::create_token;
use log::{error, info, warn};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, NotSet, Set};
use uuid::Uuid;

pub async fn user_login_service(db: &DatabaseConnection, dto: LoginRequestDto) -> Result<Option<String>, SystemError> {
    let selected_user = find_user_by_email(db, &dto.username).await?;
    let mut roles = Vec::new();
    if selected_user.is_none() {
        return Err(SystemError::NotFoundError(dto.username));
    }
    let selected_user = selected_user.unwrap();
    roles.push(selected_user.role);

    let is_verify = verify_password(&dto.password, &selected_user.password)
        .unwrap_or_else(|e| false);

    if is_verify {
        let token = create_token(selected_user.email, roles);
        match token {
            Ok(token) => {
                info!("token created successfully: {:?}",token);
                Ok(Some(token))
            }
            Err(e) => {
                error!("token not created {:?}",e);
                Err(SystemError::JwtError(JwtError::TokenError("Failed to generate token : ".to_string())))
            }
        }
    } else {
        warn!("invalid password {:?}",dto.password);
        Err(SystemError::PasswordError(PasswordError::InvalidPassword))
    }
}

pub async fn create_user_service(db: &DatabaseConnection, dto: UserRequestDto) -> Result<Model, SystemError> {
    let selected_user = find_user_by_email(db, &dto.email).await?;
    if selected_user.is_some() {
        return Err(SystemError::DuplicateError(dto.email));
    }

    let hash_pw = match hash_password(&dto.password) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Password hashing failed: {:?}", e);
            return Err(SystemError::PasswordError(PasswordError::PasswordHashErr(e.to_string())));
        }
    };

    let new_user = user_model::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(dto.name),
        email: Set(dto.email),
        role: Set("User".to_string()),
        password: Set(hash_pw),
    };

    match create_user_repo(db, new_user).await {
        Ok(user) => {
            info!("User successfully created: {:?}", user);
            Ok(user)
        }
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            Err(e)
        }
    }
}

pub async fn update_user_service(db: &DatabaseConnection, id: String, dto: UserRequestDto) -> Result<Model, SystemError> {
    //convert String to uuid
    let user_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(SystemError::ValidationError("Invalid UUID format".to_string())),
    };

    let select_user = User::find_by_id(user_id).one(db).await?;

    if select_user.is_none() {
        return Err(SystemError::NotFoundError(id.to_string() + " id"));
    }

    let update_hash_pw = match hash_password(&dto.password) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Password hashing failed: {:?}", e);
            return Err(SystemError::PasswordError(PasswordError::PasswordHashErr(e.to_string())));
        }
    };

    let mut select_user = select_user.unwrap();
    let mut active_user: user_model::ActiveModel = select_user.into();

    active_user.name = Set(dto.name);
    active_user.email = Set(dto.email);
    active_user.password = Set(update_hash_pw);

    match update_user_repo(db, active_user).await {
        Ok(update_user) => {
            info!("User successfully created: {:?}", update_user);
            Ok(update_user)
        }
        Err(e) => {
            error!("Failed to update user: {:?}", e);
            Err(e)
        }
    }
}

pub async fn delete_user_service(db: &DatabaseConnection, id: String) -> Result<DeleteResult, SystemError> {
    let user_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(SystemError::ValidationError("Invalid UUID format".to_string()))
    };

    let selected_user = User::find_by_id(user_id).one(db).await?;
    if selected_user.is_none() {
        return Err(SystemError::NotFoundError(id.to_string() + " id"));
    }
    let selected_user = selected_user.unwrap();
    match delete_user_repo(db, selected_user).await {
        Ok(delete_user) => {
            info!("User successfully deleted");
            Ok(delete_user)
        }
        Err(e) => {
            error!("Failed to delete user: {:?}", e);
            Err(e)
        }
    }
}

pub async fn get_all_paginate_service(db: &DatabaseConnection, search_text: String, page: u64, size: u64) -> Result<PaginateUserResponseDto, SystemError> {
    let users = all_users_repo(db, &search_text, page, size).await?;
    let users_count = all_users_count_repo(db, &search_text).await?;
    let paginate_users = PaginateUserResponseDto {
        count: users_count,
        list: users.iter().map(create_response_dto).collect(),
    };

    Ok(paginate_users)
}

fn create_response_dto(user: &Model) -> UserResponseDto {
    UserResponseDto {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
    }
}



