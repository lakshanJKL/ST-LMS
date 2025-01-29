use futures_util::TryStreamExt;
use mongodb::{bson::{doc}, Collection, Database};
use mongodb::bson::oid::ObjectId;
use crate::exceptions::errors::{SystemError, JwtError, PasswordError };
use crate::models::user_model::{User, CreateUser, UpdateUser, UserLoginDto};
use crate::utill::bcrypt::{hash_password, verify_password};
use crate::utill::jwt::create_token;

pub struct UserRepo {
    collection: Collection<User>,
}

impl UserRepo {
    pub async fn new(db: Database) -> Self {
        let collection = db.collection("users");
        UserRepo { collection }
    }

    // user login
    pub async fn user_login(&self, user_dto: UserLoginDto) -> Result<Option<String>,SystemError> {
        let mut roles = Vec::new();
        let filter = doc! {"email": &user_dto.username};
        let select_user = self.collection.find_one(filter).await?;

        if let Some(user) = select_user {
            roles.push(user.role);

            let is_verify= verify_password(&user_dto.password, &user.password)
                .unwrap_or_else(|e| false);

            println!("is verify : {}",is_verify);
            if is_verify {
                // Generate JWT token on successful login
                    let token = create_token(user.email, roles);
                    match token {
                        Ok(token) => Ok(Some(token)),
                        Err(_) => Err(SystemError::JwtError(JwtError::TokenError("Failed to generate token".to_string()))),
                    }

            }else {
               Err(SystemError::PasswordError(PasswordError::InvalidPassword(user_dto.password)))
            }
        } else {
            Err(SystemError::NotFoundError(user_dto.username))
        }
    }


    // create user
    pub async fn create_new_user(&self, user: CreateUser) -> Result<User,SystemError> {

        let filter = doc! {"email": &user.email};
        if self.collection.find_one(filter).await?.is_some(){
           return  Err(SystemError::DuplicateError(user.email))
        }

        let hash_pw = hash_password(&user.password).expect("password hash failed");

        let new_user = User {
            name: user.name,
            email: user.email,
            role: "User".to_string(),
            password: hash_pw,
        };

        self.collection.insert_one(&new_user).await.ok().expect("Error creating user");
        Ok(new_user)
    }

    //all user
    pub async fn get_all_users(&self) -> mongodb::error::Result<Vec<User>> {
        let filter = doc! {};
        let cursor = self.collection.find(filter).await?;
        cursor.try_collect().await
    }

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
            return Err(SystemError::NotFoundError(object_id.to_string()+" id"));
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
            Ok(updated_user) => Ok(updated_user),
            Err(e) => Err(SystemError::MongoError(e)), // Convert MongoDB error to `DatabaseError`
        }
    }


    // delete user
    pub async fn delete_user(&self, id: String) -> Result<(),SystemError> {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(oid) => oid,
            Err(_) => {
                eprintln!("Invalid ObjectId: {}", id);
                return Err(SystemError::ValidationError("Invalid ObjectId".to_string()));
            }
        };

        let filter = doc! {"_id":&object_id};
        if self.collection.find_one(filter.clone()).await?.is_none(){
            return  Err(SystemError::NotFoundError(object_id.to_string()+" id"))
        }

        self.collection.delete_one(filter).await?;
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





