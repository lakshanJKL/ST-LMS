use serde::{ Serialize};

#[derive(Debug,Serialize)]
pub struct GenericResponse<T>{
    pub code:i32,
    pub message:String,
    pub data:T
}