use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utill::validator::{custom_email_check,custom_password_check};

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct User{
    #[validate(length(min =10,message="Name must be at least 3 characters long"))]
    pub name: String,
    #[validate(email(message="Invalid email format"))]
    pub email: String,
    #[validate(length(min=8,message="Invalid role"))]
    pub role:String,
    #[validate(length(min=6,message="Password must be at least 6 characters long"))]
    pub password: String,
}

//**********  dto **************

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct CreateUser{
    #[validate(length(min =10,message="Name must be at least 3 characters long"))]
    pub name: String,
    #[validate(email(message="Invalid email format"))]
    pub email: String,
    #[validate(length(min=6,message="Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct UserLoginDto{
    #[validate(custom ="custom_email_check")] // create custom email validation
    pub username: String,
    #[validate(custom = "custom_password_check")] // create custom password validation
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct UpdateUser{
    #[validate(length(min =10,message="Name must be at least 3 characters long"))]
    pub name: Option<String>,
    #[validate(email(message="Invalid email format"))]
    pub email: Option<String>,
    #[validate(length(min=6,message="Password must be at least 6 characters long"))]
    pub password: Option<String>,
}
