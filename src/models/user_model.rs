use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub name: String,
    pub email: String,
}

//**********  dto **************

#[derive(Debug,Serialize,Deserialize)]
pub struct CreateUser{
    pub name: String,
    pub email: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateUser{
    pub name: Option<String>,
    pub email: Option<String>,
}
