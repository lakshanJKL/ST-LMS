use futures_util::TryStreamExt;
use log::{error, info, warn};
use mongodb::{bson::{doc}, Collection, Database};
use mongodb::bson::oid::ObjectId;
use crate::exceptions::errors::{SystemError, JwtError, PasswordError};
use crate::models::user_model::{User, CreateUser, UpdateUser, UserLoginDto};
use crate::repo::audit_log_repo::AuditLogRepo;
use crate::utill::bcrypt::{hash_password, verify_password};
use crate::utill::jwt::create_token;

pub struct UserRepo {
    collection: Collection<User>,
    log_repo: AuditLogRepo,
}

impl UserRepo {
    pub async fn new(db: &Database, log_repo: AuditLogRepo) -> Self {
        let collection = db.collection("users");
        UserRepo {
            collection,
            log_repo,
        }
    }

    // user login
    pub async fn user_login(&self, user_dto: UserLoginDto) -> Result<Option<String>, SystemError> {
        let mut roles = Vec::new();
        let filter = doc! {"email": &user_dto.username};
        let select_user = self.collection.find_one(filter).await?;

        if let Some(user) = select_user {
            roles.push(user.role);

            let is_verify = verify_password(&user_dto.password, &user.password)
                .unwrap_or_else(|e| false);

            if is_verify {
                // Generate JWT token on successful login
                let token = create_token(user.email, roles);

                match token {
                    Ok(token) => {
                        info!("token created successfully: {:?}",token);
                        Ok(Some(token))
                    }
                    Err(_) => {
                        error!("token not created");
                        Err(SystemError::JwtError(JwtError::TokenError("Failed to generate token".to_string())))
                    }
                }
            } else {
                warn!("invalid password {:?}",user_dto.password);
                Err(SystemError::PasswordError(PasswordError::InvalidPassword(user_dto.password)))
            }
        } else {
            error!("not found :{:?}",user_dto.username);
            Err(SystemError::NotFoundError(user_dto.username))
        }
    }


    // create user
    pub async fn create_new_user(&self, user_dto: CreateUser) -> Result<User, SystemError> {
        let filter = doc! {"email": &user_dto.email};
        if self.collection.find_one(filter).await?.is_some() {
            return Err(SystemError::DuplicateError(user_dto.email));
        }

        let hash_pw = hash_password(&user_dto.password).expect("password hash failed");

        let new_user = User {
            name: user_dto.name,
            email: user_dto.email,
            role: "User".to_string(),
            password: hash_pw,
        };

        match self.collection.insert_one(&new_user).await {
            Ok(res) => {
                // add to log
                self.log_repo.log_action(Some(res.inserted_id.to_string()), "Create_user".to_string(), format!("Created user with email,user {:?}", new_user.email)).await?;
                info!("user created successfully {:?}",&new_user);
                Ok(new_user)
            }
            Err(e) => {
                error!("user create failed");
                Err(SystemError::MongoError(e))
            }
        }
    }

    //all user
    pub async fn get_all_users(&self) -> Result<Vec<User>, SystemError> {
        let filter = doc! {};
        let cursor = self.collection.find(filter).await.map_err(SystemError::MongoError)?; // Handle MongoDB error
        let users = cursor.try_collect::<Vec<User>>().await.map_err(SystemError::MongoError)?; // Convert Cursor to Vec<User>

        info!("users get successfully");
        Ok(users)
    }

    // get all user paginate
    // pub async fn get_all_users_paginate(&self,search_text:Option<str>,page:i32,size:i32)->Result<Vec<User>,SystemError>{
    //     let filter = match search_text {
    //         Some(text) => doc! {"name":{"regex":text,"$options":"i"}} ,// Case-insensitive search
    //         None => doc!{}
    //     };
    //
    //     let skip = (page-1) * size;
    //     let find_options = mongodb::options::FindOptions::builder()
    //         .skip(Some(skip as u64))
    //         .limit(Some(size as i64))
    //         .build();
    //
    //   //  let cursor = self.collection.find(filter.clone(),find_options).await?;
    //
    // }

    //update user
    pub async fn update_system_user(&self, update_user: UpdateUser, id: String) -> Result<Option<User>, SystemError> {
        // Parse the ObjectId
        let object_id = match ObjectId::parse_str(&id) {
            Ok(oid) => oid,
            Err(_) => {
                return Err(SystemError::ValidationError("Invalid ObjectId".to_string()));
            }
        };

        let filter = doc! {"_id": &object_id};

        // Check if the user exists
        if self.collection.find_one(filter.clone()).await?.is_none() {
            return Err(SystemError::NotFoundError(object_id.to_string() + " id"));
        }

        let mut update_doc = doc! {};

        if let Some(name) = &update_user.name {
            update_doc.insert("name", name);
        }

        if let Some(email) = &update_user.email {
            update_doc.insert("email", email);
        }

        if let Some(password) = &update_user.password {
            let update_hash_pw = hash_password(password).expect("password hash failed");
            update_doc.insert("password", update_hash_pw);
        }

        let update = doc! {"$set": update_doc};

        // Perform update and return the updated user
        match self.collection.find_one_and_update(filter, update).await {
            Ok(updated_user) => {
                info!("user updated successfully {:?}",updated_user);
                Ok(updated_user)
            }
            Err(e) => {
                error!("user update failed");
                Err(SystemError::MongoError(e))    // Convert MongoDB error to `DatabaseError`
            }
        }
    }


    // delete user
    pub async fn delete_user(&self, id: String) -> Result<(), SystemError> {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(oid) => oid,
            Err(_) => {
                warn!("Invalid object id : {:?}",id);
                return Err(SystemError::ValidationError("Invalid ObjectId".to_string()));
            }
        };

        let filter = doc! {"_id":&object_id};
        if self.collection.find_one(filter.clone()).await?.is_none() {
            return Err(SystemError::NotFoundError(object_id.to_string() + " id"));
        }

        let delete_result = self.collection.delete_one(filter).await.map_err(SystemError::MongoError)?;
        if delete_result.deleted_count == 0 {
            warn!("No user deleted, possibly because the id did not exist: {:?}", object_id);
            return Err(SystemError::NotFoundError(object_id.to_string()));
        }

        info!("User deleted successfully with id: {:?}", object_id);
        Ok(())
    }

    // initialize admin
    // pub async fn initialize_user_admin(&self) {
    //
    //         //admin password
    //         let hash_pw = hash_password("1234").expect("password hash failed");
    //
    //         let new_user = User {
    //             name: "lakshan".to_string(),
    //             email: "admin@gmail.com".to_string(),
    //             role: "Admin".to_string(),
    //             password: hash_pw,
    //         };
    //
    //         self.collection.insert_one(new_user)
    //             .await.ok()
    //             .expect("Error creating user roles");
    //     }

}





