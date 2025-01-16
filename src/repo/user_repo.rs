use futures_util::TryStreamExt;
use mongodb::{bson::{doc, to_bson}, error::Result, Collection, Database};
use mongodb::bson::oid::ObjectId;
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
    pub async fn user_login(&self, user_dto: UserLoginDto) -> Result<Option<String>> {
        let mut roles = Vec::new();
        let filter = doc! {"email": &user_dto.username};
        let select_user = self.collection.find_one(filter).await?;

        if let Some(user) = select_user {
            roles.push(user.role);

            match verify_password(&user_dto.password, &user.password) {
                // Generate JWT token on successful login
                Ok(true) => {
                    let token = create_token(user.email, roles);
                    Ok(Some(token.unwrap()))
                }
                Ok(false) => Ok(None),
                Err(_) => Ok(None)
            }
        } else {
            Ok(None)
        }
    }


    // create user
    pub async fn create_new_user(&self, user: CreateUser) -> Result<User> {
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
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let filter = doc! {};
        let cursor = self.collection.find(filter).await?;
        cursor.try_collect().await
    }

    //update user
    pub async fn update_system_user(&self, update_user: UpdateUser, id: String) -> Result<Option<User>> {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(oid) => oid,
            Err(_) => {
                eprintln!("Invalid ObjectId: {}", id);
                return Ok(None); // Return None if id is invalid
            }
        };

        let filter = doc! {"_id":&object_id};
        let mut update_doc = doc! {};

        if let Some(name) = &update_user.name {
            update_doc.insert("name", name);
        }

        if let Some(email) = &update_user.name {
            update_doc.insert("email", email);
        }

        if let Some(password) = &update_user.password {
            let update_hash_pw = hash_password(password).expect("password hash failed");
            update_doc.insert("password", update_hash_pw);
        }

        // Safely unwrap the `Bson` value for the `$set` operation
        let update = doc! {"$set": update_doc};


        self.collection.find_one_and_update(filter, update).await
    }

    // delete user
    pub async fn delete_user(&self, id: String) -> Result<()> {
        let object_id = match ObjectId::parse_str(&id) {
            Ok(oid) => oid,
            Err(_) => {
                eprintln!("Invalid ObjectId: {}", id);
                return Ok(());
            }
        };

        let filter = doc! {"_id":&object_id};
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





