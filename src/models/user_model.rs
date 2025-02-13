use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utill::validator::{custom_text_check};

// Define the Model (immutable representation of a row in the database)
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::student_model::Entity")]
    Students,
}

impl Related<super::student_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}


//**********  dto **************
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequestDto {
    #[validate(length(min = 5, message = "Name must be at least 5 characters long"), email(
        message = "Invalid email format"
    ))]
    pub username: String,
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserRequestDto {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserQueryOptions {
    #[validate(custom = "custom_text_check")]
    pub search_text: Option<String>,
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: u64,
    #[validate(range(min = 1, max = 10, message = "Size must be between 1 and 100"))]
    pub size: u64,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginateUserResponseDto {
    pub count: u64,
    pub list: Vec<UserResponseDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
}



