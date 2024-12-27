use actix_web::{web, HttpResponse, Responder};
use mongodb::Database;
use crate::models::user_model::{CreateUser, UpdateUser};
use crate::services::user_service::{create_user_service, delete_user_service, get_all_users_service, update_user_service};

/// Create a new user (POST /users)
pub async fn create_user_controller(db:web::Data<Database>, create_user:web::Json<CreateUser>) ->impl Responder{
   let collection = db.collection("users");
    match create_user_service(&collection,create_user.into_inner()).await {
      Ok(user) => HttpResponse::Ok().json(user),
      Err(e) => HttpResponse::InternalServerError().body(e)
    }
}

/// Create a new user (GET/users)
pub async fn get_all_users_controller(db:web::Data<Database>) -> impl Responder{
  let collection =  db.collection("users");
  match get_all_users_service(&collection).await {
      Ok(users) => HttpResponse::Ok().json(users),
      Err(e) => HttpResponse::InternalServerError().body(e)
  }
}

/// Update a user by ID (PUT /users/{id})
pub async fn update_user_controller(db:web::Data<Database>,update_user:web::Json<UpdateUser>,id:web::Path<String>)->impl Responder{
   let collection =  db.collection("users");
    println!("user id : {}",&id);
   match update_user_service(&collection,update_user.into_inner(),id.to_string()).await {
       Ok(Some(user)) => HttpResponse::Ok().json(user),
       Ok(None) => HttpResponse::NotFound().body("user not found"),
       Err(e) => HttpResponse::InternalServerError().body(e)
   }
}

/// Delete a user by ID (DELETE /users/{id})
pub async fn delete_user_controller(db:web::Data<Database>,id:web::Path<String>)->impl Responder{
    let collection = db.collection("users");
    match delete_user_service(&collection,id.to_string()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e)=> HttpResponse::InternalServerError().body(e)
    }
}


