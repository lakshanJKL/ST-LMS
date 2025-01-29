
use std::borrow::Cow;
use validator::ValidationError;
//pub type Result<T> = std::result::Result<T,UserServiceError>;

// pub  fn validate<T: Validate>(data: &T) -> Result<()> {
//     match data.validate() {
//         Ok(_) => Ok(()),
//         Err(e) => {
//             let errors = e
//                 .field_errors()
//                 .iter()
//                 .map(|(field, errors)| {
//                     let messages: Vec<Cow<str>> = errors
//                         .iter()
//                         .map(|err| err.message.clone().unwrap_or_else(|| Cow::Borrowed("Invalid value")))
//                         .collect();
//                     format!("{}: {}", field, messages.join(", "))
//                 })
//                 .collect::<Vec<String>>()
//                 .join("; ");
//             Err(UserServiceError::ValidationError(errors))
//         }
//     }
// }

// custom email validation
pub fn custom_email_check(value:&str)->Result<(),ValidationError>{
   if value.contains("@") {
       Ok(())
   }else {
       Err(ValidationError::new("Invalid email, email must be like example@info.com"))
   }
}

// custom password validation
pub fn custom_password_check(value:&str)->Result<(),ValidationError>{
    if value.len() <=4 {
        Ok(())
    } else {
        Err(ValidationError::new("password is invalid, password must be at least 4 characters long"))
    }
}