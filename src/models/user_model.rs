use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub name: String,
    pub email: String,
    pub password: String,
}

//**********  dto **************

#[derive(Debug,Serialize,Deserialize)]
pub struct CreateUser{
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserLoginDto{
    pub username: String,
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateUser{
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: String,
}
