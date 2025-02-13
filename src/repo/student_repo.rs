use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, ModelTrait, PaginatorTrait};
use crate::exceptions::errors::SystemError;
use crate::models::student_model::{Entity, Model, Column, ActiveModel};
use crate::models::{user_model, Student};
use sea_orm::QueryFilter;

pub async fn create_student_repo(db: &DatabaseConnection, student: ActiveModel) -> Result<Model, SystemError> {
    student.insert(db).await.map_err(|e| SystemError::DbError(e))
}

pub async fn update_student_repo(db: &DatabaseConnection, student: ActiveModel) -> Result<Model, SystemError> {
    student.update(db).await.map_err(|e| SystemError::DbError(e))
}

pub async fn delete_student_repo(db: &DatabaseConnection, student: Model) -> Result<DeleteResult, SystemError> {
    student.delete(db).await.map_err(|e| SystemError::DbError(e))
}

pub async fn all_students_repo(db: &DatabaseConnection, search_text: &String, page: u64, size: u64) -> Result<Vec<Model>, SystemError> {
    let paginator = Student::find()
        .filter(Column::Grade.contains(search_text))
        .paginate(db, size);

    paginator.fetch_page(page - 1).await.map_err(|e| SystemError::DbError(e))
}

pub async fn all_students_count_repo(db: &DatabaseConnection, search_text: &String) -> Result<u64, SystemError> {
    Student::find()
        .filter(Column::Grade.contains(search_text))
        .count(db)
        .await.map_err(|e| SystemError::DbError(e))
}

pub async fn find_student_by_user(db: &DatabaseConnection, user: user_model::Model) -> Result<Option<Model>, SystemError> {
    user.find_related(Entity).one(db).await.map_err(|e| SystemError::DbError(e))
}