use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, ModelTrait, PaginatorTrait};
use crate::exceptions::errors::SystemError;
use crate::models::User;
use crate::models::user_model::{ActiveModel, Model, Column};
use sea_orm::QueryFilter;

pub async fn create_user_repo(db: &DatabaseConnection, user: ActiveModel) -> Result<Model, SystemError> {
    user.insert(db).await.map_err(|e| SystemError::DbError(e))
}

pub async fn update_user_repo(db: &DatabaseConnection, user: ActiveModel) -> Result<Model, SystemError> {
    user.update(db).await.map_err(|e| SystemError::DbError(e))
}

pub async fn delete_user_repo(db: &DatabaseConnection, user: Model) -> Result<DeleteResult, SystemError> {
    user.delete(db).await.map_err(|e| SystemError::DbError(e))
}

pub async fn all_users_repo(db: &DatabaseConnection, search_text: &String, page: u64, size: u64) -> Result<Vec<Model>, SystemError> {
    let paginator = User::find()
        .filter(Column::Name.contains(search_text))
        .paginate(db, size);

    paginator.fetch_page(page - 1).await.map_err(|e| SystemError::DbError(e))
}

pub async fn all_users_count_repo(db: &DatabaseConnection, search_text: &String) -> Result<u64, SystemError> {
    User::find()
        .filter(Column::Name.contains(search_text))
        .count(db)
        .await.map_err(|e| SystemError::DbError(e))
}

pub async fn find_user_by_email(db: &DatabaseConnection, email: &String) -> Result<Option<Model>, SystemError> {
    User::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await.map_err(|e| SystemError::DbError(e))
}
