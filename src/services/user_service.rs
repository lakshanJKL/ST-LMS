use mongodb::error::Result;
use crate::models::user_model::{CreateUser, UpdateUser, User, UserLoginDto};
use crate::repo::user_repo::UserRepo;
use crate::utill::jwt::create_token;

pub struct UserService{
    repo:UserRepo
}

impl UserService{
    pub fn new(repo:UserRepo)->Self{
        UserService{repo}
    }

    pub async fn user_login_service(&self,dto:UserLoginDto)->Result<Option<String>>{
        self.repo.user_login(dto).await
    }

    pub async fn create_user_service(&self, user: CreateUser) ->Result<User>{
        println!("service {:?}",user);
        self.repo.create_new_user(user).await
    }

    pub async fn get_all_users_service(&self)->Result<Vec<User>>{
        self.repo.get_all_users().await
    }

    pub async fn update_user_service(&self,update_user: UpdateUser,id:String)->Result<Option<User>>{
        self.repo.update_system_user(update_user,id).await
    }

    pub async fn delete_user_service(&self,id:String)->Result<()>{
        self.repo.delete_user(id).await
    }
}

