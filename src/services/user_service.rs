use crate::exceptions::errors::SystemError;
use crate::models::user_model::{CreateUser, UpdateUser, User, UserLoginDto};
use crate::repo::user_repo::UserRepo;

pub struct UserService{
    repo:UserRepo
}

impl UserService{
    pub fn new(repo:UserRepo)->Self{
        UserService{repo}
    }

    pub async fn user_login_service(&self,dto:UserLoginDto)->Result<Option<String>,SystemError>{
        self.repo.user_login(dto).await
    }

    pub async fn create_user_service(&self, user: CreateUser) ->Result<User,SystemError>{
        self.repo.create_new_user(user).await
    }

    pub async fn get_all_users_service(&self)->mongodb::error::Result<Vec<User>>{
        self.repo.get_all_users().await
    }

    // pub async fn get_all_users_paginate_service(&self,search_text:&str,page:i32,size:i32)->Result<Vec<User>>{
    //     self.repo.get_all_paginate(search_text,page,size).await
    // }

    pub async fn update_user_service(&self,update_user: UpdateUser,id:String)->Result<Option<User>,SystemError>{
        self.repo.update_system_user(update_user,id).await
    }

    pub async fn delete_user_service(&self,id:String)->Result<(),SystemError>{
        self.repo.delete_user(id).await
    }
}

