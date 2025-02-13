use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::user_model;
use crate::utill::validator::custom_text_check;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "students")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub grade: String,
    pub class_teacher: String,
    pub user_id: Uuid,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_model::Entity",
        from = "Column::UserId",
        to = "super::user_model::Column::Id"
    )]
    User,
}

impl Related<super::user_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}


// dto

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StudentRequestDto {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub grade: String,
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub class_teacher: String,
    #[validate(length(min = 5, message = "Name must be at least 5 characters long"), email(
        message = "Invalid email format"
    ))]
    pub user_email: String,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StudentQueryOptions {
    #[validate(custom = "custom_text_check")]
    pub search_text: Option<String>,
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: u64,
    #[validate(range(min = 1, max = 10, message = "Size must be between 1 and 100"))]
    pub size: u64,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginateStudentResponseDto {
    pub count: u64,
    pub list: Vec<StudentResponseDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentResponseDto {
    pub id: Uuid,
    pub grade: String,
    pub class_teacher: String,
    pub user_id: Uuid,
}
