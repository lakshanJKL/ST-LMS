use futures_util::TryStreamExt;
use mongodb::{bson::{doc, to_bson}, error::Result, Collection, Database};
use mongodb::bson::oid::ObjectId;
use crate::models::user_model::{User, CreateUser, UpdateUser};

pub struct UserRepo{
    collection: Collection<User>
}

impl UserRepo{
    pub async fn new(db:Database)->Self{
        let collection =  db.collection("users");
        UserRepo{ collection }
    }

    // create user
    pub async fn create_new_user(&self, user:CreateUser) ->Result<User>{
        let new_user = User{
            name:user.name,
            email:user.email
        };

        self.collection.insert_one(&new_user).await.ok().expect("Error creating user");
        Ok(new_user)
    }

    //all user
    pub async fn get_all_users(&self)->Result<Vec<User>>{
        let filter = doc! {};
        let cursor = self.collection.find(filter).await?;
        cursor.try_collect().await
    }

    //update user
    pub async fn update_system_user(&self,update_user: UpdateUser,id:String)->Result<Option<User>>{
        let object_id = match ObjectId::parse_str(&id) {
            Ok(oid) => oid,
            Err(_) => {
                eprintln!("Invalid ObjectId: {}", id);
                return Ok(None); // Return None if id is invalid
            }
        };

        let filter = doc! {"_id":&object_id};

        // Convert UpdateUser to BSON
        let update_bson = to_bson(&update_user)?;

        // Safely unwrap the `Bson` value for the `$set` operation
        let update_doc = doc! {"$set": update_bson};

        self.collection.find_one_and_update(filter,update_doc).await
    }

    // delete user
    pub async fn delete_user(&self,id:String)->Result<()>{

        let object_id =  match ObjectId::parse_str(&id) {
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
}





