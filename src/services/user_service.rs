use mongodb::Collection;
use crate::models::user_model::{CreateUser, UpdateUser, User};
use crate::repo::user_repo::{create_new_user, delete_user, get_all_users, update_system_user};

pub async fn create_user_service(collection: &Collection<User>, create_user: CreateUser) ->Result<User,String>{
   create_new_user(collection,create_user).await.map_err(|_| "Failed to create user".to_string())
}

pub async fn get_all_users_service(collection: &Collection<User>)->Result<Vec<User>,String>{
    get_all_users(collection).await.map_err(|_| "Failed to fetch users".to_string())
}

pub async fn update_user_service(collection: &Collection<User>,update_user: UpdateUser,id:String)->Result<Option<User>,String>{
    update_system_user(collection,update_user,id).await.map_err(|_| "Failed to update user".to_string())
}

pub async fn delete_user_service(collection: &Collection<User>,id:String)->Result<(),String>{
    delete_user(collection,id).await.map_err(|_| "Failed to delete user".to_string())
}